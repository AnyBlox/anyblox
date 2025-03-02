use std::ptr;

use crate::ColumnProjection;

use super::{NativeBatch, NativeImpl};

#[derive(Default)]
pub struct NativeTaxpayerFsstImpl {
    state: Option<State>,
}

struct State {
    dataset_info: DatasetInfo,
    cols: (DecodedColumn, DecodedColumn, DecodedColumn),
    batch_capsule: BatchCapsule,
}

struct BatchCapsule {
    batch: super::NativeBatch,
    buffer_arrays: [[*mut u8; 3]; 3],
    inner_batches: [NativeBatch; 3],
    children_array: [*mut NativeBatch; 3],
    top_level_buffer_array: [*mut NativeBatch; 1],
}

const FSST_ESCAPE: u8 = 0xFF;

struct DatasetInfo {
    first_name_column: FsstColumn,
    last_name_column: FsstColumn,
    state_column: FsstColumn,
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
            data: vec![],
            data_offset: 0,
            validity: vec![],
            offsets: vec![],
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

impl NativeImpl for NativeTaxpayerFsstImpl {
    fn anyblox_decode(
        &mut self,
        data: &[u8],
        start_tuple: usize,
        tuple_count: usize,
        projection: ColumnProjection,
    ) -> &super::NativeBatch {
        let state = self.state.get_or_insert_with(|| {
            let dataset_info = read_metadata(data.as_ptr());
            State {
                dataset_info,
                cols: (DecodedColumn::new(), DecodedColumn::new(), DecodedColumn::new()),
                batch_capsule: BatchCapsule::new(),
            }
        });

        let first_name_col = if projection.contains(0) {
            unsafe {
                decode_column(
                    &mut state.cols.0,
                    &state.dataset_info.first_name_column,
                    start_tuple,
                    tuple_count,
                )
            };
            Some(&state.cols.0)
        } else {
            None
        };
        let last_name_col = if projection.contains(1) {
            unsafe {
                decode_column(
                    &mut state.cols.1,
                    &state.dataset_info.last_name_column,
                    start_tuple,
                    tuple_count,
                )
            };
            Some(&state.cols.1)
        } else {
            None
        };
        let state_col = if projection.contains(2) {
            unsafe {
                decode_column(
                    &mut state.cols.2,
                    &state.dataset_info.state_column,
                    start_tuple,
                    tuple_count,
                )
            };
            Some(&state.cols.2)
        } else {
            None
        };

        unsafe {
            state
                .batch_capsule
                .write(tuple_count, first_name_col, last_name_col, state_col);
        }

        &state.batch_capsule.batch
    }
}

unsafe fn decode_column(column: &mut DecodedColumn, compressed: &FsstColumn, start_tuple: usize, tuple_count: usize) {
    let offset_of_first = compressed.offsets.add(start_tuple).read() as usize;
    let offset_of_after_last = compressed.offsets.add(start_tuple + tuple_count).read() as usize;
    column.prepare(tuple_count, offset_of_after_last - offset_of_first);

    for tuple_idx in start_tuple..(start_tuple + tuple_count) {
        let start_offset = compressed.offsets.add(tuple_idx).read() as usize;
        let end_offset = compressed.offsets.add(tuple_idx + 1).read() as usize;
        let compressed_len = end_offset - start_offset;

        if compressed_len == 0 {
            column.push_null();
        } else {
            let ptr = column.ptr_for_next_value(compressed_len * 8);
            let mut read_i = 0;
            let mut write_i = 0;
            while read_i < compressed_len {
                let b = compressed.data.add(start_offset + read_i).read();
                read_i += 1;

                if b == FSST_ESCAPE {
                    let b = compressed.data.add(start_offset + read_i).read();
                    ptr.add(write_i).write(b);
                    read_i += 1;
                    write_i += 1;
                } else {
                    let len = compressed.table.lens.add(b as usize).read();
                    let symbol = compressed.table.symbols.add(b as usize).read();
                    ptr.add(write_i).cast::<u64>().write_unaligned(symbol);
                    write_i += len as usize;
                }
            }
            column.commit_value(write_i);
        }
    }

    column.finish();
}

fn read_metadata(data: *const u8) -> DatasetInfo {
    // The header is four 32bit integers: row_count, col1 end, col2 end, col3 end.
    let data_header = data.cast::<u32>();
    let row_count: usize = unsafe { data_header.read() } as usize;
    let first_name_start_offset = unsafe { data_header.add(1).read() };
    let last_name_start_offset = unsafe { data_header.add(2).read() };
    let state_start_offset = unsafe { data_header.add(3).read() };

    DatasetInfo {
        first_name_column: read_column_metadata(data, first_name_start_offset, row_count),
        last_name_column: read_column_metadata(data, last_name_start_offset, row_count),
        state_column: read_column_metadata(data, state_start_offset, row_count),
    }
}

fn read_column_metadata(data: *const u8, start: u32, row_count: usize) -> FsstColumn {
    // First is the symbol table.
    let ptr = unsafe { data.add(start as usize) };
    let (symbol_table, ptr_after_symbols) = read_symbol_table_data(ptr);
    // Then we have (row_count + 1) offsets.
    let offsets = ptr_after_symbols.cast::<u32>();
    // And finally the actual data.
    let string_data = unsafe { offsets.add(row_count + 1).cast::<u8>() };

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
    let symbols_offset = if rem == 0 { len + 1 } else { len + 1 + (8 - rem) };
    let symbols = unsafe { data.add(symbols_offset).cast::<u64>() };
    let end = unsafe { symbols.add(len).cast::<u8>() };

    let table = SymbolTable { symbols, lens };
    (table, end)
}

impl BatchCapsule {
    pub fn new() -> Self {
        Self {
            batch: NativeBatch::empty(),
            buffer_arrays: [[std::ptr::null_mut(); 3]; 3],
            inner_batches: [NativeBatch::empty(), NativeBatch::empty(), NativeBatch::empty()],
            children_array: [std::ptr::null_mut(); 3],
            top_level_buffer_array: [std::ptr::null_mut(); 1],
        }
    }

    pub unsafe fn write(
        &mut self,
        row_count: usize,
        first_name: Option<&DecodedColumn>,
        last_name: Option<&DecodedColumn>,
        state: Option<&DecodedColumn>,
    ) {
        self.batch.length = row_count as u32;
        self.batch.null_count = 0;
        self.batch.n_buffers = 1;
        self.batch.buffers = self.top_level_buffer_array.as_mut_ptr().cast();
        self.batch.children = self.children_array.as_mut_ptr().cast();
        self.batch.dictionary = ptr::null_mut();

        let mut idx = 0;
        if let Some(x) = first_name {
            write_column(self, idx, x, row_count);
            idx += 1;
        }
        if let Some(x) = last_name {
            write_column(self, idx, x, row_count);
            idx += 1;
        }
        if let Some(x) = state {
            write_column(self, idx, x, row_count);
            idx += 1;
        }
        self.batch.n_children = idx as u32;

        unsafe fn write_column(capsule: &mut BatchCapsule, idx: usize, column: &DecodedColumn, len: usize) {
            capsule.children_array[idx] = capsule.inner_batches.as_mut_ptr().add(idx);

            capsule.buffer_arrays[idx][0] = column.validity_ptr().cast_mut();
            capsule.buffer_arrays[idx][1] = column.offsets_ptr().cast_mut();
            capsule.buffer_arrays[idx][2] = column.data_ptr().cast_mut();

            capsule.inner_batches[idx].length = len as u32;
            capsule.inner_batches[idx].null_count = column.null_count();
            capsule.inner_batches[idx].n_buffers = 3;
            capsule.inner_batches[idx].n_children = 0;
            capsule.inner_batches[idx].buffers = capsule.buffer_arrays.as_mut_ptr().add(idx).cast();
            capsule.inner_batches[idx].children = ptr::null_mut();
            capsule.inner_batches[idx].dictionary = ptr::null_mut();
        }
    }
}
