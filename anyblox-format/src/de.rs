use crate::{
    error::{AnyBloxFormatError, Error},
    model::*,
    schema,
};
use arrow::array::{Array, AsArray};
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
    ptr::NonNull,
    sync::Arc,
};

#[cfg(feature = "serde")]
pub fn deserialize_metadata_from_file(path: &Path) -> Result<Metadata, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let ext = path.extension().map(|x| x.to_string_lossy());
    match ext.as_deref() {
        Some("toml") | None => read_metadata_toml(reader),
        Some("json") => read_metadata_json(reader),
        Some(ext) => Err(AnyBloxFormatError::unsupported_file_extension(ext).into()),
    }
}

#[cfg(feature = "serde")]
fn read_metadata_toml<R: Read>(mut read: R) -> Result<Metadata, Error> {
    let mut str = String::new();
    read.read_to_string(&mut str)?;
    let metadata = toml::de::from_str(&str).map_err(AnyBloxFormatError::deserialization_error)?;
    Ok(metadata)
}

#[cfg(feature = "serde")]
fn read_metadata_json<R: Read>(read: R) -> Result<Metadata, Error> {
    let metadata = serde_json::de::from_reader(read).map_err(AnyBloxFormatError::deserialization_error)?;
    Ok(metadata)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<AnyBloxFile<'_>, Error> {
    use arrow::{
        error::ArrowError,
        ipc::{convert, gen::File::root_as_footer, reader::read_footer_length},
    };
    let buf = unsafe {
        arrow::buffer::Buffer::from_custom_allocation(
            NonNull::new_unchecked(bytes.as_ptr().cast_mut()),
            bytes.len(),
            Arc::new(()),
        )
    };

    let trailer_start = buf.len() - 10;
    let footer_len = read_footer_length(buf[trailer_start..].try_into().unwrap())?;
    let footer = root_as_footer(&buf[trailer_start - footer_len..trailer_start])
        .map_err(|err| ArrowError::ParseError(format!("Unable to get root as footer: {err:?}")))?;

    let ipc_schema = footer.schema().unwrap();
    if !ipc_schema.endianness().equals_to_target_endianness() {
        return Err(ArrowError::IpcError(
            "the endianness of the source system does not match the endianness of the target system.".to_owned(),
        )
        .into());
    }

    let schema = Arc::new(convert::fb_to_schema(ipc_schema));

    let decoder = arrow::ipc::reader::FileDecoder::new(schema.clone(), footer.version());

    let batches = footer.recordBatches().unwrap();

    if batches.len() != 1 {
        return Err(AnyBloxFormatError::invalid_number_of_batches(batches.len()).into());
    }

    let block = batches.get(0);
    let block_len = block.bodyLength() as usize + block.metaDataLength() as usize;
    let data = buf.slice_with_length(block.offset() as _, block_len);
    let batch = decoder.read_record_batch(block, &data).unwrap().unwrap();

    let mut metadata = HashMap::new();
    let metadata_vector = footer
        .custom_metadata()
        .ok_or_else(AnyBloxFormatError::custom_metadata_missing)?;
    for kv in metadata_vector.into_iter() {
        metadata.insert(kv.key().unwrap().to_string(), kv.value().unwrap().to_string());
    }

    deserialize_arrow(&schema, &metadata, batch)
}

pub fn deserialize_file<R: io::Read + io::Seek>(read: R) -> Result<AnyBloxFile<'static>, Error> {
    let mut reader = arrow::ipc::reader::FileReader::try_new(read, None)?;
    let schema = reader.schema();

    if reader.num_batches() != 1 {
        return Err(AnyBloxFormatError::invalid_number_of_batches(reader.num_batches()).into());
    }
    let batch = reader.next().expect("num_batches == 1")?;
    let metadata = reader.custom_metadata();

    deserialize_arrow(schema.as_ref(), metadata, batch)
}

pub fn deserialize_arrow(
    schema: &arrow::datatypes::Schema,
    metadata: &HashMap<String, String>,
    batch: arrow::record_batch::RecordBatch,
) -> Result<AnyBloxFile<'static>, Error> {
    use crate::metadata::keys;
    let version = metadata
        .get(keys::VERSION)
        .ok_or_else(|| AnyBloxFormatError::missing_metadata_key(keys::VERSION))?;
    let version = match version.as_str() {
        "0.6" => Ok(AnyBloxVersion::V0_6),
        "0.7" => Ok(AnyBloxVersion::V0_7),
        "0.8" => Ok(AnyBloxVersion::V0_8),
        "0.9" => Ok(AnyBloxVersion::V0_9),
        "0.10" => Ok(AnyBloxVersion::V0_10),
        _ => Err(AnyBloxFormatError::invalid_metadata_format(keys::VERSION, version)),
    }?;

    let (schema_idx, decoder_idx, data_idx) =
        extract_fields(schema).ok_or_else(|| AnyBloxFormatError::invalid_fields(schema))?;

    if batch.num_rows() != 1 {
        return Err(AnyBloxFormatError::invalid_number_of_rows(batch.num_rows()).into());
    }

    let schema_array = batch.column(schema_idx).as_list::<i32>();
    let decoder_array = batch.column(decoder_idx).as_binary::<i32>();
    let data_array = batch.column(data_idx).as_binary::<i64>();

    if decoder_array.is_null(0) {
        return Err(AnyBloxFormatError::null_decoder().into());
    }

    if schema_array.is_null(0) {
        return Err(AnyBloxFormatError::null_schema().into());
    }

    let field_list = schema_array.value(0);
    let schema = deserialize_schema(field_list.as_struct());

    let ty = metadata
        .get(keys::TYPE)
        .ok_or_else(|| AnyBloxFormatError::missing_metadata_key(keys::TYPE))?;
    let ty: AnyBloxType = match ty.as_str() {
        "SELF-CONTAINED" => Ok(AnyBloxType::SelfContained),
        "EXTENSION" => Ok(AnyBloxType::Extension),
        _ => Err(AnyBloxFormatError::invalid_metadata_format(keys::TYPE, ty)),
    }?;

    if data_array.is_null(0) {
        if ty == AnyBloxType::SelfContained {
            return Err(AnyBloxFormatError::null_data_in_self_contained().into());
        }
    } else if ty == AnyBloxType::Extension {
        return Err(AnyBloxFormatError::non_null_data_in_extension().into());
    };

    let decoder_uri = metadata
        .get(keys::decoder::URI)
        .ok_or_else(|| AnyBloxFormatError::missing_metadata_key(keys::decoder::URI))?;
    let data_name = metadata
        .get(keys::data::NAME)
        .ok_or_else(|| AnyBloxFormatError::missing_metadata_key(keys::data::NAME))?;
    let data_count: u64 = metadata
        .get(keys::data::COUNT)
        .ok_or_else(|| AnyBloxFormatError::missing_metadata_key(keys::data::COUNT))?
        .parse()
        .map_err(|err| AnyBloxFormatError::invalid_metadata_format(keys::data::COUNT, err))?;

    let mut decoder_metadata = DecoderMetadataBuilder::new(decoder_uri.clone());
    let mut data_metadata = DataMetadataBuilder::new(data_name.clone(), data_count);

    if let Some(description) = metadata.get(keys::decoder::DESCRIPTION) {
        decoder_metadata.set_description(description.clone());
    }
    if let Some(license) = metadata.get(keys::decoder::LICENSE) {
        decoder_metadata.set_description(license.clone());
    }
    if let Some(batch_size_str) = metadata.get(keys::decoder::MIN_BATCH_SIZE) {
        let value = batch_size_str
            .parse()
            .map_err(|err| AnyBloxFormatError::invalid_metadata_format(keys::decoder::MIN_BATCH_SIZE, err))?;
        decoder_metadata.set_min_batch_size(value);
    }
    if let Some(checksum) = metadata.get(keys::decoder::CHECKSUM_BLAKE3) {
        decoder_metadata.set_checksum_blake3(checksum.clone());
    }

    if let Some(description) = metadata.get(keys::data::DESCRIPTION) {
        data_metadata.set_description(description.clone());
    }
    if let Some(size_str) = metadata.get(keys::data::SIZE_IN_BYTES) {
        let value = size_str
            .parse()
            .map_err(|err| AnyBloxFormatError::invalid_metadata_format(keys::data::SIZE_IN_BYTES, err))?;
        data_metadata.set_size_in_bytes(value);
    };

    let decoder_metadata = decoder_metadata.finish();
    let data_metadata = data_metadata.finish();
    let metadata = Metadata::new(ty, version, schema, decoder_metadata, data_metadata);

    let bundle = match ty {
        AnyBloxType::SelfContained => {
            AnyBloxSelfContained::new(metadata, decoder_array.clone(), data_array.clone()).into()
        }
        AnyBloxType::Extension => AnyBloxExtension::new(metadata, decoder_array.clone()).into(),
    };

    return Ok(bundle);

    fn extract_fields(schema: &arrow::datatypes::Schema) -> Option<(usize, usize, usize)> {
        if schema.fields().len() != 3 {
            return None;
        }
        let (schema_idx, schema_fld) = schema.fields().find("schema")?;
        let (decoder_idx, decoder_fld) = schema.fields().find("decoder")?;
        let (data_idx, data_fld) = schema.fields().find("data")?;
        if schema_fld.is_nullable() || !schema_fld.data_type().equals_datatype(&schema::get_schema_datatype()) {
            return None;
        }
        if decoder_fld.is_nullable()
            || !decoder_fld
                .data_type()
                .equals_datatype(&arrow::datatypes::DataType::Binary)
        {
            return None;
        }
        if !data_fld.is_nullable()
            || !data_fld
                .data_type()
                .equals_datatype(&arrow::datatypes::DataType::LargeBinary)
        {
            return None;
        }

        Some((schema_idx, decoder_idx, data_idx))
    }
}

fn deserialize_schema(schema_array: &arrow::array::StructArray) -> Schema {
    let mut fields = Vec::with_capacity(schema_array.len());
    for i in 0..schema_array.len() {
        let name = schema_array.column(0).as_string::<i32>().value(i);
        let union_array = arrow::array::as_union_array(schema_array.column(1));
        let union_id = union_array.type_id(i);
        let datatype = schema::DATA_TYPE_LAYOUT
            .cast_by_id(union_id, union_array.child(union_id).as_ref(), i)
            .clone();
        let nullable = schema_array.column(2).as_boolean().value(i);

        fields.push(Field::new(name, datatype, nullable));
    }
    Schema::new(fields)
}
