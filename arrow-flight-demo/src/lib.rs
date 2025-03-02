use arrow::array::{Array, AsArray, RecordBatch};
use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

pub fn init_logging() {
    let env_filter = tracing_subscriber::filter::EnvFilter::builder()
        .with_default_directive(tracing::level_filters::LevelFilter::from_level(tracing::Level::DEBUG).into())
        .from_env_lossy();
    tracing_subscriber::fmt().with_env_filter(env_filter).init();
}

pub trait PrimitiveColumnSink: Default {
    type Result;

    fn into_result(self) -> Self::Result;

    fn consume_null(&mut self);

    fn consume_value(&mut self, slice: &[u8]);
}

pub trait Sink {
    type Error: std::error::Error + Send + Sync + 'static;
    type Result: Display;

    fn consume_batch(&mut self, batch: RecordBatch) -> Result<(), Self::Error>;

    fn into_result(self) -> Self::Result;
}

pub struct ChecksumSink {
    len: usize,
    null_count: usize,
    values_hasher: simd_adler32::Adler32,
    validity_hasher: simd_adler32::Adler32,
}

#[derive(Debug)]
pub struct VecSink {
    values: Vec<Vec<u8>>,
    validity: Vec<bool>,
}

#[derive(Debug)]
pub struct EmptySink {
    _schema: Arc<arrow::datatypes::Schema>,
}

pub struct EmptyResult;

impl EmptySink {
    pub fn new(schema: Arc<arrow::datatypes::Schema>) -> Self {
        Self { _schema: schema }
    }
}

impl Sink for EmptySink {
    type Error = arrow::error::ArrowError;

    type Result = EmptyResult;

    fn consume_batch(&mut self, _batch: RecordBatch) -> Result<(), Self::Error> {
        Ok(())
    }

    fn into_result(self) -> Self::Result {
        EmptyResult
    }
}

impl Display for EmptyResult {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

pub struct TableSink<S> {
    column_sinks: Vec<S>,
    schema: Arc<arrow::datatypes::Schema>,
    record_batch_count: usize,
    tuple_count: usize,
    bytes: usize,
}

pub struct Checksum {
    len: usize,
    null_count: usize,
    hash: u64,
}

pub struct TableResult<R> {
    column_results: Vec<R>,
    schema: Arc<arrow::datatypes::Schema>,
    record_batch_count: usize,
    tuple_count: usize,
    bytes: usize,
}

impl ChecksumSink {
    pub fn new() -> Self {
        Self {
            len: 0,
            null_count: 0,
            values_hasher: simd_adler32::Adler32::new(),
            validity_hasher: simd_adler32::Adler32::new(),
        }
    }
}

impl Default for ChecksumSink {
    fn default() -> Self {
        Self::new()
    }
}

impl Checksum {
    pub fn num_rows(&self) -> usize {
        self.len
    }

    pub fn null_count(&self) -> usize {
        self.null_count
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }
}

impl VecSink {
    pub fn new() -> Self {
        Self {
            values: vec![],
            validity: vec![],
        }
    }
}

impl Default for VecSink {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: PrimitiveColumnSink> TableSink<S> {
    pub fn new(schema: Arc<arrow::datatypes::Schema>) -> Self {
        let column_sinks = std::iter::repeat_with(|| S::default())
            .take(schema.fields().len())
            .collect::<Vec<_>>();
        Self {
            column_sinks,
            schema,
            tuple_count: 0,
            record_batch_count: 0,
            bytes: 0,
        }
    }
}

impl PrimitiveColumnSink for ChecksumSink {
    type Result = Checksum;
    fn into_result(self) -> Self::Result {
        let value_hash = self.values_hasher.finish();
        let validity_hash = self.validity_hasher.finish();
        let hash = (validity_hash as u64) | ((value_hash as u64) << 32);
        Checksum {
            len: self.len,
            null_count: self.null_count,
            hash,
        }
    }

    fn consume_value(&mut self, slice: &[u8]) {
        self.validity_hasher.write(&[1_u8]);
        self.values_hasher.write(slice);
        self.len += slice.len();
    }

    fn consume_null(&mut self) {
        self.validity_hasher.write(&[0_u8]);
        self.null_count += 1;
    }
}

impl PrimitiveColumnSink for VecSink {
    type Result = (Vec<Vec<u8>>, Vec<bool>);

    fn into_result(self) -> Self::Result {
        (self.values, self.validity)
    }

    fn consume_value(&mut self, slice: &[u8]) {
        self.validity.push(true);
        self.values.push(slice.to_vec())
    }

    fn consume_null(&mut self) {
        self.validity.push(false);
        self.values.push(vec![]);
    }
}

impl<S: PrimitiveColumnSink> Sink for TableSink<S>
where
    S::Result: Display,
{
    type Error = arrow::error::ArrowError;
    type Result = TableResult<S::Result>;

    fn consume_batch(&mut self, batch: RecordBatch) -> Result<(), Self::Error> {
        self.record_batch_count += 1;
        self.tuple_count += batch.num_rows();

        for (i, sink) in self.column_sinks.iter_mut().enumerate() {
            let column = batch.column(i);
            self.bytes += column.get_buffer_memory_size();
            hash_one(column, sink);
        }
        return Ok(());

        fn hash_one<S: PrimitiveColumnSink>(array: &Arc<dyn arrow::array::Array>, sink: &mut S) {
            use arrow::datatypes::*;

            if let Some(binary) = array.as_binary_opt::<i32>() {
                hash_generic(binary, sink);
            }
            if let Some(binary) = array.as_binary_opt::<i64>() {
                hash_generic(binary, sink);
            }
            if let Some(binary) = array.as_binary_view_opt() {
                hash_generic(binary, sink);
            }
            if let Some(bools) = array.as_boolean_opt() {
                hash_generic(bools, sink);
            }
            if let Some(binary) = array.as_fixed_size_binary_opt() {
                hash_generic(binary, sink);
            }
            if let Some(ints) = array.as_primitive_opt::<Int8Type>() {
                hash_generic(ints, sink);
            }
            if let Some(ints) = array.as_primitive_opt::<UInt8Type>() {
                hash_generic(ints, sink);
            }
            if let Some(ints) = array.as_primitive_opt::<Int16Type>() {
                hash_generic(ints, sink);
            }
            if let Some(ints) = array.as_primitive_opt::<UInt16Type>() {
                hash_generic(ints, sink);
            }
            if let Some(ints) = array.as_primitive_opt::<Int32Type>() {
                hash_generic(ints, sink);
            }
            if let Some(ints) = array.as_primitive_opt::<UInt32Type>() {
                hash_generic(ints, sink);
            }
            if let Some(ints) = array.as_primitive_opt::<Int64Type>() {
                hash_generic(ints, sink);
            }
            if let Some(ints) = array.as_primitive_opt::<UInt64Type>() {
                hash_generic(ints, sink);
            }
            if let Some(ints) = array.as_primitive_opt::<Date32Type>() {
                hash_generic(ints, sink);
            }
            if let Some(ints) = array.as_primitive_opt::<Date64Type>() {
                hash_generic(ints, sink);
            }
            if let Some(string) = array.as_string_opt::<i32>() {
                hash_generic(string, sink);
            }
            if let Some(string) = array.as_string_opt::<i64>() {
                hash_generic(string, sink);
            }
            if let Some(string) = array.as_string_view_opt() {
                hash_generic(string, sink);
            }

            fn hash_generic<'a, A: IntoIterator<Item = Option<B>>, B: ByteRepr + 'a, S: PrimitiveColumnSink>(
                accessor: A,
                sink: &mut S,
            ) {
                for item in accessor {
                    if let Some(value) = item {
                        value.pass_as_bytes(|b| sink.consume_value(b));
                    } else {
                        sink.consume_null();
                    }
                }
            }
        }
    }

    fn into_result(self) -> Self::Result {
        let column_results = self.column_sinks.into_iter().map(|x| x.into_result()).collect();
        TableResult {
            column_results,
            schema: self.schema,
            record_batch_count: self.record_batch_count,
            tuple_count: self.tuple_count,
            bytes: self.bytes,
        }
    }
}

impl Display for Checksum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} bytes, {} nulls (0x{:0>8x})",
            self.len, self.null_count, self.hash
        )
    }
}

impl<R> TableResult<R> {
    pub fn total_bytes(&self) -> usize {
        self.bytes
    }
}

impl<R: Display> Display for TableResult<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Arrow report:  {} tuples in {} batches, {} bytes total",
            self.tuple_count, self.record_batch_count, self.bytes
        )?;
        writeln!(f, "               Schema: {}", self.schema)?;
        for (idx, column_result) in self.column_results.iter().enumerate() {
            writeln!(f, "               Column {idx}: {}", column_result)?;
        }
        Ok(())
    }
}

impl Debug for ChecksumSink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChecksumSink")
            .field("len", &self.len)
            .field("null_count", &self.null_count)
            .field("values_hasher (current value)", &self.values_hasher.finish())
            .field("validity_hasher (current value)", &self.validity_hasher.finish())
            .finish()
    }
}

/// Utility trait to allow representing types returned from Arrow arrays as consistent
/// byte representations.
///
/// Used for sinks.
pub trait ByteRepr {
    fn pass_as_bytes<F, R>(&self, f: F) -> R
    where
        F: FnMut(&[u8]) -> R;
}

impl ByteRepr for &[u8] {
    fn pass_as_bytes<F, R>(&self, mut f: F) -> R
    where
        F: FnMut(&[u8]) -> R,
    {
        f(&(self.len() as u32).to_le_bytes());
        f(self)
    }
}

impl ByteRepr for &str {
    fn pass_as_bytes<F, R>(&self, mut f: F) -> R
    where
        F: FnMut(&[u8]) -> R,
    {
        f(&(self.len() as u32).to_le_bytes());
        f(self.as_bytes())
    }
}

impl ByteRepr for bool {
    fn pass_as_bytes<F, R>(&self, mut f: F) -> R
    where
        F: FnMut(&[u8]) -> R,
    {
        if *self {
            f(&[1])
        } else {
            f(&[0])
        }
    }
}

macro_rules! byte_repr_for_int {
    ($int:ty) => {
        impl ByteRepr for $int {
            fn pass_as_bytes<F, R>(&self, mut f: F) -> R
            where
                F: FnMut(&[u8]) -> R,
            {
                f(&self.to_le_bytes())
            }
        }
    };
}

byte_repr_for_int!(i8);
byte_repr_for_int!(u8);
byte_repr_for_int!(i16);
byte_repr_for_int!(u16);
byte_repr_for_int!(i32);
byte_repr_for_int!(u32);
byte_repr_for_int!(i64);
byte_repr_for_int!(u64);
byte_repr_for_int!(i128);
byte_repr_for_int!(u128);
