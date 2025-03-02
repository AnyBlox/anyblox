use arrow::{array::GenericByteArray, datatypes::GenericBinaryType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{fmt::Display, marker::PhantomData};

#[derive(Debug, Clone)]
pub enum AnyBloxFile<'a> {
    Extension(AnyBloxExtension<'a>),
    SelfContained(AnyBloxSelfContained<'a>),
}

#[derive(Debug, Clone)]
pub struct AnyBloxExtension<'a> {
    decoder: GenericByteArray<GenericBinaryType<i32>>,
    metadata: Metadata,
    phantom: PhantomData<&'a [u8]>,
}

#[derive(Debug, Clone)]
pub struct AnyBloxSelfContained<'a> {
    data: GenericByteArray<GenericBinaryType<i64>>,
    decoder: GenericByteArray<GenericBinaryType<i32>>,
    metadata: Metadata,
    phantom: PhantomData<&'a [u8]>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Metadata {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    ty: AnyBloxType,
    version: AnyBloxVersion,
    schema: Schema,
    decoder: DecoderMetadata,
    data: DataMetadata,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum AnyBloxType {
    #[cfg_attr(feature = "serde", serde(rename = "SELF-CONTAINED"))]
    SelfContained,
    #[cfg_attr(feature = "serde", serde(rename = "EXTENSION"))]
    Extension,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum AnyBloxVersion {
    // Introduction of the versioning field.
    #[cfg_attr(feature = "serde", serde(rename = "0.6"))]
    V0_6,
    // Added column projection.
    #[cfg_attr(feature = "serde", serde(rename = "0.7"))]
    V0_7,
    // Changed to standard Arrow C FFI
    #[cfg_attr(feature = "serde", serde(rename = "0.8"))]
    V0_8,
    // Added FixedSizeBinary to the schema
    #[cfg_attr(feature = "serde", serde(rename = "0.9"))]
    V0_9,
    // Reworked the metadata to the final paper version with a Decoder URI, checksum
    #[cfg_attr(feature = "serde", serde(rename = "0.10"))]
    V0_10,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Schema {
    fields: Vec<Field>,
}

fn _serde_yield_true() -> bool {
    true
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Field {
    name: String,
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    ty: DataType,
    #[cfg_attr(feature = "serde", serde(default = "_serde_yield_true"))]
    nullable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum DataType {
    Null,
    Boolean,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float16,
    Float32,
    Float64,
    Timestamp(TimeUnit, Option<String>),
    Date32,
    Date64,
    Time32(TimeUnit),
    Time64(TimeUnit),
    Duration(TimeUnit),
    Interval(IntervalUnit),
    Binary,
    FixedSizeBinary(i32),
    LargeBinary,
    BinaryView,
    Utf8,
    LargeUtf8,
    Utf8View,
    Decimal128(u8, i8),
    Decimal256(u8, i8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum TimeUnit {
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum IntervalUnit {
    YearMonth,
    DayTime,
    MonthDayNano,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct DecoderMetadata {
    uri: String,
    description: Option<String>,
    license: Option<String>,
    checksum_blake3: Option<String>,
    min_batch_size: Option<u64>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct DataMetadata {
    name: String,
    count: u64,
    description: Option<String>,
    size_in_bytes: Option<u64>,
}

impl Metadata {
    pub fn new(
        ty: AnyBloxType,
        version: AnyBloxVersion,
        schema: Schema,
        decoder: DecoderMetadata,
        data: DataMetadata,
    ) -> Self {
        Self {
            ty,
            version,
            schema,
            data,
            decoder,
        }
    }

    pub fn ty(&self) -> AnyBloxType {
        self.ty
    }

    pub fn version(&self) -> AnyBloxVersion {
        self.version
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn data(&self) -> &DataMetadata {
        &self.data
    }

    pub fn decoder(&self) -> &DecoderMetadata {
        &self.decoder
    }
}

impl Display for AnyBloxType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SelfContained => write!(f, "SELF-CONTAINED"),
            Self::Extension => write!(f, "EXTENSION"),
        }
    }
}

impl Display for AnyBloxVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::V0_6 => write!(f, "0.6"),
            Self::V0_7 => write!(f, "0.7"),
            Self::V0_8 => write!(f, "0.8"),
            Self::V0_9 => write!(f, "0.9"),
            Self::V0_10 => write!(f, "0.10"),
        }
    }
}

impl Schema {
    pub fn new(fields: Vec<Field>) -> Self {
        Self { fields }
    }

    pub fn fields(&self) -> &[Field] {
        &self.fields
    }
}

impl Field {
    pub fn new<S: ToString>(name: S, datatype: DataType, nullable: bool) -> Self {
        Self {
            name: name.to_string(),
            ty: datatype,
            nullable,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ty(&self) -> &DataType {
        &self.ty
    }

    pub fn nullable(&self) -> bool {
        self.nullable
    }
}

pub struct DecoderMetadataBuilder {
    inner: DecoderMetadata,
}

pub struct DataMetadataBuilder {
    inner: DataMetadata,
}

impl DecoderMetadataBuilder {
    pub fn new(uri: String) -> Self {
        Self {
            inner: DecoderMetadata {
                uri,
                description: None,
                license: None,
                checksum_blake3: None,
                min_batch_size: None,
            },
        }
    }

    pub fn set_description(&mut self, value: String) -> &mut Self {
        self.inner.description = Some(value);
        self
    }

    pub fn set_license(&mut self, value: String) -> &mut Self {
        self.inner.license = Some(value);
        self
    }

    pub fn set_checksum_blake3(&mut self, value: String) -> &mut Self {
        self.inner.checksum_blake3 = Some(value);
        self
    }

    pub fn set_min_batch_size(&mut self, value: u64) -> &mut Self {
        self.inner.min_batch_size = Some(value);
        self
    }

    pub fn finish(self) -> DecoderMetadata {
        self.inner
    }
}

impl From<DecoderMetadataBuilder> for DecoderMetadata {
    fn from(value: DecoderMetadataBuilder) -> Self {
        value.finish()
    }
}

impl DataMetadataBuilder {
    pub fn new(name: String, count: u64) -> Self {
        Self {
            inner: DataMetadata {
                name,
                count,
                description: None,
                size_in_bytes: None,
            },
        }
    }

    pub fn set_description(&mut self, value: String) -> &mut Self {
        self.inner.description = Some(value);
        self
    }

    pub fn set_size_in_bytes(&mut self, value: u64) -> &mut Self {
        self.inner.size_in_bytes = Some(value);
        self
    }

    pub fn finish(self) -> DataMetadata {
        self.inner
    }
}

impl From<DataMetadataBuilder> for DataMetadata {
    fn from(value: DataMetadataBuilder) -> Self {
        value.finish()
    }
}

impl DecoderMetadata {
    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn license(&self) -> Option<&str> {
        self.license.as_deref()
    }

    pub fn checksum_blake3(&self) -> Option<&str> {
        self.checksum_blake3.as_deref()
    }

    pub fn min_batch_size(&self) -> Option<u64> {
        self.min_batch_size
    }
}

impl DataMetadata {
    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn size_in_bytes(&self) -> Option<u64> {
        self.size_in_bytes
    }
}

impl AnyBloxFile<'_> {
    pub fn is_extension(&self) -> bool {
        matches!(self, AnyBloxFile::Extension(_))
    }

    pub fn is_self_contained(&self) -> bool {
        matches!(self, AnyBloxFile::SelfContained(_))
    }
}

impl AnyBloxExtension<'_> {
    pub fn new(metadata: Metadata, decoder: GenericByteArray<GenericBinaryType<i32>>) -> Self {
        Self {
            metadata,
            decoder,
            phantom: PhantomData,
        }
    }

    pub fn decoder(&self) -> &[u8] {
        self.decoder.value(0)
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

impl<'a> From<AnyBloxExtension<'a>> for AnyBloxFile<'a> {
    fn from(value: AnyBloxExtension<'a>) -> Self {
        Self::Extension(value)
    }
}

impl AnyBloxSelfContained<'_> {
    pub fn new(
        metadata: Metadata,
        decoder: GenericByteArray<GenericBinaryType<i32>>,
        data: GenericByteArray<GenericBinaryType<i64>>,
    ) -> Self {
        Self {
            data,
            decoder,
            metadata,
            phantom: PhantomData,
        }
    }

    pub fn data(&self) -> &[u8] {
        self.data.value(0)
    }

    pub fn decoder(&self) -> &[u8] {
        self.decoder.value(0)
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

impl<'a> From<AnyBloxSelfContained<'a>> for AnyBloxFile<'a> {
    fn from(value: AnyBloxSelfContained<'a>) -> Self {
        Self::SelfContained(value)
    }
}

impl From<&Schema> for arrow::datatypes::Schema {
    fn from(value: &Schema) -> Self {
        let mut builder = arrow::datatypes::SchemaBuilder::new();
        for field in value.fields() {
            let arrow_field: arrow::datatypes::Field = field.into();
            builder.push(arrow_field);
        }
        builder.finish()
    }
}

impl From<&arrow::datatypes::Field> for Field {
    fn from(value: &arrow::datatypes::Field) -> Self {
        Self {
            name: value.name().clone(),
            nullable: value.is_nullable(),
            ty: value.data_type().into(),
        }
    }
}

impl From<&Field> for arrow::datatypes::Field {
    fn from(value: &Field) -> Self {
        Self::new(value.name(), value.ty().into(), value.nullable())
    }
}

impl From<&arrow::datatypes::DataType> for DataType {
    fn from(value: &arrow::datatypes::DataType) -> Self {
        use arrow::datatypes::DataType as Arrow;
        match value {
            Arrow::Null => Self::Null,
            Arrow::Boolean => Self::Boolean,
            Arrow::Int8 => Self::Int8,
            Arrow::Int16 => Self::Int16,
            Arrow::Int32 => Self::Int32,
            Arrow::Int64 => Self::Int64,
            Arrow::UInt8 => Self::UInt8,
            Arrow::UInt16 => Self::UInt16,
            Arrow::UInt32 => Self::UInt32,
            Arrow::UInt64 => Self::UInt64,
            Arrow::Float16 => Self::Float16,
            Arrow::Float32 => Self::Float32,
            Arrow::Float64 => Self::Float64,
            Arrow::Timestamp(tu, tz) => Self::Timestamp((*tu).into(), tz.as_ref().map(|x| x.to_string())),
            Arrow::Date32 => Self::Date32,
            Arrow::Date64 => Self::Date64,
            Arrow::Time32(tu) => Self::Time32((*tu).into()),
            Arrow::Time64(tu) => Self::Time64((*tu).into()),
            Arrow::Duration(tu) => Self::Duration((*tu).into()),
            Arrow::Interval(iu) => Self::Interval((*iu).into()),
            Arrow::Binary => Self::Binary,
            Arrow::FixedSizeBinary(s) => Self::FixedSizeBinary(*s),
            Arrow::LargeBinary => Self::LargeBinary,
            Arrow::BinaryView => Self::BinaryView,
            Arrow::Utf8 => Self::Utf8,
            Arrow::LargeUtf8 => Self::LargeUtf8,
            Arrow::Utf8View => Self::Utf8View,
            Arrow::Decimal128(p, s) => Self::Decimal128(*p, *s),
            Arrow::Decimal256(p, s) => Self::Decimal256(*p, *s),
            _ => unimplemented!(),
        }
    }
}

impl From<&DataType> for arrow::datatypes::DataType {
    fn from(value: &DataType) -> Self {
        match value {
            DataType::Null => Self::Null,
            DataType::Boolean => Self::Boolean,
            DataType::Int8 => Self::Int8,
            DataType::Int16 => Self::Int16,
            DataType::Int32 => Self::Int32,
            DataType::Int64 => Self::Int64,
            DataType::UInt8 => Self::UInt8,
            DataType::UInt16 => Self::UInt16,
            DataType::UInt32 => Self::UInt32,
            DataType::UInt64 => Self::UInt64,
            DataType::Float16 => Self::Float16,
            DataType::Float32 => Self::Float32,
            DataType::Float64 => Self::Float64,
            DataType::Timestamp(tu, tz) => Self::Timestamp((*tu).into(), tz.as_ref().map(|x| x.clone().into())),
            DataType::Date32 => Self::Date32,
            DataType::Date64 => Self::Date64,
            DataType::Time32(tu) => Self::Time32((*tu).into()),
            DataType::Time64(tu) => Self::Time64((*tu).into()),
            DataType::Duration(tu) => Self::Duration((*tu).into()),
            DataType::Interval(iu) => Self::Interval((*iu).into()),
            DataType::Binary => Self::Binary,
            DataType::FixedSizeBinary(s) => Self::FixedSizeBinary(*s),
            DataType::LargeBinary => Self::LargeBinary,
            DataType::BinaryView => Self::BinaryView,
            DataType::Utf8 => Self::Utf8,
            DataType::LargeUtf8 => Self::LargeUtf8,
            DataType::Utf8View => Self::Utf8View,
            DataType::Decimal128(p, s) => Self::Decimal128(*p, *s),
            DataType::Decimal256(p, s) => Self::Decimal256(*p, *s),
        }
    }
}

impl From<arrow::datatypes::TimeUnit> for TimeUnit {
    fn from(value: arrow::datatypes::TimeUnit) -> Self {
        use arrow::datatypes::TimeUnit as Arrow;
        match value {
            Arrow::Second => Self::Second,
            Arrow::Millisecond => Self::Millisecond,
            Arrow::Microsecond => Self::Microsecond,
            Arrow::Nanosecond => Self::Nanosecond,
        }
    }
}

impl From<arrow::datatypes::IntervalUnit> for IntervalUnit {
    fn from(value: arrow::datatypes::IntervalUnit) -> Self {
        use arrow::datatypes::IntervalUnit as Arrow;
        match value {
            Arrow::YearMonth => Self::YearMonth,
            Arrow::DayTime => Self::DayTime,
            Arrow::MonthDayNano => Self::MonthDayNano,
        }
    }
}

impl From<TimeUnit> for arrow::datatypes::TimeUnit {
    fn from(value: TimeUnit) -> Self {
        match value {
            TimeUnit::Second => Self::Second,
            TimeUnit::Millisecond => Self::Millisecond,
            TimeUnit::Microsecond => Self::Microsecond,
            TimeUnit::Nanosecond => Self::Nanosecond,
        }
    }
}

impl From<IntervalUnit> for arrow::datatypes::IntervalUnit {
    fn from(value: IntervalUnit) -> Self {
        match value {
            IntervalUnit::YearMonth => Self::YearMonth,
            IntervalUnit::DayTime => Self::DayTime,
            IntervalUnit::MonthDayNano => Self::MonthDayNano,
        }
    }
}
