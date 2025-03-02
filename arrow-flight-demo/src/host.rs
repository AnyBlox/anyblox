use anyhow::{bail, Result};
use arrow::{
    array::{LargeBinaryArray, RecordBatch},
    buffer::{Buffer, OffsetBuffer},
};
use arrow_flight::{
    encode::{FlightDataEncoder, FlightDataEncoderBuilder},
    error::FlightError,
    FlightClient,
};
use arrow_flight_demo::Sink;
use bytes::Bytes;
use clap::Parser;
use futures::{Stream, StreamExt};
use std::{
    cmp,
    path::{Path, PathBuf},
    sync::Arc,
    time::Instant,
};
use tokio::{fs, io::AsyncReadExt};
use tonic::transport::Channel;

#[derive(Parser)]
struct Args {
    #[clap(short = 'p', long)]
    input_path: PathBuf,
    #[clap(alias = "addr", long)]
    container_addr: String,
    #[clap(required = false, default_value = "0", short = 's', long)]
    start_tuple: usize,
    #[clap(short = 'c')]
    tuple_count: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    arrow_flight_demo::init_logging();
    let args = Args::parse();
    let mut client = build_client(args.container_addr.clone()).await?;
    let flight_data = build_flight_data(&args.input_path, args.start_tuple, args.tuple_count).await?;

    let mut sink = arrow_flight_demo::EmptySink::new(expected_schema().into());

    let job_start = Instant::now();

    let mut stream = client.do_exchange(flight_data).await?;
    let read_start = Instant::now();
    while let Some(batch) = stream.next().await {
        let batch = batch?;
        sink.consume_batch(batch)?;
    }

    let read_elapsed = read_start.elapsed();
    let job_elapsed = job_start.elapsed();
    println!(
        "total: {: >8.4}ms (read: {: >8.4}ms)",
        job_elapsed.as_secs_f32() * 1_000.0,
        read_elapsed.as_secs_f32() * 1_000.0
    );

    Ok(())
}

async fn build_client(addr: String) -> Result<FlightClient> {
    let channel = Channel::from_shared(addr)?.connect().await?;
    let mut client = FlightClient::new(channel);

    tracing::info!("Sending handshake to decoder...");
    let response = client.handshake("Host").await?;
    if response != *"Decoder" {
        bail!(
            "unexpected handshake response: '{}'",
            String::from_utf8_lossy(&response)
        )
    }
    tracing::info!("Handshake successful.");

    Ok(client)
}

async fn build_flight_data(file_path: &Path, start_tuple: usize, tuple_count: usize) -> Result<FlightDataEncoder> {
    let params = {
        let mut params = [0_u8; 32];
        params[..8].copy_from_slice(&start_tuple.to_le_bytes());
        params[8..16].copy_from_slice(&tuple_count.to_le_bytes());
        params[16..24].copy_from_slice(&BinaryStream::BATCH_SIZE.to_le_bytes());
        params[24..].copy_from_slice(&2_000_000_usize.to_le_bytes());
        params
    };
    let file = fs::File::open(file_path).await?;
    let stream = stream_binary_data(file).await?;
    Ok(FlightDataEncoderBuilder::new()
        .with_metadata(Bytes::from_owner(params))
        .with_schema(binary_schema().into())
        .build(stream))
}

async fn stream_binary_data(mut file: fs::File) -> Result<impl Stream<Item = Result<RecordBatch, FlightError>>> {
    let mut data = vec![];
    file.read_to_end(&mut data).await?;
    tracing::info!("Total file len: {}", data.len());

    Ok(BinaryStream::new(data))
}

fn binary_schema() -> arrow::datatypes::Schema {
    let mut builder = arrow::datatypes::SchemaBuilder::new();
    builder.push(arrow::datatypes::Field::new(
        "data",
        arrow::datatypes::DataType::LargeBinary,
        false,
    ));
    builder.finish()
}

fn expected_schema() -> arrow::datatypes::Schema {
    let mut builder = arrow::datatypes::SchemaBuilder::new();
    builder.push(arrow::datatypes::Field::new(
        "psc_key",
        arrow::datatypes::DataType::Int32,
        false,
    ));
    builder.finish()
}

struct BinaryStream {
    data: Buffer,
    idx: usize,
}

impl BinaryStream {
    const BATCH_SIZE: usize = 2_000_000;

    fn new(data: Vec<u8>) -> Self {
        let buffer = Buffer::from_vec(data);
        Self { data: buffer, idx: 0 }
    }
}

impl Stream for BinaryStream {
    type Item = Result<RecordBatch, FlightError>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        std::task::Poll::Ready({
            let batch_size = cmp::min(self.data.len() - self.idx, Self::BATCH_SIZE);
            let row = &self.data.slice_with_length(self.idx, batch_size);
            unsafe {
                *self.map_unchecked_mut(|x| &mut x.idx) += batch_size;
            };
            if row.is_empty() {
                None
            } else {
                let offsets = OffsetBuffer::from_lengths([batch_size]);
                let array = LargeBinaryArray::new(offsets, row.clone(), None);
                let batch = RecordBatch::try_new(binary_schema().into(), vec![Arc::new(array)])?;
                Some(Ok(batch))
            }
        })
    }
}
