use std::future::{ready, Future, Ready};
use std::io;
use std::pin::pin;
use std::sync::Arc;
use std::fmt::Write;

use arrow::ArrowArray;
use bytes::Bytes;
use column_projection::ColumnProjection;
use decoder_lib::*;
use ffi_utils::WasmPtr;
use futures::StreamExt;
use vortex::{alp::{ALPEncoding, ALPRDEncoding}, ArrayLen, ArrayDType};
use vortex::array::{
    PrimitiveEncoding, SparseEncoding, StructEncoding, VarBinEncoding, VarBinViewEncoding, 
};
use vortex::buffer::Buffer;
use vortex::bytebool::ByteBoolEncoding;
use vortex::datetime_parts::DateTimePartsEncoding;
use vortex::dict::DictEncoding;
use vortex::encoding::EncodingRef;
use vortex::error::VortexError;
use vortex::fastlanes::{BitPackedEncoding, DeltaEncoding, FoREncoding};
use vortex::file::{LayoutContext, LayoutDeserializer};
use vortex::fsst::FSSTEncoding;
use vortex::io::VortexReadAt;
use vortex::runend::RunEndEncoding;
use vortex::runend_bool::RunEndBoolEncoding;
use vortex::validity::Validity;
use vortex::zigzag::ZigZagEncoding;
use vortex::{Context, IntoCanonical};

#[derive(Clone)]
struct PtrRead {
    data: *const u8,
    len: usize,
}

// wasm has only one thread so this doesn't matter
unsafe impl Sync for PtrRead {}
unsafe impl Send for PtrRead {}

impl PtrRead {
    pub fn new(ptr: *const u8, len: usize) -> Self {
        Self {
            data: ptr,
            len,
        }
    }
}

impl VortexReadAt for PtrRead {
    fn read_byte_range(
        &self,
        pos: u64,
        len: u64,
    ) -> impl Future<Output = io::Result<Bytes>> + 'static {
        log!("Read of {len} bytes from {pos}");
        if (pos + len) as usize > self.len {
            ready(Err(io::Error::from(io::ErrorKind::UnexpectedEof)))
        } else {
            let slice =
                unsafe { core::slice::from_raw_parts(self.data.add(pos as usize), len as usize) };
            ready(Ok(Bytes::from_static(slice)))
        }
    }

    fn size(&self) -> impl Future<Output = io::Result<u64>> + 'static {
        ready(Ok(self.len as u64))
    }
}
 
#[no_mangle]
#[target_feature(enable = "simd128")]
pub unsafe extern "C" fn anyblox_decode(
    data: *const u8,
    data_length: usize,
    start_tuple: usize,
    tuple_count: usize,
    state: *mut u8,
    projection_mask_1: u32,
    projection_mask_2: u32,
) -> *const u8 {
    std::panic::set_hook(Box::new(decoder_lib::panic_handler));
    let state_check = state.read();
    if state_check == 0 {
        let state_obj = State::new();
        state.write(1);
        state
            .add(align_of::<State>())
            .cast::<State>()
            .write(state_obj);
    }
    let state: &mut State = &mut *state.add(align_of::<State>()).cast::<State>();
    state.reset();
    let projection = ColumnProjection::new(projection_mask_1, projection_mask_2);

    match futures::executor::block_on(decode(
        data,
        data_length,
        start_tuple,
        tuple_count,
        state,
        projection,
    )) {
        Ok(_) => {},
        Err(err) => {
            log!("Error: {}", err);
            panic!("{}", err)
        }
    };

    let ptr: *const ArrowArray = &raw const state.batch;
    ptr.cast::<u8>()
}

async fn decode(
    data: *const u8,
    data_length: usize,
    start_tuple: usize,
    tuple_count: usize,
    state: &mut State,
    projection: ColumnProjection,
) -> Result<(), VortexError> {
    let tuples_per_page = unsafe {
        data.cast::<u32>().read()
    };
    let page_to_read = start_tuple as u32 / tuples_per_page;
    let (page_start, page_end) = unsafe {
        (data.cast::<u32>().add(2 * page_to_read as usize + 1).read(), data.cast::<u32>().add(2 * page_to_read as usize + 2).read())
    };

    let page_len = page_end - page_start;

    let read = PtrRead::new(unsafe { data.add(page_start as usize) }, page_len as usize);
    let mut reader = vortex::file::VortexReadBuilder::new(
        read,
        LayoutDeserializer::new(state.compressor_context.clone(), LayoutContext::default().into()),
    )
    .build()
    .await?;

    let mut tuple_idx = page_to_read * tuples_per_page;
    while let Some(array) = reader.next().await {
        let array = array?;

        if tuple_idx as usize + array.len() < start_tuple {
            tuple_idx += array.len() as u32;
            continue;
        }

        let canonical = array.into_canonical()?.into_struct()?;
        let tuple_offset = start_tuple - tuple_idx as usize;
        state.set_tuple_offset(tuple_offset);
        state.set_len(std::cmp::min(tuple_count, canonical.len() - tuple_offset) as u32);

        for (_, column_array) in canonical
            .children()
            .enumerate()
            .filter(|(i, _)| projection.contains(*i as u32))
        {
            let column = column_array.into_canonical()?;
            match column {
                vortex::Canonical::Primitive(primitive_array) => {
                    let value_size = match primitive_array.dtype() {
                        vortex::dtype::DType::Primitive(ptype, _) => match ptype {
                            vortex::dtype::PType::I8 |
                            vortex::dtype::PType::U8 => 1,
                            vortex::dtype::PType::I16 |
                            vortex::dtype::PType::U16 |
                            vortex::dtype::PType::F16 => 2,
                            vortex::dtype::PType::I32 |
                            vortex::dtype::PType::U32 |
                            vortex::dtype::PType::F32 => 4,
                            vortex::dtype::PType::I64 |
                            vortex::dtype::PType::U64 |
                            vortex::dtype::PType::F64 => 8,
                        },
                        _ => unreachable!()
                    };
                    state.write_primitive(primitive_array, value_size);
                }
                vortex::Canonical::VarBinView(var_bin_view_array) => {
                    state.write_string(var_bin_view_array);
                }
                _ => unreachable!(),
            }
        }

        break;
    }

    Ok(())
}

struct State {
    batch: ArrowArray,
    buffer_arrays: [Vec<WasmPtr>; 16],
    inner_batches: [ArrowArray; 16],
    children: [WasmPtr; 16],
    top_level_buffer_array: [WasmPtr; 1],
    owned_buffers: Vec<Buffer>,
    compressor_context: Arc<Context>,
    tuple_offset: usize,
}

macro_rules! rep16 {
    ($e:expr) => {
        [$e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e, $e]
    };
}

impl State {
    pub fn new() -> Self {
        Self {
            batch: ArrowArray::empty(),
            buffer_arrays: rep16!(vec![WasmPtr::NULL, WasmPtr::NULL]),
            inner_batches: rep16!(ArrowArray::empty()),
            children: [WasmPtr::NULL; 16],
            top_level_buffer_array: [WasmPtr::NULL],
            owned_buffers: Vec::with_capacity(3 * 16),
            compressor_context: Arc::new(Context::default().with_encodings([
                &ALPEncoding as EncodingRef,
                &ALPRDEncoding,
                &ByteBoolEncoding,
                &DateTimePartsEncoding,
                &DictEncoding,
                &BitPackedEncoding,
                &DeltaEncoding,
                &FoREncoding,
                &FSSTEncoding,
                &PrimitiveEncoding,
                &RunEndEncoding,
                &RunEndBoolEncoding,
                &SparseEncoding,
                &StructEncoding,
                &VarBinEncoding,
                &VarBinViewEncoding,
                &ZigZagEncoding,
            ])),
            tuple_offset: 0,
        }
    }

    pub fn reset(&mut self) {
        self.batch.buffers = WasmPtr::from(self.top_level_buffer_array.as_ptr());
        self.batch.children = WasmPtr::from(self.children.as_ptr());
        self.batch.dictionary = WasmPtr::NULL;
        self.batch.length = 0;
        self.batch.n_buffers = 1;
        self.batch.n_children = 0;
        self.batch.null_count = 0;
        self.batch.offset = 0;

        self.owned_buffers.clear();
    }

    pub fn set_tuple_offset(&mut self, offset: usize) {
        self.tuple_offset = offset;
    }
    
    pub fn set_len(&mut self, len: u32) {
        self.batch.length = len;
    }

    pub fn write_primitive(&mut self, array: vortex::array::PrimitiveArray, value_size: usize) {
        let idx = self.batch.n_children as usize;

        self.buffer_arrays[idx][0] = WasmPtr::NULL;
        self.owned_buffers.push(array.buffer().clone());
        self.buffer_arrays[idx][1] = WasmPtr::from(unsafe { array.buffer().as_ptr().add(value_size * self.tuple_offset) });

        self.inner_batches[idx] = ArrowArray {
            length: self.batch.length,
            null_count: 0,
            offset: 0,
            n_buffers: 2,
            n_children: 0,
            buffers: WasmPtr::from(self.buffer_arrays[idx].as_ptr()),
            children: WasmPtr::NULL,
            dictionary: WasmPtr::NULL,
        };

        self.children[idx] = WasmPtr::from(unsafe { self.inner_batches.as_mut_ptr().add(idx) });
        self.batch.n_children += 1;
    }

    pub fn write_string(&mut self, array: vortex::array::VarBinViewArray) {
        let idx = self.batch.n_children as usize;

        self.buffer_arrays[idx].clear();
        self.buffer_arrays[idx].reserve(2 + array.buffer_count());
        self.buffer_arrays[idx].push(WasmPtr::NULL);

        let views = array.views();
        let view_buffer = views.buffer().unwrap();
        let ptr = view_buffer.as_ptr();
        log!("view_buffer ptr: {:x}", ptr as u32);
        self.owned_buffers.push(view_buffer.clone());
        self.buffer_arrays[idx].push(WasmPtr::from(unsafe  { view_buffer.as_ptr().add(16 * self.tuple_offset) }));

        self.owned_buffers.extend(array.buffers().map(|x| x.buffer().unwrap().clone()));
        self.buffer_arrays[idx].extend(array.buffers().map(|x| WasmPtr::from(x.buffer().unwrap().as_ptr())));

        let buf_lens = array.buffers().map(|x| x.len() as u64).collect::<Vec<_>>();
        let buf_len_buffer = Buffer::from(buf_lens);
        
        self.owned_buffers.push(buf_len_buffer.clone());
        self.buffer_arrays[idx].push(WasmPtr::from(buf_len_buffer.as_ptr()));

        self.inner_batches[idx] = ArrowArray {
            length: self.batch.length,
            null_count: 0,
            offset: 0,
            n_buffers: self.buffer_arrays[idx].len() as u32,
            n_children: 0,
            buffers: WasmPtr::from(self.buffer_arrays[idx].as_mut_ptr()),
            children: WasmPtr::NULL,
            dictionary: WasmPtr::NULL,
        };

        self.children[idx] = WasmPtr::from(unsafe { self.inner_batches.as_mut_ptr().add(idx) });
        self.batch.n_children += 1;
    }
}
