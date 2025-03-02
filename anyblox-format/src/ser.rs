use crate::{error::Error, metadata, model::*, schema};
use arrow::{array::Array, ipc::writer::FileWriter};
use std::{io::Write, sync::Arc};

pub fn serialize_extension<W: Write>(metadata: &Metadata, wasm_bytes: &[u8], writer: W) -> Result<(), Error> {
    let schema = Arc::new(schema::get_anyblox_schema(metadata));

    let schema_array = build_schema_array(metadata);
    let wasm_array = {
        let mut builder = arrow::array::BinaryBuilder::new();
        builder.append_value(wasm_bytes);
        Arc::new(builder.finish())
    };
    let data_array = {
        let mut builder = arrow::array::LargeBinaryBuilder::new();
        builder.append_null();
        Arc::new(builder.finish())
    };

    {
        let mut arrow_writer = arrow::ipc::writer::FileWriter::try_new(writer, &schema)?;
        write_metadata_to_arrow(&mut arrow_writer, metadata)?;
        let record_batch =
            arrow::record_batch::RecordBatch::try_new(schema, vec![schema_array, wasm_array, data_array])
                .expect("record batch with correct sizes");
        arrow_writer.write(&record_batch)?;
        arrow_writer.finish()?;
    }

    Ok(())
}

pub fn serialize_self_contained<W: Write>(
    metadata: &Metadata,
    wasm_bytes: &[u8],
    data_bytes: &[u8],
    writer: W,
) -> Result<(), Error> {
    let schema = Arc::new(schema::get_anyblox_schema(metadata));

    let schema_array = build_schema_array(metadata);
    let wasm_array = {
        let mut builder = arrow::array::BinaryBuilder::new();
        builder.append_value(wasm_bytes);
        Arc::new(builder.finish())
    };
    let data_array = {
        let mut builder = arrow::array::LargeBinaryBuilder::new();
        builder.append_value(data_bytes);
        Arc::new(builder.finish())
    };

    {
        let mut arrow_writer = arrow::ipc::writer::FileWriter::try_new(writer, &schema)?;
        write_metadata_to_arrow(&mut arrow_writer, metadata)?;
        let record_batch =
            arrow::record_batch::RecordBatch::try_new(schema, vec![schema_array, wasm_array, data_array])
                .expect("record batch with correct sizes");
        arrow_writer.write(&record_batch)?;
        arrow_writer.finish()?;
    }

    Ok(())
}

fn build_schema_array(metadata: &Metadata) -> Arc<arrow::array::GenericListArray<i32>> {
    let mut name_field_builder = arrow::array::StringBuilder::new();
    let mut datatype_field_builder = DataTypeArrayBuilder::new(metadata.schema().fields().len());
    let mut nullable_field_builder = arrow::array::BooleanBuilder::new();
    for field in metadata.schema().fields() {
        name_field_builder.append_value(field.name());
        datatype_field_builder.append_type(field.ty());
        nullable_field_builder.append_value(field.nullable());
    }

    let name_array = name_field_builder.finish();
    let datatype_array = datatype_field_builder.finish();
    let nullable_array = nullable_field_builder.finish();

    let struct_array = arrow::array::StructArray::new(
        schema::get_schema_struct_fields(),
        vec![Arc::new(name_array), Arc::new(datatype_array), Arc::new(nullable_array)],
        None,
    );
    let offsets = arrow::buffer::OffsetBuffer::from_lengths([struct_array.len()]);
    let list = arrow::array::ListArray::new(
        Arc::new(arrow::datatypes::Field::new(
            "field",
            arrow::datatypes::DataType::Struct(schema::get_schema_struct_fields()),
            false,
        )),
        offsets,
        Arc::new(struct_array),
        None,
    );
    Arc::new(list)
}

fn write_metadata_to_arrow<W: Write>(writer: &mut FileWriter<W>, metadata: &Metadata) -> Result<(), Error> {
    use metadata::keys;
    writer.write_metadata(keys::TYPE, ToStringWrap(metadata.ty()));
    writer.write_metadata(keys::VERSION, ToStringWrap(metadata.version()));

    writer.write_metadata(keys::decoder::URI, metadata.decoder().uri());
    write_if_some(writer, keys::decoder::DESCRIPTION, metadata.decoder().description());
    write_if_some(writer, keys::decoder::LICENSE, metadata.decoder().license());
    write_if_some(writer, keys::decoder::CHECKSUM_BLAKE3, metadata.decoder().checksum_blake3());;
    write_if_some(writer, keys::decoder::MIN_BATCH_SIZE, metadata.decoder().min_batch_size());

    writer.write_metadata(keys::data::NAME, metadata.data().name());
    writer.write_metadata(keys::data::COUNT, ToStringWrap(metadata.data().count()));
    write_if_some(writer, keys::data::DESCRIPTION, metadata.data().description());
    write_if_some(writer, keys::data::SIZE_IN_BYTES, metadata.data().size_in_bytes());

    return Ok(());

    fn write_if_some<W: Write, S: ToString>(writer: &mut FileWriter<W>, key: &str, entry: Option<S>) {
        if let Some(value) = entry {
            writer.write_metadata(key, ToStringWrap(value))
        }
    }
}

struct ToStringWrap<S>(S);

impl<S: ToString> From<ToStringWrap<S>> for String {
    fn from(value: ToStringWrap<S>) -> Self {
        value.0.to_string()
    }
}

struct DataTypeArrayBuilder {
    types: Vec<DataType>,
}

impl DataTypeArrayBuilder {
    pub fn new(capacity: usize) -> Self {
        Self {
            types: Vec::with_capacity(capacity),
        }
    }

    pub fn append_type(&mut self, ty: &DataType) {
        self.types.push(ty.clone());
    }

    pub fn finish(self) -> arrow::array::UnionArray {
        let arrays = schema::populate_datatype_union_arrays(&self.types);
        let type_ids = self
            .types
            .iter()
            .map(|t| schema::DATA_TYPE_LAYOUT.id_of_datatype(t))
            .collect();

        arrow::array::UnionArray::try_new(schema::get_datatype_union_fields(), type_ids, None, arrays)
            .expect("DataTypeArrayBuilder should make correct arrays")
    }
}
