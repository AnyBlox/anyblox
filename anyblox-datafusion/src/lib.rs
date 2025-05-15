use anyblox::bundle::AnyBloxBundle;
use anyblox::{AnyBloxJob, AnyBloxJobParametersBuilder, AnyBloxRuntime, ColumnProjection, NativeJob};
use arrow::{datatypes::Schema, record_batch::RecordBatch};
use async_trait::async_trait;
use datafusion::arrow::datatypes::SchemaRef;
use datafusion::catalog::Session;
use datafusion::common::stats::Precision;
use datafusion::common::Statistics;
use datafusion::datasource::{TableProvider, TableType};
use datafusion::error::DataFusionError;
use datafusion::execution::{SendableRecordBatchStream, TaskContext};
use datafusion::logical_expr::{Expr, UserDefinedLogicalNode};
use datafusion::physical_expr::{EquivalenceProperties, Partitioning};
use datafusion::physical_plan::metrics;
use datafusion::physical_plan::metrics::{ExecutionPlanMetricsSet, MetricBuilder, MetricsSet};
use datafusion::physical_plan::stream::RecordBatchStreamAdapter;
use datafusion::physical_plan::{DisplayAs, DisplayFormatType, ExecutionMode, ExecutionPlan, PlanProperties};
use futures::Stream;
use memmap2::{Mmap, MmapOptions};
use std::any::Any;
use std::fmt::{Debug, Formatter};
use std::os::fd::AsRawFd;
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::thread::ThreadId;
use std::time::Instant;
use tokio::fs;

const DEFAULT_BATCH_SIZE: u64 = 122880;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum JobType {
    Wasm,
    Native,
}

pub struct AnyBloxTable {
    anyblox_runtime: Arc<AnyBloxRuntime>,
    bundle: Arc<AnyBloxBundle>,
    schema: Arc<Schema>,
    job_type: JobType,
}

impl Debug for AnyBloxTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AnyBloxTable")
    }
}

impl AnyBloxTable {
    pub async fn new_wasm(
        runtime: Arc<AnyBloxRuntime>,
        bundle_path: impl AsRef<Path>,
        data_path: Option<impl AsRef<Path>>,
    ) -> datafusion::common::Result<Self> {
        Self::new(runtime, bundle_path, data_path, JobType::Wasm).await
    }

    pub async fn new_native(
        runtime: Arc<AnyBloxRuntime>,
        bundle_path: impl AsRef<Path>,
        data_path: Option<impl AsRef<Path>>,
    ) -> datafusion::common::Result<Self> {
        Self::new(runtime, bundle_path, data_path, JobType::Native).await
    }

    async fn new(
        runtime: Arc<AnyBloxRuntime>,
        bundle_path: impl AsRef<Path>,
        data_path: Option<impl AsRef<Path>>,
        job_type: JobType,
    ) -> datafusion::common::Result<Self> {
        let bundle = open_anyblox_bundle(bundle_path, data_path)
            .await
            .map_err(|e| DataFusionError::External(e))?;
        let schema: Schema = bundle.metadata().schema().into();

        Ok(Self {
            anyblox_runtime: runtime,
            bundle: Arc::new(bundle),
            schema: Arc::new(schema),
            job_type,
        })
    }

    pub fn name(&self) -> &str {
        self.bundle.metadata().data().name()
    }
}

#[async_trait]
impl TableProvider for AnyBloxTable {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn schema(&self) -> SchemaRef {
        self.schema.clone()
    }

    fn table_type(&self) -> TableType {
        TableType::Base
    }

    async fn scan(
        &self,
        _state: &dyn Session,
        projection: Option<&Vec<usize>>,
        _filters: &[Expr],
        limit: Option<usize>,
    ) -> datafusion::common::Result<Arc<dyn ExecutionPlan>> {
        let anyblox_projection = if let Some(proj) = projection {
            ColumnProjection::from_indices(proj.iter().copied())
        } else {
            ColumnProjection::all(self.schema.fields.len())
        }
        .map_err(|e| DataFusionError::External(Box::new(e)))?;

        Ok(Arc::new(AnyBloxPhysicalPlan::new(
            self.anyblox_runtime.clone(),
            self.bundle.clone(),
            self.schema.clone(),
            anyblox_projection,
            limit.unwrap_or(self.bundle.metadata().data().count() as usize),
            self.job_type,
        )?))
    }

    fn statistics(&self) -> Option<Statistics> {
        Some(Statistics {
            num_rows: Precision::Exact(usize::try_from(self.bundle.metadata().data().count()).unwrap()),
            total_byte_size: self
                .bundle
                .metadata()
                .data()
                .size_in_bytes()
                .map_or(Precision::Absent, |v| Precision::Exact(usize::try_from(v).unwrap())),
            column_statistics: Statistics::unknown_column(self.schema.as_ref()),
        })
    }
}

struct AnyBloxPhysicalPlan {
    anyblox_runtime: Arc<AnyBloxRuntime>,
    anyblox_bundle: Arc<AnyBloxBundle>,
    properties: PlanProperties,
    schema: Arc<Schema>,
    column_projection: ColumnProjection,
    limit: usize,
    job_type: JobType,
    metrics: ExecutionPlanMetricsSet,
}

impl Debug for AnyBloxPhysicalPlan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AnybloxPhysicalPlan")
    }
}

impl DisplayAs for AnyBloxPhysicalPlan {
    fn fmt_as(&self, _t: DisplayFormatType, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl AnyBloxPhysicalPlan {
    pub fn new(
        runtime: Arc<AnyBloxRuntime>,
        bundle: Arc<AnyBloxBundle>,
        schema: Arc<Schema>,
        column_projection: ColumnProjection,
        limit: usize,
        job_type: JobType,
    ) -> datafusion::common::Result<Self> {
        let properties = PlanProperties::new(
            EquivalenceProperties::new(Arc::new(
                column_projection
                    .project_schema(schema.as_ref())
                    .map_err(|e| DataFusionError::ArrowError(e, None))?,
            )),
            Partitioning::UnknownPartitioning(1),
            ExecutionMode::Bounded,
        );
        let mut metrics = ExecutionPlanMetricsSet::new();

        Ok(Self {
            anyblox_runtime: runtime,
            anyblox_bundle: bundle,
            properties,
            schema,
            column_projection,
            limit,
            job_type,
            metrics,
        })
    }
}

impl ExecutionPlan for AnyBloxPhysicalPlan {
    fn name(&self) -> &str {
        "AnybloxScan"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn properties(&self) -> &PlanProperties {
        &self.properties
    }

    fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>> {
        vec![]
    }

    fn with_new_children(
        self: Arc<Self>,
        _children: Vec<Arc<dyn ExecutionPlan>>,
    ) -> datafusion::common::Result<Arc<dyn ExecutionPlan>> {
        Ok(self)
    }

    fn execute(
        &self,
        partition: usize,
        _context: Arc<TaskContext>,
    ) -> datafusion::common::Result<SendableRecordBatchStream> {
        let batch_size = {
            let min_batch_size = self.anyblox_bundle.metadata().decoder().min_batch_size().unwrap_or(1);
            let times = DEFAULT_BATCH_SIZE.div_ceil(min_batch_size);
            min_batch_size * times
        };
        let stream_metrics = StreamMetrics::new(&self.metrics, partition);
        let creation_start = Instant::now();

        match self.job_type {
            JobType::Wasm => {
                let params = AnyBloxJobParametersBuilder::new()
                    .with_column_projection(self.column_projection.clone())
                    .finish(&self.anyblox_bundle)
                    .map_err(|e| DataFusionError::External(Box::new(e)))?;
                let job = self
                    .anyblox_runtime
                    .init_blocking_job(params)
                    .map_err(|e| DataFusionError::External(Box::new(e)))?;
                let schema = job.schema();
                let stream = AnyBloxBatchStream::new(
                    self.anyblox_runtime.clone(),
                    job,
                    batch_size as usize,
                    self.limit,
                    stream_metrics.clone(),
                );
                let adapted = RecordBatchStreamAdapter::new(schema, stream);
                stream_metrics.elapsed_compute.add_elapsed(creation_start);
                Ok(Box::pin(adapted))
            }
            JobType::Native => {
                let job = self
                    .anyblox_runtime
                    .init_native_job_with_projection(
                        self.anyblox_bundle.metadata().decoder().uri(),
                        self.schema.as_ref(),
                        self.column_projection,
                        true,
                    )
                    .map_err(|e| DataFusionError::External(Box::new(e)))?;
                let schema = job.schema();
                let mmap = unsafe {
                    MmapOptions::new()
                        .offset(u64::try_from(self.anyblox_bundle.dataset_offset()).unwrap())
                        .map(self.anyblox_bundle.dataset_fd().as_raw_fd())
                }
                .map_err(DataFusionError::IoError)?;
                let stream = AnyBloxNativeBatchStream::new(
                    self.anyblox_runtime.clone(),
                    mmap,
                    job,
                    batch_size as usize,
                    self.limit,
                    stream_metrics.clone(),
                );
                let adapted = RecordBatchStreamAdapter::new(schema, stream);
                stream_metrics.elapsed_compute.add_elapsed(creation_start);
                Ok(Box::pin(adapted))
            }
        }
    }

    fn metrics(&self) -> Option<MetricsSet> {
        Some(self.metrics.clone_inner())
    }

    fn statistics(&self) -> datafusion::common::Result<Statistics> {
        Ok(Statistics {
            num_rows: Precision::Exact(self.limit),
            total_byte_size: self
                .anyblox_bundle
                .metadata()
                .data()
                .size_in_bytes()
                .map_or(Precision::Absent, |v| {
                    Precision::Inexact(
                        usize::try_from(
                            u128::from(v) * u128::try_from(self.limit).unwrap()
                                / u128::from(self.anyblox_bundle.metadata().data().count()),
                        )
                        .unwrap(),
                    )
                }),
            column_statistics: Statistics::unknown_column(self.schema.as_ref()),
        })
    }

    fn supports_limit_pushdown(&self) -> bool {
        true
    }

    fn fetch(&self) -> Option<usize> {
        Some(self.limit)
    }
}

struct AnyBloxBatchStream {
    job: AnyBloxJob,
    thread_id: ThreadId,
    next_tuple: usize,
    batch_size: usize,
    remaining_tuples: usize,
    runtime: Arc<AnyBloxRuntime>,
    metrics: StreamMetrics,
}

struct AnyBloxNativeBatchStream {
    job: NativeJob,
    mmap: Mmap,
    thread_id: ThreadId,
    next_tuple: usize,
    batch_size: usize,
    remaining_tuples: usize,
    runtime: Arc<AnyBloxRuntime>,
    metrics: StreamMetrics,
}

impl AnyBloxBatchStream {
    fn new(
        runtime: Arc<AnyBloxRuntime>,
        job: AnyBloxJob,
        batch_size: usize,
        tuple_count: usize,
        metrics: StreamMetrics,
    ) -> Self {
        Self {
            job,
            thread_id: std::thread::current().id(),
            next_tuple: 0,
            runtime,
            batch_size,
            remaining_tuples: tuple_count,
            metrics,
        }
    }
}

impl AnyBloxNativeBatchStream {
    fn new(
        runtime: Arc<AnyBloxRuntime>,
        mmap: Mmap,
        job: NativeJob,
        batch_size: usize,
        tuple_count: usize,
        metrics: StreamMetrics,
    ) -> Self {
        Self {
            job,
            mmap,
            thread_id: std::thread::current().id(),
            next_tuple: 0,
            runtime,
            batch_size,
            remaining_tuples: tuple_count,
            metrics,
        }
    }
}

// SAFETY: We assert the ThreadID never changes.
// This works only for a single-threaded runtime, but that's all we need for now.
unsafe impl Send for AnyBloxBatchStream {}

impl Stream for AnyBloxBatchStream {
    type Item = Result<RecordBatch, DataFusionError>;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let start = std::time::Instant::now();
        assert_eq!(self.thread_id, std::thread::current().id());
        if self.remaining_tuples == 0 {
            return Poll::Ready(None);
        }
        let runtime = self.runtime.clone();
        let next_tuple = self.next_tuple;
        let batch_size = self.batch_size;
        let request = batch_size.min(self.remaining_tuples);

        let result = runtime
            .run_blocking_job(&mut self.job, next_tuple, request)
            .map_err(|e| DataFusionError::External(Box::new(e)));
        let ret = match result {
            Ok(batch) if batch.row_count() == 0 => Poll::Ready(None),
            Ok(batch) => {
                self.next_tuple += batch.row_count();
                self.remaining_tuples -= batch.row_count();
                self.metrics.output_rows.add(batch.row_count());
                let arrow_batch = batch
                    .into_arrow_record_batch(self.job.schema())
                    .map_err(|e| DataFusionError::External(Box::new(e)));
                Poll::Ready(Some(arrow_batch))
            }
            Err(e) => Poll::Ready(Some(Err(e))),
        };
        self.metrics.elapsed_compute.add_elapsed(start);
        ret
    }
}

impl Stream for AnyBloxNativeBatchStream {
    type Item = Result<RecordBatch, DataFusionError>;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let start = std::time::Instant::now();
        assert_eq!(self.thread_id, std::thread::current().id());
        if self.remaining_tuples == 0 {
            return Poll::Ready(None);
        }
        let runtime = self.runtime.clone();
        let next_tuple = self.next_tuple;
        let batch_size = self.batch_size;
        let request = batch_size.min(self.remaining_tuples);
        let s = &mut *self;
        let data: &[u8] = &s.mmap;
        let job = &mut s.job;

        let result = runtime
            .run_native_job(job, data, next_tuple, request)
            .map_err(|e| DataFusionError::External(Box::new(e)));
        let ret = match result {
            Ok(batch) if batch.row_count() == 0 => Poll::Ready(None),
            Ok(batch) => {
                self.next_tuple += batch.row_count();
                self.remaining_tuples -= batch.row_count();
                self.metrics.output_rows.add(batch.row_count());
                let arrow_batch = batch
                    .into_arrow_record_batch(self.job.schema())
                    .map_err(|e| DataFusionError::External(Box::new(e)));
                Poll::Ready(Some(arrow_batch))
            }
            Err(e) => Poll::Ready(Some(Err(e))),
        };
        self.metrics.elapsed_compute.add_elapsed(start);
        ret
    }
}

#[derive(Clone)]
struct StreamMetrics {
    output_rows: metrics::Count,
    elapsed_compute: metrics::Time,
}

impl StreamMetrics {
    fn new(metric_set: &ExecutionPlanMetricsSet, partition: usize) -> Self {
        let output_rows = MetricBuilder::new(metric_set).output_rows(partition);
        let elapsed_compute = MetricBuilder::new(metric_set).elapsed_compute(partition);
        Self {
            output_rows,
            elapsed_compute,
        }
    }

    fn output_rows(&self) -> &metrics::Count {
        &self.output_rows
    }

    fn elapsed_compute(&self) -> &metrics::Time {
        &self.elapsed_compute
    }
}

async fn open_anyblox_bundle(
    anyblox_path: impl AsRef<Path>,
    dataset_path: Option<impl AsRef<Path>>,
) -> Result<AnyBloxBundle, Box<dyn std::error::Error + Send + Sync>> {
    let bundle_file = fs::File::open(anyblox_path).await?;
    let bundle_len = bundle_file.metadata().await?.len() as usize;
    let dataset_file = if let Some(path) = dataset_path {
        let file = fs::File::open(path).await?;
        let len = file.metadata().await?.len();
        Some((file, len as usize))
    } else {
        None
    };
    let anyblox_bundle = if let Some((dataset_file, dataset_len)) = dataset_file {
        AnyBloxBundle::new_extension(bundle_file, bundle_len, dataset_file, dataset_len)?
    } else {
        AnyBloxBundle::new_self_contained(bundle_file, bundle_len)?
    };
    Ok(anyblox_bundle)
}
