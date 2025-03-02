#![no_std]
extern crate alloc;

use alloc::vec::Vec;
use core::{cmp, fmt::Write};
use decoder_lib::{arrow::ArrowArray, ffi_utils::WasmPtr, log};

const FSST_ESCAPE: u8 = 0xFF;
const STRING_COLUMNS: usize = 5;

struct State {
    dataset_info: DatasetInfo,
    rid_col: Vec<i64>,
    string_cols: [DecodedColumn; STRING_COLUMNS],
}

struct DatasetInfo {
    row_count: usize,
    first_rid: usize,
    string_cols: [ColInfo; STRING_COLUMNS],
}

struct ColInfo {
    page_offsets: *const u32,
    num_pages: usize,
    tuples_per_page: usize,
    tuples_on_last_page: usize,
}

struct FsstColumn {
    table: SymbolTable,
    offsets: *const u32,
    data: *const u8,
}

struct SymbolTable {
    symbols: *const u64,
    lens: *const u8,
}

struct DecodedColumn {
    data: Vec<u8>,
    data_offset: usize,
    validity: Vec<u8>,
    offsets: Vec<u32>,
    null_count: u32,
    validity_byte: u8,
    validity_byte_idx: usize,
}

impl DecodedColumn {
    fn new() -> Self {
        DecodedColumn {
            data: Vec::new(),
            data_offset: 0,
            validity: Vec::new(),
            offsets: Vec::new(),
            null_count: 0,
            validity_byte: 0,
            validity_byte_idx: 0,
        }
    }

    fn prepare(&mut self, tuple_count: usize, total_compressed_len: usize) {
        self.data.clear();
        self.data.reserve(2 * total_compressed_len);
        self.data_offset = 0;
        self.validity.clear();
        self.validity.reserve((tuple_count + 7) / 8);
        self.offsets.clear();
        self.offsets.reserve(tuple_count);
        self.null_count = 0;
        self.validity_byte = 0;
        self.validity_byte_idx = 0;
        self.offsets.push(0);
    }

    fn ptr_for_next_value(&mut self, max_len: usize) -> *mut u8 {
        let start = self.data_offset;
        let end = self.data_offset + max_len;
        if end >= self.data.len() {
            self.data.resize(end + 8, 0);
        }
        unsafe { self.data.as_mut_ptr().add(start) }
    }

    fn commit_value(&mut self, len: usize) {
        self.validity_byte |= 1 << self.validity_byte_idx;
        self.validity_byte_idx += 1;

        self.try_push_validity();

        self.data_offset += len;
        self.offsets.push(self.data_offset as u32);
    }

    fn push_null(&mut self) {
        self.validity_byte_idx += 1;
        self.null_count += 1;

        self.try_push_validity();
        self.offsets.push(self.data_offset as u32);
    }

    fn try_push_validity(&mut self) {
        if self.validity_byte_idx == 8 {
            self.force_push_validity();
        }
    }

    fn force_push_validity(&mut self) {
        self.validity.push(self.validity_byte);
        self.validity_byte = 0;
        self.validity_byte_idx = 0;
    }

    fn finish(&mut self) {
        if self.validity_byte_idx != 0 {
            self.force_push_validity();
        }
    }

    fn null_count(&self) -> u32 {
        self.null_count
    }

    fn data_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    fn validity_ptr(&self) -> *const u8 {
        self.validity.as_ptr()
    }

    fn offsets_ptr(&self) -> *const u8 {
        self.offsets.as_ptr().cast()
    }
}

#[no_mangle]
#[target_feature(enable = "simd128")]
pub unsafe extern "C" fn anyblox_decode(
    data: *const u8,
    _data_length: usize,
    start_tuple: usize,
    tuple_count: usize,
    state: *mut u8,
    projection_mask_1: u32,
    projection_mask_2: u32,
) -> *const u8 {
    let state_check = state.read();
    if state_check == 0 {
        let dataset_info = read_metadata(data);
        let state_obj = State {
            dataset_info,
            rid_col: Vec::new(),
            string_cols: [
                DecodedColumn::new(),
                DecodedColumn::new(),
                DecodedColumn::new(),
                DecodedColumn::new(),
                DecodedColumn::new(),
            ],
        };
        state.write(1);
        state
            .add(align_of::<State>())
            .cast::<State>()
            .write(state_obj);
    }
    let state: &mut State = &mut *state.add(align_of::<State>()).cast::<State>();
    let projection =
        decoder_lib::column_projection::ColumnProjection::new(projection_mask_1, projection_mask_2);

    // We decode the columns independently to not jump around in memory for possibly a lot.
    let last_available_tuple = state.dataset_info.row_count;
    let tuple_end = cmp::min(start_tuple + tuple_count, last_available_tuple);
    let tuple_count = tuple_end - start_tuple;
    let start_tuple = start_tuple - state.dataset_info.first_rid;
    let mut is_decoded = [false; 5];
    let mut decoded_string_cols = [None; STRING_COLUMNS];
    let mut actual_tuple_count = tuple_count;
    for id in 0..STRING_COLUMNS {
        if projection.contains(id as u32 + 1) {
            actual_tuple_count = decode_column(
                data,
                &mut state.string_cols[id as usize],
                &state.dataset_info.string_cols[id as usize],
                start_tuple,
                tuple_count,
            );
            is_decoded[id as usize] = true;
        }
    }
    for id in 0..STRING_COLUMNS {
        decoded_string_cols[id as usize] =
            is_decoded[id as usize].then_some(&state.string_cols[id as usize]);
    }
    let decoded_rid: Option<&[i64]> = if projection.contains(0) {
        state.rid_col.clear();
        state.rid_col.reserve_exact(tuple_count);

        for rid in start_tuple..(start_tuple + actual_tuple_count) {
            state.rid_col.push(rid  as i64 + state.dataset_info.first_rid as i64);
        }
        Some(&state.rid_col)
    } else {
        None
    };

    write(
        actual_tuple_count,
        decoded_rid,
        decoded_string_cols,
        &raw mut BATCH,
    );
    let ptr: *const ArrowArray = &raw const BATCH;
    ptr.cast::<u8>()
}

static mut BATCH: ArrowArray = ArrowArray::empty();

unsafe fn decode_column(
    data: *const u8,
    decoded_column: &mut DecodedColumn,
    encoded_column: &ColInfo,
    start_tuple: usize,
    tuple_count: usize,
) -> usize {
    let page_to_read = start_tuple / encoded_column.tuples_per_page;
    let first_tuple_on_page = start_tuple % encoded_column.tuples_per_page;
    let tuple_count_on_page = if page_to_read == encoded_column.num_pages - 1 {
        encoded_column.tuples_on_last_page
    } else {
        encoded_column.tuples_per_page
    };
    let page_offset = encoded_column.page_offsets.add(page_to_read).read();

    let page = read_page_metadata(data, page_offset, tuple_count_on_page);

    log!(
        "decode column data, page num {}, offsets: {:?} {:?}",
        page_to_read,
        page.data,
        page.offsets
    );
    let tuple_count = cmp::min(tuple_count, tuple_count_on_page - first_tuple_on_page);
    let offset_of_first = page.offsets.add(first_tuple_on_page).read() as usize;
    let offset_of_after_last = page.offsets.add(first_tuple_on_page + tuple_count).read() as usize;
    log!("offsets: {} to {}", offset_of_first, offset_of_after_last);
    decoded_column.prepare(tuple_count, offset_of_after_last - offset_of_first);

    for tuple_idx in first_tuple_on_page..(first_tuple_on_page + tuple_count) {
        log!("tuple_idx: {}", tuple_idx);
        let start_offset = page.offsets.add(tuple_idx).read() as usize;
        let end_offset = page.offsets.add(tuple_idx + 1).read() as usize;
        let compressed_len = end_offset - start_offset;
        log!("offsets: {} to {}", start_offset, end_offset);

        if compressed_len == 0 {
            decoded_column.push_null();
        } else {
            log!("requesting ptr_for_next_value: {}", compressed_len * 8);
            let ptr = decoded_column.ptr_for_next_value(compressed_len * 8);
            let mut read_i = 0;
            let mut write_i = 0;
            while read_i < compressed_len {
                let b = page.data.add(start_offset + read_i).read();
                read_i += 1;

                if b == FSST_ESCAPE {
                    let b = page.data.add(start_offset + read_i).read();
                    decoder_lib::log!("WRITE 8: {:?}", ptr.add(write_i));
                    ptr.add(write_i).write(b);
                    read_i += 1;
                    write_i += 1;
                } else {
                    let len = page.table.lens.add(b as usize).read();
                    let symbol = page.table.symbols.add(b as usize).read();
                    decoder_lib::log!("WRITE 64: {:?}", ptr.add(write_i));
                    ptr.add(write_i).cast::<u64>().write_unaligned(symbol);
                    write_i += len as usize;
                }
            }
            decoded_column.commit_value(write_i);
        }
    }

    decoded_column.finish();
    tuple_count
}

fn read_metadata(data: *const u8) -> DatasetInfo {
    // The header is eight 32bit integers: tuples per page, first rid, col1 start, col2 start, ..., col5 start.
    let data_header = data.cast::<u32>();
    let tuples_per_page = unsafe { data_header.read() } as usize;
    let first_rid = unsafe { data_header.add(1).read() } as usize;
    let mut offsets = [0; STRING_COLUMNS];
    for i in 0..STRING_COLUMNS {
        offsets[i] = unsafe { data_header.add(2 + i).read() };
    }
    let is_last_part = first_rid == 75202560;
    let row_count = if is_last_part { 24794937 } else { 25067520 };
    let num_pages = if is_last_part { 202 } else { 204 };
    let tuples_on_last_page = if is_last_part { 96057 } else { 122880 };

    log!("column offsets: {:?}", offsets);

    DatasetInfo {
        first_rid,
        row_count,
        string_cols: [
            read_column_metadata(
                data,
                offsets[0],
                num_pages,
                tuples_per_page,
                tuples_on_last_page,
            ),
            read_column_metadata(
                data,
                offsets[1],
                num_pages,
                tuples_per_page,
                tuples_on_last_page,
            ),
            read_column_metadata(
                data,
                offsets[2],
                num_pages,
                tuples_per_page,
                tuples_on_last_page,
            ),
            read_column_metadata(
                data,
                offsets[3],
                num_pages,
                tuples_per_page,
                tuples_on_last_page,
            ),
            read_column_metadata(
                data,
                offsets[4],
                num_pages,
                tuples_per_page,
                tuples_on_last_page,
            ),
        ],
    }
}

fn read_column_metadata(
    data: *const u8,
    start: u32,
    num_pages: usize,
    tuples_per_page: usize,
    tuples_on_last_page: usize,
) -> ColInfo {
    let ptr = unsafe { data.add(start as usize).cast() };

    ColInfo {
        page_offsets: ptr,
        num_pages,
        tuples_per_page,
        tuples_on_last_page,
    }
}

fn read_page_metadata(data: *const u8, start: u32, row_count: usize) -> FsstColumn {
    // First is the symbol table.
    let ptr = unsafe { data.add(start as usize) };
    let (symbol_table, ptr_after_symbols) = read_symbol_table_data(ptr);
    // Then we have (row_count + 1) offsets.
    let offsets = ptr_after_symbols.cast::<u32>();
    // And finally the actual data.
    let string_data = unsafe { offsets.add(row_count + 1).cast::<u8>() };

    log!(
        "page descr (symboltab, offsets, strings): {:?} {:?} {:?}",
        ptr,
        ptr_after_symbols,
        string_data
    );

    FsstColumn {
        table: symbol_table,
        offsets,
        data: string_data,
    }
}

fn read_symbol_table_data(data: *const u8) -> (SymbolTable, *const u8) {
    // The encoding has 1 + N bytes + padding + 8*N, where N is the number of symbols in the table,
    // and padding is the amount required to have symbols aligned to the 8-byte boundary.
    // |N: u8|len0:u8|len1:u8|...|lenN:u8|opt_padding|sym0: u64|sym1: u64|...|symN:u64|
    let len_byte = unsafe { data.read() };
    let len = len_byte as usize;
    let lens = unsafe { data.add(1) };
    let rem = (len + 1) % 8;
    let symbols_offset = if rem == 0 {
        len + 1
    } else {
        len + 1 + (8 - rem)
    };
    let symbols = unsafe { data.add(symbols_offset).cast::<u64>() };
    let end = unsafe { symbols.add(len).cast::<u8>() };

    let table = SymbolTable { symbols, lens };
    (table, end)
}

unsafe fn write(
    row_count: usize,
    rid: Option<&[i64]>,
    string_columns: [Option<&DecodedColumn>; STRING_COLUMNS],
    batch: *mut ArrowArray,
) {
    static mut BUFFER_ARRAYS: [[WasmPtr; 3]; 6] = [[WasmPtr::NULL; 3]; 6];
    static mut INNER_BATCHES: [ArrowArray; 6] = [
        ArrowArray::empty(),
        ArrowArray::empty(),
        ArrowArray::empty(),
        ArrowArray::empty(),
        ArrowArray::empty(),
        ArrowArray::empty(),
    ];
    static mut CHILDREN_ARRAY: [WasmPtr; 6] = [WasmPtr::NULL; 6];
    static mut TOP_LEVEL_BUFFER_ARRAY: [WasmPtr; 1] = [WasmPtr::NULL];

    (*batch).length = row_count as u32;
    (*batch).null_count = 0;
    (*batch).n_buffers = 1;
    (*batch).buffers = WasmPtr::from((&raw mut TOP_LEVEL_BUFFER_ARRAY) as u32);
    (*batch).children = WasmPtr::from((&raw mut CHILDREN_ARRAY) as u32);
    (*batch).dictionary = WasmPtr::NULL;

    let mut idx = 0;
    if let Some(rid) = rid {
        write_rid_column(idx, rid, row_count);
        idx += 1;
    }
    for col in string_columns.iter().filter_map(|x| *x) {
        write_string_column(idx, col, row_count);
        idx += 1;
    }
    (*batch).n_children = idx as u32;

    unsafe fn write_rid_column(idx: usize, column: &[i64], len: usize) {
        CHILDREN_ARRAY[idx] = WasmPtr::from(&raw const INNER_BATCHES[idx] as u32);

        BUFFER_ARRAYS[idx][0] = WasmPtr::NULL;
        BUFFER_ARRAYS[idx][1] = column.as_ptr().into();

        INNER_BATCHES[idx].length = len as u32;
        INNER_BATCHES[idx].null_count = 0;
        INNER_BATCHES[idx].n_buffers = 2;
        INNER_BATCHES[idx].n_children = 0;
        INNER_BATCHES[idx].buffers = WasmPtr::from((&raw mut BUFFER_ARRAYS[idx]) as u32);
        INNER_BATCHES[idx].children = WasmPtr::NULL;
        INNER_BATCHES[idx].dictionary = WasmPtr::NULL;
    }

    unsafe fn write_string_column(idx: usize, column: &DecodedColumn, len: usize) {
        CHILDREN_ARRAY[idx] = WasmPtr::from(&raw const INNER_BATCHES[idx] as u32);

        BUFFER_ARRAYS[idx][0] = column.validity_ptr().into();
        BUFFER_ARRAYS[idx][1] = column.offsets_ptr().into();
        BUFFER_ARRAYS[idx][2] = column.data_ptr().into();

        INNER_BATCHES[idx].length = len as u32;
        INNER_BATCHES[idx].null_count = column.null_count();
        INNER_BATCHES[idx].n_buffers = 3;
        INNER_BATCHES[idx].n_children = 0;
        INNER_BATCHES[idx].buffers = WasmPtr::from((&raw mut BUFFER_ARRAYS[idx]) as u32);
        INNER_BATCHES[idx].children = WasmPtr::NULL;
        INNER_BATCHES[idx].dictionary = WasmPtr::NULL;
    }
}
