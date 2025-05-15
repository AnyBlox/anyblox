use anyblox::config::LogLevel;
use anyblox::AnyBloxRuntime;
use anyblox_datafusion::AnyBloxTable;
use anyhow::Result;
use arrow::array::{Array, AsArray};
use arrow::record_batch::RecordBatch;
use clap::{Parser, ValueEnum};
use datafusion::catalog::Session;
use datafusion::execution::context::SessionContext as DataFusionSessionContext;
use parse_duration::parse as parse_duration;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use tokio::io::AsyncWriteExt;

const WARMUP_RUNS: usize = 2;
const SAMPLES: usize = 3;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ExpType {
    AnyBlox,
    Native,
    Parquet,
}

#[derive(Debug, Parser, Clone)]
#[command(flatten_help = true)]
pub struct Args {
    /// Path to the AnyBlox directory.
    anyblox_dir: PathBuf,
    /// Path to the parquet directory.
    parquet_dir: PathBuf,
    /// Report output path.
    out_path: PathBuf,
    #[clap(long, required = false, default_value = "any-blox")]
    exp: ExpType,
    #[clap(short, long, required = false)]
    batch_size: Option<usize>,
    #[clap(short, long, required = false)]
    query: Option<String>,
    #[clap(short, long, required = false)]
    exclude: Vec<String>,
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

        match args.exp {
            ExpType::AnyBlox => add_anyblox_tables(&ctx, args.anyblox_dir, anyblox_rt.clone(), false).await?,
            ExpType::Native => add_anyblox_tables(&ctx, args.anyblox_dir, anyblox_rt.clone(), true).await?,
            ExpType::Parquet => add_parquet_hits_tables(&ctx, args.parquet_dir.clone()).await?,
        }
        add_parquet_tables(&ctx, args.parquet_dir).await?;
        create_views(&ctx).await?;

        let mut report = Report::new();

        for (q_name, q_sql) in queries().iter() {
            if let Some(q) = args.query.as_ref() {
                if q != q_name {
                    continue;
                }
            }
            if args.exclude.contains(&q_name.to_string()) {
                continue;
            }
            println!("{}", q_name);
            // Warmup
            for _ in 0..WARMUP_RUNS {
                let df = ctx.sql(q_sql).await?.explain(true, true)?;
                let _ = df.collect().await?;
            }

            for _ in 0..SAMPLES {
                let df = ctx.sql(q_sql).await?.explain(true, true)?;
                let results = df.collect().await?;
                report.add_explain_results(q_name.to_string(), results, args.exp);
            }
        }

        let mut file = tokio::fs::File::create(&args.out_path).await?;
        file.write_all(report.to_string().as_bytes()).await?;

        Ok(())
    })
}

async fn add_anyblox_tables(
    ctx: &DataFusionSessionContext,
    dir_path: PathBuf,
    anyblox_rt: Arc<AnyBloxRuntime>,
    native: bool,
) -> Result<()> {
    for i in 0..=3 {
        let file_path = dir_path.join(format!("hits-strings-{i}")).with_extension("any");
        let anyblox_table = if native {
            AnyBloxTable::new_native(anyblox_rt.clone(), file_path, None::<PathBuf>).await?
        } else {
            AnyBloxTable::new_wasm(anyblox_rt.clone(), file_path, None::<PathBuf>).await?
        };
        ctx.register_table(format!("hits_strings_{i}"), Arc::new(anyblox_table))?;
    }

    Ok(())
}

async fn add_parquet_hits_tables(ctx: &DataFusionSessionContext, dir_path: PathBuf) -> Result<()> {
    for i in 0..=3 {
        let file_path = dir_path.join(format!("hits-strings-{i}")).with_extension("parquet");
        ctx.register_parquet(
            &format!("hits_strings_{i}"),
            file_path.to_str().unwrap(),
            Default::default(),
        )
        .await?;
    }

    Ok(())
}

async fn add_parquet_tables(ctx: &DataFusionSessionContext, dir_path: PathBuf) -> Result<()> {
    let file_path = dir_path.join("hits-base").with_extension("parquet");
    ctx.register_parquet("hits_base", file_path.to_str().unwrap(), Default::default())
        .await?;

    Ok(())
}

#[derive(serde::Serialize)]
struct Report(HashMap<String, Vec<(f64, f64)>>);

impl Report {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn add_explain_results(&mut self, query_name: String, results: Vec<RecordBatch>, exp: ExpType) {
        assert_eq!(results.len(), 1);
        let results = &results[0];
        assert_eq!(results.num_rows(), 4);
        assert_eq!(results.num_columns(), 2);
        let runtime_string = results.column(1).as_string::<i32>().value(3);
        let total_runtime =
            parse_duration(runtime_string).expect(format!("valid time, got {}", runtime_string).as_str());

        let detailed_plan = results.column(1).as_string::<i32>().value(1);
        let op_time = match exp {
            ExpType::AnyBlox | ExpType::Native => sum_anyblox_time_from_explain_output(detailed_plan),
            ExpType::Parquet => sum_parquet_strings_from_explain_output(detailed_plan),
        };

        let result = (total_runtime.as_secs_f64(), op_time.as_secs_f64());
        match self.0.entry(query_name) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                entry.get_mut().push(result);
            }
            std::collections::hash_map::Entry::Vacant(mut entry) => {
                entry.insert(vec![result]);
            }
        }
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;
        write!(f, "{}", str)
    }
}

fn sum_anyblox_time_from_explain_output(plan: &str) -> Duration {
    static REGEX: OnceLock<regex::Regex> = OnceLock::new();
    let regex = REGEX.get_or_init(|| {
        Regex::new(r#"AnybloxPhysicalPlan, metrics=\[output_rows\{partition=[0-9]+}=([0-9]*), elapsed_compute\{partition=[0-9]+}=([^]]*)]"#).unwrap()
    });
    let mut duration = std::time::Duration::ZERO;
    for line in plan.lines() {
        let line = line.trim();
        if let Some(captures) = regex.captures(line) {
            let elapsed_string = captures.get(2).unwrap().as_str();
            let d = parse_duration(elapsed_string).expect(format!("valid time, got {}", elapsed_string).as_str());
            duration += d;
        }
    }

    duration
}

fn sum_parquet_strings_from_explain_output(plan: &str) -> Duration {
    static REGEX: OnceLock<regex::Regex> = OnceLock::new();
    static REGEX_TIME_ELAPSED_PROCESSING: OnceLock<Regex> = OnceLock::new();
    static REGEX_TIME_ELAPSED_OPENING: OnceLock<Regex> = OnceLock::new();
    let regex = REGEX.get_or_init(|| {
        Regex::new(r#"ParquetExec: file_groups=\{[^}]*/hits-strings-[0-3].parquet.*]]},.* metrics=\[([^]]*)]"#).unwrap()
    });
    let regex_time_elapsed_processing =
        REGEX_TIME_ELAPSED_PROCESSING.get_or_init(|| Regex::new(r#"time_elapsed_processing[^,]*=([^,]*),"#).unwrap());
    let regex_time_elapsed_opening =
        REGEX_TIME_ELAPSED_OPENING.get_or_init(|| Regex::new(r#"time_elapsed_opening[^,]*=([^,]*),"#).unwrap());
    let mut duration = std::time::Duration::ZERO;
    for line in plan.lines() {
        let line = line.trim();
        if let Some(captures) = regex.captures(line) {
            let metrics = captures.get(1).unwrap().as_str();
            let caps_time_elapsed_processing = regex_time_elapsed_processing.captures(metrics).unwrap();
            let caps_time_elapsed_opening = regex_time_elapsed_opening.captures(metrics).unwrap();
            let time_elapsed_processing_string = caps_time_elapsed_processing
                .get(1)
                .unwrap()
                .as_str()
                .replace("µs", "microseconds");
            let time_elapsed_opening_string = caps_time_elapsed_opening
                .get(1)
                .unwrap()
                .as_str()
                .replace("µs", "microseconds");
            let d = parse_duration(&time_elapsed_processing_string)
                .expect(format!("valid time, got {}", time_elapsed_processing_string).as_str())
                + parse_duration(&time_elapsed_opening_string)
                    .expect(format!("valid time, got {}", time_elapsed_opening_string).as_str());
            duration += d;
        }
    }

    duration
}

async fn create_views(ctx: &DataFusionSessionContext) -> Result<()> {
    ctx.sql(
        r#"
CREATE VIEW hits_strings AS (
  SELECT * FROM hits_strings_0
  UNION ALL
  SELECT * FROM hits_strings_1
  UNION ALL
  SELECT * FROM hits_strings_2
  UNION ALL
  SELECT * FROM hits_strings_3
);
        "#,
    )
    .await?;
    ctx.sql(
        r#"
CREATE VIEW hits AS (
  SELECT * FROM hits_base b
  JOIN hits_strings s ON b.rid = s.rid
);
        "#,
    )
    .await?;
    Ok(())
}

fn queries() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "6",
            r#"
SELECT COUNT(DISTINCT "SearchPhrase") FROM hits;
"#,
        ),
        (
            "11",
            r#"
SELECT "MobilePhoneModel", COUNT(DISTINCT "UserID") AS u FROM hits WHERE "MobilePhoneModel" <> '' GROUP BY "MobilePhoneModel" ORDER BY u DESC LIMIT 10;
"#,
        ),
        (
            "12",
            r#"
SELECT "MobilePhone", "MobilePhoneModel", COUNT(DISTINCT "UserID") AS u FROM hits WHERE "MobilePhoneModel" <> '' GROUP BY "MobilePhone", "MobilePhoneModel" ORDER BY u DESC LIMIT 10;
"#,
        ),
        (
            "13",
            r#"
SELECT "SearchPhrase", COUNT(*) AS c FROM hits WHERE "SearchPhrase" <> '' GROUP BY "SearchPhrase" ORDER BY c DESC LIMIT 10;
"#,
        ),
        (
            "14",
            r#"
SELECT "SearchPhrase", COUNT(DISTINCT "UserID") AS u FROM hits WHERE "SearchPhrase" <> '' GROUP BY "SearchPhrase" ORDER BY u DESC LIMIT 10;
"#,
        ),
        (
            "15",
            r#"
SELECT "SearchEngineID", "SearchPhrase", COUNT(*) AS c FROM hits WHERE "SearchPhrase" <> '' GROUP BY "SearchEngineID", "SearchPhrase" ORDER BY c DESC LIMIT 10;
"#,
        ),
        (
            "17",
            r#"
SELECT "UserID", "SearchPhrase", COUNT(*) FROM hits GROUP BY "UserID", "SearchPhrase" ORDER BY COUNT(*) DESC LIMIT 10;
"#,
        ),
        (
            "18",
            r#"
SELECT "UserID", "SearchPhrase", COUNT(*) FROM hits GROUP BY "UserID", "SearchPhrase" LIMIT 10;
"#,
        ),
        (
            "19",
            r#"
SELECT "UserID", extract(minute FROM "EventTime") AS m, "SearchPhrase", COUNT(*) FROM hits GROUP BY "UserID", m, "SearchPhrase" ORDER BY COUNT(*) DESC LIMIT 10;
"#,
        ),
        (
            "21",
            r#"
SELECT COUNT(*) FROM hits WHERE "URL" LIKE '%google%';
"#,
        ),
        (
            "22",
            r#"
SELECT "SearchPhrase", MIN("URL"), COUNT(*) AS c FROM hits WHERE "URL" LIKE '%google%' AND "SearchPhrase" <> '' GROUP BY "SearchPhrase" ORDER BY c DESC LIMIT 10;
"#,
        ),
        (
            "23",
            r#"
SELECT "SearchPhrase", MIN("URL"), MIN("Title"), COUNT(*) AS c, COUNT(DISTINCT "UserID") FROM hits WHERE "Title" LIKE '%Google%' AND "URL" NOT LIKE '%.google.%' AND "SearchPhrase" <> '' GROUP BY "SearchPhrase" ORDER BY c DESC LIMIT 10;
"#,
        ),
        (
            "24",
            r#"
SELECT * FROM hits WHERE "URL" LIKE '%google%' ORDER BY "EventTime" LIMIT 10;
"#,
        ),
        (
            "26",
            r#"
SELECT "SearchPhrase" FROM hits WHERE "SearchPhrase" <> '' ORDER BY "SearchPhrase" LIMIT 10;
"#,
        ),
        (
            "27",
            r#"
SELECT "SearchPhrase" FROM hits WHERE "SearchPhrase" <> '' ORDER BY "EventTime", "SearchPhrase" LIMIT 10;
"#,
        ),
        (
            "29",
            r#"
SELECT REGEXP_REPLACE("Referer", '^https?://(?:www\.)?([^/]+)/.*$', '\1') AS k, AVG(LENGTH("Referer")) AS l, COUNT(*) AS c, MIN("Referer") FROM hits WHERE "Referer" <> '' GROUP BY k HAVING COUNT(*) > 100000 ORDER BY l DESC LIMIT 25;
"#,
        ),
        (
            "31",
            r#"
SELECT "SearchEngineID", "ClientIP", COUNT(*) AS c, SUM("IsRefresh"), AVG("ResolutionWidth") FROM hits WHERE "SearchPhrase" <> '' GROUP BY "SearchEngineID", "ClientIP" ORDER BY c DESC LIMIT 10;
"#,
        ),
        (
            "32",
            r#"
SELECT "WatchID", "ClientIP", COUNT(*) AS c, SUM("IsRefresh"), AVG("ResolutionWidth") FROM hits WHERE "SearchPhrase" <> '' GROUP BY "WatchID", "ClientIP" ORDER BY c DESC LIMIT 10;
"#,
        ),
        (
            "33",
            r#"
SELECT "WatchID", "ClientIP", COUNT(*) AS c, SUM("IsRefresh"), AVG("ResolutionWidth") FROM hits GROUP BY "WatchID", "ClientIP" ORDER BY c DESC LIMIT 10;
"#,
        ),
        (
            "34",
            r#"
SELECT "URL", COUNT(*) AS c FROM hits GROUP BY "URL" ORDER BY c DESC LIMIT 10;
"#,
        ),
        (
            "37",
            r#"
SELECT "URL", COUNT(*) AS "PageViews" FROM hits WHERE "CounterID" = 62 AND "EventDate" >= '2013-07-01' AND "EventDate" <= '2013-07-31' AND "DontCountHits" = 0 AND "IsRefresh" = 0 AND "URL" <> '' GROUP BY "URL" ORDER BY "PageViews" DESC LIMIT 10;
"#,
        ),
        (
            "38",
            r#"
SELECT "Title", COUNT(*) AS "PageViews" FROM hits WHERE "CounterID" = 62 AND "EventDate" >= '2013-07-01' AND "EventDate" <= '2013-07-31' AND "DontCountHits" = 0 AND "IsRefresh" = 0 AND "Title" <> '' GROUP BY "Title" ORDER BY "PageViews" DESC LIMIT 10;
"#,
        ),
        (
            "39",
            r#"
SELECT "URL", COUNT(*) AS "PageViews" FROM hits WHERE "CounterID" = 62 AND "EventDate" >= '2013-07-01' AND "EventDate" <= '2013-07-31' AND "IsRefresh" = 0 AND "IsLink" <> 0 AND "IsDownload" = 0 GROUP BY "URL" ORDER BY "PageViews" DESC LIMIT 10 OFFSET 1000;
"#,
        ),
        (
            "40",
            r#"
SELECT "TraficSourceID", "SearchEngineID", "AdvEngineID", CASE WHEN ("SearchEngineID" = 0 AND "AdvEngineID" = 0) THEN "Referer" ELSE '' END AS "Src", "URL" AS "Dst", COUNT(*) AS "PageViews" FROM hits WHERE "CounterID" = 62 AND "EventDate" >= '2013-07-01' AND "EventDate" <= '2013-07-31' AND "IsRefresh" = 0 GROUP BY "TraficSourceID", "SearchEngineID", "AdvEngineID", "Src", "Dst" ORDER BY "PageViews" DESC LIMIT 10 OFFSET 1000;
"#,
        ),
    ]
}
