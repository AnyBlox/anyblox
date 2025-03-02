use std::io;
use std::sync::Arc;
use std::{
    future::{ready, Future},
    ptr,
};

use bytes::Bytes;
use futures::StreamExt;
use vortex::array::{PrimitiveEncoding, SparseEncoding, StructEncoding, VarBinEncoding, VarBinViewEncoding};
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
use vortex::zigzag::ZigZagEncoding;
use vortex::{
    alp::{ALPEncoding, ALPRDEncoding},
    ArrayDType, ArrayLen,
};
use vortex::{Context, IntoCanonical};

#[derive(Clone)]
struct PtrRead {
    data: *const u8,
    len: usize,
}

// no mutating possible
unsafe impl Sync for PtrRead {}
unsafe impl Send for PtrRead {}

impl PtrRead {
    pub fn new(ptr: *const u8, len: usize) -> Self {
        Self { data: ptr, len }
    }
}

impl VortexReadAt for PtrRead {
    fn read_byte_range(&self, pos: u64, len: u64) -> impl Future<Output = io::Result<Bytes>> + 'static {
        if (pos + len) as usize > self.len {
            ready(Err(io::Error::from(io::ErrorKind::UnexpectedEof)))
        } else {
            let slice = unsafe { core::slice::from_raw_parts(self.data.add(pos as usize), len as usize) };
            ready(Ok(Bytes::from_static(slice)))
        }
    }

    fn size(&self) -> impl Future<Output = io::Result<u64>> + 'static {
        ready(Ok(self.len as u64))
    }
}

#[derive(Default)]
pub struct NativeTpchVortexImpl {
    state: Option<State>,
}

impl super::NativeImpl for NativeTpchVortexImpl {
    fn anyblox_decode(
        &mut self,
        data: &[u8],
        start_tuple: usize,
        tuple_count: usize,
        projection: crate::ColumnProjection,
    ) -> &super::NativeBatch {
        let state = self.state.get_or_insert_with(State::new);
        state.reset();

        unsafe {
            futures::executor::block_on(decode(
                data.as_ptr(),
                data.len(),
                start_tuple,
                tuple_count,
                state,
                projection,
            ))
            .unwrap();
        }

        &state.batch
    }
}

#[target_feature(enable = "sse2")]
async unsafe fn decode(
    data: *const u8,
    _data_length: usize,
    start_tuple: usize,
    tuple_count: usize,
    state: &mut State,
    projection: crate::ColumnProjection,
) -> Result<(), VortexError> {
    let tuples_per_page = unsafe { data.cast::<u32>().read() };
    let page_to_read = start_tuple as u32 / tuples_per_page;
    let (page_start, page_end) = unsafe {
        (
            data.cast::<u32>().add(2 * page_to_read as usize + 1).read(),
            data.cast::<u32>().add(2 * page_to_read as usize + 2).read(),
        )
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
            .filter(|(i, _)| projection.contains(*i))
        {
            let column = column_array.into_canonical()?;
            match column {
                vortex::Canonical::Primitive(primitive_array) => {
                    let value_size = match primitive_array.dtype() {
                        vortex::dtype::DType::Primitive(ptype, _) => match ptype {
                            vortex::dtype::PType::I8 | vortex::dtype::PType::U8 => 1,
                            vortex::dtype::PType::I16 | vortex::dtype::PType::U16 | vortex::dtype::PType::F16 => 2,
                            vortex::dtype::PType::I32 | vortex::dtype::PType::U32 | vortex::dtype::PType::F32 => 4,
                            vortex::dtype::PType::I64 | vortex::dtype::PType::U64 | vortex::dtype::PType::F64 => 8,
                        },
                        _ => unreachable!(),
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
    batch: super::NativeBatch,
    buffer_arrays: [Vec<*const u8>; 16],
    inner_batches: [super::NativeBatch; 16],
    children: [*mut super::NativeBatch; 16],
    top_level_buffer_array: [*const u8; 1],
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
            batch: super::NativeBatch::empty(),
            buffer_arrays: rep16!(vec![ptr::null(), ptr::null()]),
            inner_batches: rep16!(super::NativeBatch::empty()),
            children: [ptr::null_mut(); 16],
            top_level_buffer_array: [ptr::null()],
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
        self.batch.buffers = self.top_level_buffer_array.as_mut_ptr();
        self.batch.children = self.children.as_mut_ptr();
        self.batch.dictionary = ptr::null_mut();
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

        self.buffer_arrays[idx][0] = ptr::null_mut();
        self.owned_buffers.push(array.buffer().clone());
        self.buffer_arrays[idx][1] = unsafe { array.buffer().as_ptr().add(value_size * self.tuple_offset).cast_mut() };

        self.inner_batches[idx] = super::NativeBatch {
            length: self.batch.length,
            null_count: 0,
            offset: 0,
            n_buffers: 2,
            n_children: 0,
            buffers: self.buffer_arrays[idx].as_mut_ptr(),
            children: ptr::null_mut(),
            dictionary: ptr::null_mut(),
        };

        self.children[idx] = unsafe { self.inner_batches.as_mut_ptr().add(idx) };
        self.batch.n_children += 1;
    }

    pub fn write_string(&mut self, array: vortex::array::VarBinViewArray) {
        let idx = self.batch.n_children as usize;
        self.buffer_arrays[idx].clear();
        self.buffer_arrays[idx].reserve(2 + array.buffer_count());

        self.buffer_arrays[idx].push(ptr::null_mut());

        let views = array.views();
        let view_buffer = views.buffer().unwrap();
        self.owned_buffers.push(view_buffer.clone());
        self.buffer_arrays[idx].push(unsafe { view_buffer.as_ptr().add(16 * self.tuple_offset) });

        self.owned_buffers
            .extend(array.buffers().map(|x| x.buffer().unwrap().clone()));
        self.buffer_arrays[idx].extend(array.buffers().map(|x| x.buffer().unwrap().as_ptr()));

        let buf_lens = array.buffers().map(|x| x.len() as u64).collect::<Vec<_>>();
        let buf_len_buffer = Buffer::from(buf_lens);

        self.owned_buffers.push(buf_len_buffer.clone());
        self.buffer_arrays[idx].push(buf_len_buffer.as_ptr());

        self.inner_batches[idx] = super::NativeBatch {
            length: self.batch.length,
            null_count: 0,
            offset: 0,
            n_buffers: self.buffer_arrays[idx].len() as u32,
            n_children: 0,
            buffers: self.buffer_arrays[idx].as_mut_ptr(),
            children: ptr::null_mut(),
            dictionary: ptr::null_mut(),
        };

        self.children[idx] = unsafe { self.inner_batches.as_mut_ptr().add(idx) };
        self.batch.n_children += 1;
    }
}
