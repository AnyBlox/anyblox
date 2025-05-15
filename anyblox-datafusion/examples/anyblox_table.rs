use anyblox::config::LogLevel;
use anyhow::Result;
use clap::Parser;
use datafusion::catalog::Session;
use datafusion::execution::context::SessionContext as DataFusionSessionContext;
use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Arc;
use anyblox_datafusion::AnyBloxTable;

#[derive(Debug, Parser, Clone)]
#[command(flatten_help = true)]
pub struct Args {
    /// Name of the table to bind to the file, accessible from SQL.
    table_name: String,
    /// SQL query to execute.
    sql: String,
    /// Path to the AnyBlox file.
    anyblox_path: PathBuf,
    /// Optional path to the dataset - required if the AnyBlox file is an Extension.
    dataset_path: Option<PathBuf>,
    #[clap(short, long, required = false)]
    batch_size: Option<usize>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let tokio_rt = tokio::runtime::Builder::new_current_thread().build()?;
    let config = {
        let mut builder = anyblox::config::ConfigBuilder::new();
        builder
            .enable_opentelemetry(false)
            .compile_with_debug(false)
            .set_log_level(LogLevel::Info)
            .set_thread_virtual_memory_limit(32 * 1024 * 1024 * 1024)
            .set_wasm_cache_limit(256 * 1024 * 1024);
        builder.into_config()
    };
    let anyblox_rt = Arc::new(anyblox::build_engine(config)?);

    tokio_rt.block_on(async move {
        let config = { datafusion::execution::config::SessionConfig::new().with_target_partitions(1) };
        let ctx = DataFusionSessionContext::new_with_config(config);

        let anyblox_table = AnyBloxTable::new_wasm(anyblox_rt.clone(), args.anyblox_path, args.dataset_path).await?;
        ctx.register_table(&args.table_name, Arc::new(anyblox_table))?;

        let df = ctx.sql(&args.sql).await?;
        df.show().await?;

        Ok(())
    })
}
