use anyblox::version::AnyBloxVersion;
use anyblox_format::model::{DataMetadata, DecoderMetadata, Schema};
use anyhow::{anyhow, Result};
use clap::Parser;
use std::{
    fmt::Display,
    fs::File,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Parser)]
struct Args {
    file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let summary = summarize_anyblox(&args.file)?;

    println!("{summary}");

    Ok(())
}

fn summarize_anyblox(path: &Path) -> Result<Summary> {
    let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
    let file = File::open(path)?;
    let total_size_in_bytes = file.metadata()?.size() as usize;
    let anyblox_file = anyblox_format::de::deserialize_file(file)?;

    let summary = match anyblox_file {
        anyblox_format::model::AnyBloxFile::Extension(file) => Summary {
            file_name,
            ty: AnyBloxType::Extension,
            version: file.metadata().version(),
            arrow: ArrowSummary { total_size_in_bytes },
            schema: file.metadata().schema().clone(),
            decoder: DecoderSummary {
                size_in_bytes: file.decoder().len(),
                metadata: file.metadata().decoder().clone(),
            },
            data: DataSummary {
                compressed_size: None,
                metadata: file.metadata().data().clone(),
            },
        },
        anyblox_format::model::AnyBloxFile::SelfContained(file) => Summary {
            file_name,
            ty: AnyBloxType::SelfContained,
            version: file.metadata().version(),
            arrow: ArrowSummary { total_size_in_bytes },
            schema: file.metadata().schema().clone(),
            decoder: DecoderSummary {
                size_in_bytes: file.decoder().len(),
                metadata: file.metadata().decoder().clone(),
            },
            data: DataSummary {
                compressed_size: Some(file.data().len()),
                metadata: file.metadata().data().clone(),
            },
        },
    };

    Ok(summary)
}

#[derive(Debug, Clone)]
struct Summary {
    file_name: String,
    arrow: ArrowSummary,
    ty: AnyBloxType,
    version: AnyBloxVersion,
    schema: Schema,
    decoder: DecoderSummary,
    data: DataSummary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum AnyBloxType {
    SelfContained,
    Extension,
}

#[derive(Debug, Clone)]
struct ArrowSummary {
    total_size_in_bytes: usize,
}

#[derive(Debug, Clone)]
struct DecoderSummary {
    size_in_bytes: usize,
    metadata: DecoderMetadata,
}

#[derive(Debug, Clone)]
struct DataSummary {
    compressed_size: Option<usize>,
    metadata: DataMetadata,
}

impl Summary {
    pub fn arrow_overhead_in_bytes(&self) -> usize {
        self.arrow.total_size_in_bytes - self.data.compressed_size.unwrap_or(0) - self.decoder.size_in_bytes
    }
}

impl Display for Summary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const SEP: &str = "=========";
        writeln!(f, "AnyBlox file {}", self.file_name)?;
        writeln!(f, "{SEP}")?;
        writeln!(f, "TYPE:                {}", self.ty)?;
        writeln!(f, "VERSION:             {}", self.version)?;
        writeln!(f, "{SEP}")?;
        writeln!(f, "FILE STATS:")?;
        writeln!(f, "  Total size:        {}B", self.arrow.total_size_in_bytes)?;
        writeln!(f, "  Arrow overhead:    {}B", self.arrow_overhead_in_bytes())?;
        writeln!(f, "{SEP}")?;
        writeln!(f, "SCHEMA:")?;
        for (i, fld) in self.schema.fields().iter().enumerate() {
            writeln!(f, "  Field #{i}:")?;
            writeln!(f, "    Name:            {}", fld.name())?;
            writeln!(f, "    Type:            {:?}", fld.ty())?;
            writeln!(f, "    Nullable:        {}", fld.nullable())?;
        }
        writeln!(f, "{SEP}")?;
        writeln!(f, "DECODER:")?;
        writeln!(f, "  URI:              {}", self.decoder.metadata.uri())?;
        writeln!(f, "  Size:              {}B", self.decoder.size_in_bytes)?;
        writeln!(
            f,
            "  Description:       {}",
            self.decoder.metadata.description().unwrap_or("<none>")
        )?;
        writeln!(
            f,
            "  License:           {}",
            self.decoder.metadata.license().unwrap_or("<none>")
        )?;
        writeln!(
            f,
            "  blake3 checksum:   {}",
            self.decoder.metadata.checksum_blake3().unwrap_or("<none>")
        )?;
        if let Some(min_batch) = self.decoder.metadata.min_batch_size() {
            writeln!(f, "  min batch size:    {min_batch}")?;
        } else {
            writeln!(f, "  min batch size:    <none>")?;
        }
        writeln!(f, "{SEP}")?;
        writeln!(f, "DATA:")?;

        match self.ty {
            AnyBloxType::SelfContained => {
                writeln!(f, "  Name:              {}", self.data.metadata.name())?;
                if let Some(bytes) = self.data.compressed_size {
                    writeln!(f, "  Size (compressed): {bytes}B")?;
                } else {
                    writeln!(f, "  Size (compressed): <none>")?;
                }
                writeln!(f, "  Tuple count:       {}", self.data.metadata.count())?;
                writeln!(
                    f,
                    "  Description:       {}",
                    self.data.metadata.description().unwrap_or("<none>")
                )?;
                if let Some(bytes) = self.data.metadata.size_in_bytes() {
                    writeln!(f, "  Size (raw):        {bytes}B")?;
                } else {
                    writeln!(f, "  Size (raw):        <none>")?;
                }
            }
            AnyBloxType::Extension => {
                writeln!(f, "no data in extension file")?;
            }
        }

        Ok(())
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

impl FromStr for AnyBloxType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "SELF-CONTAINED" => Ok(Self::SelfContained),
            "EXTENSION" => Ok(Self::Extension),
            _ => Err(anyhow!("invalid type string {s}")),
        }
    }
}
