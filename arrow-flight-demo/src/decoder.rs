use anyhow::Result;
use arrow::{
    array::{ArrayBuilder, AsArray, PrimitiveBuilder, RecordBatch},
    buffer::Buffer,
    datatypes::Int32Type,
    error::ArrowError,
};
use arrow_flight::{
    decode::{DecodedPayload, FlightDataDecoder},
    encode::{FlightDataEncoder, FlightDataEncoderBuilder},
    error::FlightError,
    flight_service_server::{FlightService, FlightServiceServer},
    Action, ActionType, Criteria, Empty, FlightData, FlightDescriptor, FlightInfo, HandshakeRequest, HandshakeResponse,
    PollInfo, PutResult, SchemaResult, Ticket,
};
use bytes::Bytes;
use clap::Parser;
use futures::{Stream, StreamExt};
use std::{
    cmp,
    marker::PhantomData,
    sync::Arc,
    time::{Duration, Instant},
};
use tonic::Streaming;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Parser)]
struct Args {
    #[clap(short = 'p', long)]
    port: String,
}

#[derive(Debug, Default)]
pub struct DecoderService {}

#[tonic::async_trait]
impl FlightService for DecoderService {
    type HandshakeStream = HandshakeStream;
    type ListFlightsStream = NotSupportedStream<FlightInfo>;
    type DoGetStream = NotSupportedStream<FlightData>;
    type DoPutStream = NotSupportedStream<PutResult>;
    type DoExchangeStream = DoExchangeStream;
    type DoActionStream = NotSupportedStream<arrow_flight::Result>;
    type ListActionsStream = NotSupportedStream<ActionType>;

    async fn handshake(
        &self,
        request: Request<Streaming<HandshakeRequest>>,
    ) -> Result<Response<Self::HandshakeStream>, Status> {
        let mut stream = request.into_inner();
        let handshake = stream.message().await?;
        match handshake {
            Some(h) => {
                if h.payload == *"Host" {
                    Ok(Response::new(HandshakeStream::default()))
                } else {
                    Err(Status::invalid_argument("invalid handshake"))
                }
            }
            None => Err(Status::invalid_argument("empty handshake not expected")),
        }
    }

    async fn list_flights(&self, _request: Request<Criteria>) -> Result<Response<Self::ListFlightsStream>, Status> {
        Err(Status::unimplemented("operation not supported"))
    }
    async fn get_flight_info(&self, _request: Request<FlightDescriptor>) -> Result<Response<FlightInfo>, Status> {
        Err(Status::unimplemented("operation not supported"))
    }
    async fn poll_flight_info(&self, _request: Request<FlightDescriptor>) -> Result<Response<PollInfo>, Status> {
        Err(Status::unimplemented("operation not supported"))
    }
    async fn get_schema(&self, _request: Request<FlightDescriptor>) -> Result<Response<SchemaResult>, Status> {
        Err(Status::unimplemented("operation not supported"))
    }
    async fn do_get(&self, _request: Request<Ticket>) -> Result<Response<Self::DoGetStream>, Status> {
        Err(Status::unimplemented("operation not supported"))
    }
    async fn do_put(&self, _request: Request<Streaming<FlightData>>) -> Result<Response<Self::DoPutStream>, Status> {
        Err(Status::unimplemented("operation not supported"))
    }
    async fn do_exchange(
        &self,
        request: Request<Streaming<FlightData>>,
    ) -> Result<Response<Self::DoExchangeStream>, Status> {
        let stream = request.into_inner();
        Ok(Response::new(DoExchangeStream::new(stream).await?))
    }
    async fn do_action(&self, _request: Request<Action>) -> Result<Response<Self::DoActionStream>, Status> {
        Err(Status::unimplemented("operation not supported"))
    }
    async fn list_actions(&self, _request: Request<Empty>) -> Result<Response<Self::ListActionsStream>, Status> {
        Err(Status::unimplemented("operation not supported"))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    arrow_flight_demo::init_logging();
    let args = Args::parse();

    let addr = format!("0.0.0.0:{}", args.port).parse()?;
    let decoder = DecoderService::default();

    tracing::info!("DUPA!");
    Server::builder()
        .add_service(FlightServiceServer::new(decoder))
        .serve(addr)
        .await?;

    Ok(())
}

pub struct DoExchangeStream {
    inner: FlightDataEncoder,
}
struct ReeDecoder {
    buffers: BufferView,
    rid: usize,
    last_rid: usize,
    run_idx: usize,
    output_batch_size: usize,
    sook: bool,
    timing: Duration,
}

struct BufferView {
    buffers: Vec<Buffer>,
    len: usize,
    one_buf_len: usize,
}

struct ReeDecoderStream {
    decoder: ReeDecoder,
    input: FlightDataDecoder,
}

impl DoExchangeStream {
    async fn new(stream: Streaming<FlightData>) -> Result<Self, Status> {
        let mut input = FlightDataDecoder::new(stream.map(|x| x.map_err(FlightError::Tonic)));
        match input.next().await {
            Some(Ok(data)) => match data.payload {
                DecodedPayload::None => Err(Status::invalid_argument("unexpected empty Flight data")),
                DecodedPayload::Schema(_) => {
                    let metadata = data.app_metadata();
                    if metadata.len() != 32 {
                        Err(Status::invalid_argument("invalid request metadata"))
                    } else {
                        let start_tuple = u64::from_le_bytes(metadata[..8].try_into().unwrap());
                        let tuple_count = u64::from_le_bytes(metadata[8..16].try_into().unwrap());
                        let input_batch_size = u64::from_le_bytes(metadata[16..24].try_into().unwrap());
                        let output_batch_size = u64::from_le_bytes(metadata[24..].try_into().unwrap());
                        tracing::info!(
                            "Starting an exchange: start_tuple={}, tuple_count={}, in_batch={}, out_batch={}",
                            start_tuple,
                            tuple_count,
                            input_batch_size,
                            output_batch_size,
                        );
                        let encoder = FlightDataEncoderBuilder::new()
                            .with_schema(output_schema().into())
                            .build(ReeDecoderStream::new(
                                input,
                                start_tuple,
                                tuple_count,
                                input_batch_size,
                                output_batch_size,
                            ));
                        Ok(Self { inner: encoder })
                    }
                }
                DecodedPayload::RecordBatch(_) => {
                    Err(Status::invalid_argument("unexpected Flight batch without Schema first"))
                }
            },
            Some(Err(err)) => Err(Status::invalid_argument(format!("Flight error: {}", err))),
            None => Err(Status::invalid_argument("unexpected end of Flight stream")),
        }
    }
}

impl ReeDecoderStream {
    pub fn new(
        input: FlightDataDecoder,
        start_tuple: u64,
        tuple_count: u64,
        input_batch_size: u64,
        output_batch_size: u64,
    ) -> Self {
        Self {
            input,
            decoder: ReeDecoder::new(start_tuple, tuple_count, input_batch_size, output_batch_size),
        }
    }
}

impl ReeDecoder {
    pub fn new(start_tuple: u64, tuple_count: u64, input_batch_size: u64, output_batch_size: u64) -> Self {
        Self {
            buffers: BufferView::new(input_batch_size as usize),
            rid: start_tuple as usize,
            run_idx: 0,
            last_rid: (start_tuple + tuple_count) as usize - 1,
            output_batch_size: output_batch_size as usize,
            sook: false,
            timing: Duration::ZERO,
        }
    }

    fn is_finished(&self) -> bool {
        self.rid > self.last_rid
    }

    fn produce_batch(&mut self) -> Result<RecordBatch, ArrowError> {
        assert!(!self.is_finished());
        if !self.sook {
            self.seek();
        }
        let mut output_array: PrimitiveBuilder<Int32Type> = PrimitiveBuilder::with_capacity(self.output_batch_size / 4);
        let start = Instant::now();
        let mut rem_tuples = cmp::min(self.last_rid - self.rid + 1, self.output_batch_size / 4);
        tracing::info!("Trying to provide {} tuples", rem_tuples);

        while rem_tuples > 0 {
            tracing::debug!("rid: {}", self.rid);
            let run_len = self.buffers.run_len_at(self.run_idx) - self.rid as u32;
            let elem = self.buffers.value_at(self.run_idx);
            tracing::debug!("run_len: {run_len}, elem: {elem}");
            if run_len as usize >= rem_tuples {
                for _ in 0..rem_tuples {
                    output_array.append_value(elem);
                }
                self.rid += rem_tuples;
                break;
            } else {
                for _ in 0..run_len {
                    output_array.append_value(elem);
                }
                self.rid += run_len as usize;
                self.run_idx += 1;
                rem_tuples -= run_len as usize;
            }
        }

        tracing::info!("Returning batch of {}", output_array.len());
        let output_array = output_array.finish();
        self.timing += start.elapsed();
        RecordBatch::try_new(Arc::new(output_schema()), vec![Arc::new(output_array)])
    }

    fn consume_input_batch(&mut self, batch: RecordBatch) -> Result<(), ArrowError> {
        if batch.num_columns() != 1 || batch.num_rows() != 1 {
            return Err(ArrowError::InvalidArgumentError(
                "invalid shape of binary input".to_string(),
            ));
        }
        let col = batch.column(0).as_binary::<i64>();
        let len = col.value_length(0);
        tracing::info!("payload len: {}", len);
        let buf = col.values();
        self.buffers.push(buf.slice_with_length(0, len as usize).clone());
        tracing::debug!("total len: {}", self.buffers.len());

        Ok(())
    }

    fn seek(&mut self) {
        let mut start = 0;
        let mut end = self.buffers.len() / 8;

        while start < end {
            let mid = (start + end) / 2;
            tracing::debug!("Seek: [{}..{}..{}]", start, mid, end);
            let last_tuple_in_run = self.buffers.run_len_at(mid);
            if last_tuple_in_run > self.rid as u32 {
                end = mid;
            } else {
                start = mid + 1;
            }
        }

        tracing::debug!("Sook: {}", start);
        self.run_idx = start;
        self.sook = true;
    }
}

impl BufferView {
    pub fn new(buf_size: usize) -> Self {
        BufferView {
            buffers: vec![],
            len: 0,
            one_buf_len: buf_size,
        }
    }

    pub fn push(&mut self, buffer: Buffer) {
        self.len += buffer.len();
        self.buffers.push(buffer);
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn run_len_at(&self, index: usize) -> u32 {
        let byte_idx = index * 4;
        let buf_idx = byte_idx / self.one_buf_len;
        let inner_idx = byte_idx % self.one_buf_len;
        unsafe {
            let ptr = self.buffers[buf_idx].as_ptr().add(inner_idx).cast::<u32>();
            *ptr
        }
    }

    pub fn value_at(&self, index: usize) -> i32 {
        let byte_idx = index * 4 + self.len() / 2;
        let buf_idx = byte_idx / self.one_buf_len;
        let inner_idx = byte_idx % self.one_buf_len;
        unsafe {
            let ptr = self.buffers[buf_idx].as_ptr().add(inner_idx).cast::<i32>();
            *ptr
        }
    }
}

impl Stream for ReeDecoderStream {
    type Item = Result<RecordBatch, FlightError>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        if self.decoder.is_finished() {
            tracing::info!(
                "Finished! Total duration: {: >8.4}ms",
                self.decoder.timing.as_secs_f32() * 1_000.0
            );
            return std::task::Poll::Ready(None);
        }
        let inner = unsafe { self.as_mut().map_unchecked_mut(|x| &mut x.input) };
        match inner.poll_next(cx) {
            std::task::Poll::Ready(data) => {
                let mut decoder = unsafe { self.as_mut().map_unchecked_mut(|x| &mut x.decoder) };
                match data {
                    Some(Ok(data)) => match data.payload {
                        DecodedPayload::None => {
                            tracing::info!("DecodedPayload::None ??");
                            cx.waker().wake_by_ref();
                            std::task::Poll::Pending
                        }
                        DecodedPayload::Schema(_) => {
                            tracing::info!("DecodedPayload::Schema, skipping message");
                            cx.waker().wake_by_ref();
                            std::task::Poll::Pending
                        }
                        DecodedPayload::RecordBatch(record_batch) => {
                            tracing::info!("Received batch");
                            cx.waker().wake_by_ref();
                            match decoder.consume_input_batch(record_batch) {
                                Ok(_) => std::task::Poll::Pending,
                                Err(err) => std::task::Poll::Ready(Some(Err(FlightError::Arrow(err)))),
                            }
                        }
                    },
                    Some(Err(err)) => std::task::Poll::Ready(Some(Err(err))),
                    None => {
                        tracing::info!("Produce batch.");
                        match self.decoder.produce_batch() {
                            Ok(batch) => {
                                tracing::info!("Produced {}", batch.num_rows());
                                std::task::Poll::Ready(Some(Ok(batch)))
                            }
                            Err(err) => std::task::Poll::Ready(Some(Err(FlightError::Arrow(err)))),
                        }
                    }
                }
            }
            std::task::Poll::Pending => {
                tracing::debug!("Waiting for more data...");
                std::task::Poll::Pending
            }
        }
    }
}

impl Stream for DoExchangeStream {
    type Item = Result<FlightData, Status>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        unsafe {
            self.map_unchecked_mut(|x| &mut x.inner)
                .poll_next_unpin(cx)
                .map_err(|x| Status::internal(x.to_string()))
        }
    }
}

fn output_schema() -> arrow::datatypes::Schema {
    let mut schema = arrow::datatypes::SchemaBuilder::new();
    schema.push(arrow::datatypes::Field::new(
        "psc_key",
        arrow::datatypes::DataType::Int32,
        false,
    ));
    schema.finish()
}

#[derive(Default)]
pub struct HandshakeStream {
    finished: bool,
}

impl Stream for HandshakeStream {
    type Item = Result<HandshakeResponse, Status>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        if self.finished {
            std::task::Poll::Ready(None)
        } else {
            let response = HandshakeResponse {
                protocol_version: 1,
                payload: Bytes::from("Decoder"),
            };
            unsafe { *self.map_unchecked_mut(|x| &mut x.finished) = true }
            std::task::Poll::Ready(Some(Ok(response)))
        }
    }
}

pub struct NotSupportedStream<T> {
    _empty: Never,
    phantom: PhantomData<T>,
}
enum Never {}

impl<T> Stream for NotSupportedStream<T> {
    type Item = Result<T, Status>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        std::task::Poll::Ready(Some(Err(Status::unimplemented("operation not supported"))))
    }
}
