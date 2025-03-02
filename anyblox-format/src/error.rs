use std::fmt::Display;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ArrowError(#[from] arrow::error::ArrowError),
    #[error(transparent)]
    AnyBloxFormatError(#[from] AnyBloxFormatError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Clone, thiserror::Error)]
pub struct AnyBloxFormatError {
    msg: String,
}

macro_rules! create {
    ($($arg:tt)*) => {
        AnyBloxFormatError {
            msg: format!($($arg)*),
        }
    };
}

impl AnyBloxFormatError {
    pub fn invalid_number_of_batches(num: usize) -> Self {
        create!("a valid anyblox file contains exactly one batch, this one contains {num}")
    }

    pub fn invalid_number_of_rows(num: usize) -> Self {
        create!("a valid anyblox file contains exactly one row, this one contains {num}")
    }

    pub fn missing_metadata_key(key: &str) -> Self {
        create!("required metadata field is missing: {key}")
    }

    pub(crate) fn custom_metadata_missing() -> Self {
        create!("custom metadata is missing from the file")
    }

    pub fn invalid_metadata_format<T: Display>(key: &str, msg: T) -> Self {
        create!("the value for the metadata field {key} is invalid: {msg}")
    }

    pub fn null_schema() -> Self {
        create!("the schema value is null")
    }

    pub fn null_decoder() -> Self {
        create!("the decoder value is null")
    }

    pub fn null_data_in_self_contained() -> Self {
        create!("the file type is SelfContained, but data is null")
    }

    pub fn non_null_data_in_extension() -> Self {
        create!("the file type is Extension, but data is not null")
    }

    pub fn unsupported_file_extension(ext: &str) -> Self {
        create!("{ext} file is not supported for anyblox metadata")
    }

    #[cfg(feature = "serde")]
    pub fn deserialization_error<E: serde::de::Error>(err: E) -> Self {
        create!("deserialization error: {err}")
    }

    pub fn invalid_fields(schema: &arrow::datatypes::Schema) -> Self {
        use std::fmt::Write;
        let mut msg = String::new();
        write!(
            &mut msg,
            "a valid anyblox file has exactly three fields:\n
        - `schema` of type List of Struct (Field),\n
        - `decoder` of type non-nullable Binary,\n
        - `data` of type LargeBinary;
        this file has the following fields:"
        )
        .unwrap();

        for field in schema.fields() {
            write!(&mut msg, "- `{}` of type {}", field.name(), field.data_type()).unwrap();
        }

        Self { msg }
    }
}

impl Display for AnyBloxFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
