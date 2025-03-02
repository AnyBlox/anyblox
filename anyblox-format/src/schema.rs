use crate::model::{self, Metadata};
use arrow::{
    array::AsArray,
    datatypes::{Field, UnionFields},
};
use std::sync::Arc;

pub fn get_anyblox_schema(_metadata: &Metadata) -> arrow::datatypes::Schema {
    use arrow::datatypes::{DataType, Field, SchemaBuilder};
    let mut builder = SchemaBuilder::new();
    let schema_field_struct = DataType::Struct(get_schema_struct_fields());
    let schema_field = Field::new(
        "schema",
        DataType::List(Field::new("field", schema_field_struct, false).into()),
        false,
    );
    let wasm_field = Field::new("decoder", DataType::Binary, false);
    let data_field = Field::new("data", DataType::LargeBinary, true);

    builder.push(schema_field);
    builder.push(wasm_field);
    builder.push(data_field);

    builder.finish()
}

pub fn get_schema_datatype() -> arrow::datatypes::DataType {
    arrow::datatypes::DataType::List(Arc::new(Field::new(
        "field",
        arrow::datatypes::DataType::Struct(get_schema_struct_fields()),
        false,
    )))
}

pub fn get_schema_struct_fields() -> arrow::datatypes::Fields {
    use arrow::datatypes::{DataType, Field, Fields};
    Fields::from(vec![
        Field::new("name", DataType::Utf8, false),
        Field::new("datatype", get_datatype_union_type(), false),
        Field::new("nullable", DataType::Boolean, false),
    ])
}

pub fn get_datatype_union_type() -> arrow::datatypes::DataType {
    use arrow::datatypes::{DataType, UnionMode};
    DataType::Union(get_datatype_union_fields(), UnionMode::Sparse)
}

pub const DATA_TYPE_COUNT: usize = 29;

pub fn get_datatype_union_fields() -> UnionFields {
    use arrow::datatypes::{DataType, Field};
    let fields = vec![
        Field::new("Null", DataType::Null, false),
        Field::new("Boolean", DataType::Null, false),
        Field::new("Int8", DataType::Null, false),
        Field::new("Int16", DataType::Null, false),
        Field::new("Int32", DataType::Null, false),
        Field::new("Int64", DataType::Null, false),
        Field::new("UInt8", DataType::Null, false),
        Field::new("UInt16", DataType::Null, false),
        Field::new("UInt32", DataType::Null, false),
        Field::new("UInt64", DataType::Null, false),
        Field::new("Float16", DataType::Null, false),
        Field::new("Float32", DataType::Null, false),
        Field::new("Float64", DataType::Null, false),
        Field::new("Timestamp", DataType::Null, false),
        Field::new("Date32", DataType::Null, false),
        Field::new("Date64", DataType::Null, false),
        Field::new("Time32", DataType::Null, false),
        Field::new("Time64", DataType::Null, false),
        Field::new("Duration", DataType::Null, false),
        Field::new("Interval", DataType::Null, false),
        Field::new("Binary", DataType::Null, false),
        Field::new("FixedBinary", DataType::Int32, false),
        Field::new("LargeBinary", DataType::Null, false),
        Field::new("BinaryView", DataType::Null, false),
        Field::new("Utf8", DataType::Null, false),
        Field::new("LargeUtf8", DataType::Null, false),
        Field::new("Utf8View", DataType::Null, false),
        Field::new("Decimal128", DataType::Null, false),
        Field::new("Decimal256", DataType::Null, false),
    ];
    let ids = 0..(fields.len() as i8);
    assert_eq!(fields.len(), DATA_TYPE_COUNT);
    UnionFields::new(ids, fields)
}

pub fn populate_datatype_union_arrays(types: &[model::DataType]) -> Vec<arrow::array::ArrayRef> {
    let len = types.len();
    let mut fixed_size_binary_array: Vec<Option<i32>> = vec![None; len];

    for (i, ty) in types.iter().enumerate() {
        match ty {
            model::DataType::FixedSizeBinary(s) => fixed_size_binary_array[i] = Some(*s),
            model::DataType::Timestamp(_, _)
            | model::DataType::Time32(_)
            | model::DataType::Time64(_)
            | model::DataType::Duration(_)
            | model::DataType::Interval(_)
            | model::DataType::Decimal128(_, _)
            | model::DataType::Decimal256(_, _) => todo!(),
            _ => (), // all other types have a null array
        }
    }

    let mut arrays: Vec<arrow::array::ArrayRef> = vec![Arc::new(arrow::array::NullArray::new(len)); DATA_TYPE_COUNT];

    arrays[21] = Arc::new(arrow::array::PrimitiveArray::<arrow::datatypes::Int32Type>::from(
        fixed_size_binary_array,
    ));

    arrays
}

pub struct DataTypeLayout;

pub const DATA_TYPE_LAYOUT: DataTypeLayout = DataTypeLayout::new();

impl DataTypeLayout {
    pub const fn new() -> Self {
        Self
    }

    pub fn id_of_datatype(&self, data_type: &model::DataType) -> i8 {
        match data_type {
            model::DataType::Null => 0,
            model::DataType::Boolean => 1,
            model::DataType::Int8 => 2,
            model::DataType::Int16 => 3,
            model::DataType::Int32 => 4,
            model::DataType::Int64 => 5,
            model::DataType::UInt8 => 6,
            model::DataType::UInt16 => 7,
            model::DataType::UInt32 => 8,
            model::DataType::UInt64 => 9,
            model::DataType::Float16 => 10,
            model::DataType::Float32 => 11,
            model::DataType::Float64 => 12,
            model::DataType::Timestamp(_, _) => 13,
            model::DataType::Date32 => 14,
            model::DataType::Date64 => 15,
            model::DataType::Time32(_) => 16,
            model::DataType::Time64(_) => 17,
            model::DataType::Duration(_) => 18,
            model::DataType::Interval(_) => 19,
            model::DataType::Binary => 20,
            model::DataType::FixedSizeBinary(_) => 21,
            model::DataType::LargeBinary => 22,
            model::DataType::BinaryView => 23,
            model::DataType::Utf8 => 24,
            model::DataType::LargeUtf8 => 25,
            model::DataType::Utf8View => 26,
            model::DataType::Decimal128(_, _) => 27,
            model::DataType::Decimal256(_, _) => 28,
        }
    }

    pub fn cast_by_id(&self, union_id: i8, as_ref: &dyn arrow::array::Array, idx: usize) -> model::DataType {
        match union_id {
            0 => model::DataType::Null,
            1 => model::DataType::Boolean,
            2 => model::DataType::Int8,
            3 => model::DataType::Int16,
            4 => model::DataType::Int32,
            5 => model::DataType::Int64,
            6 => model::DataType::UInt8,
            7 => model::DataType::UInt16,
            8 => model::DataType::UInt32,
            9 => model::DataType::UInt64,
            10 => model::DataType::Float16,
            11 => model::DataType::Float32,
            12 => model::DataType::Float64,
            13 => todo!(), // model::DataType::Timestamp(_, _)
            14 => model::DataType::Date32,
            15 => model::DataType::Date64,
            16 => todo!(), // model::DataType::Time32(_),
            17 => todo!(), // model::DataType::Time64(_),
            18 => todo!(), // model::DataType::Duration(_),
            19 => todo!(), // model::DataType::Interval(_),
            20 => model::DataType::Binary,
            21 => {
                assert_eq!(as_ref.data_type(), &arrow::datatypes::DataType::Int32);
                let int = as_ref.as_primitive::<arrow::datatypes::Int32Type>();
                model::DataType::FixedSizeBinary(int.value(idx))
            }
            22 => model::DataType::LargeBinary,
            23 => model::DataType::BinaryView,
            24 => model::DataType::Utf8,
            25 => model::DataType::LargeUtf8,
            26 => model::DataType::Utf8View,
            27 => todo!(), //model::DataType::Decimal128(_, _) => 27,
            28 => todo!(), // model::DataType::Decimal256(_, _) => 28,
            _ => unimplemented!(),
        }
    }
}
