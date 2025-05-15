use anyblox::config::LogLevel;
use anyblox_datafusion::AnyBloxTable;
use anyhow::Result;
use arrow::array::{Array, AsArray};
use arrow::record_batch::RecordBatch;
use clap::Parser;
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

const WARMUP_RUNS: usize = 5;
const SAMPLES: usize = 10;

#[derive(Debug, Parser, Clone)]
#[command(flatten_help = true)]
pub struct Args {
    /// Path to the AnyBlox file.
    anyblox_path: PathBuf,
    /// Path to the parquet directory.
    parquet_dir: PathBuf,
    /// Report output path.
    out_path: PathBuf,
    #[clap(long, required = false, default_value = "false")]
    native: bool,
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

        let anyblox_table = if args.native {
            AnyBloxTable::new_native(anyblox_rt.clone(), args.anyblox_path, None::<PathBuf>).await?
        } else {
            AnyBloxTable::new_wasm(anyblox_rt.clone(), args.anyblox_path, None::<PathBuf>).await?
        };
        ctx.register_table("lineitem", Arc::new(anyblox_table))?;
        add_parquet_tables(&ctx, args.parquet_dir).await?;
        let mut report = Report::new();

        for (q_name, q_sql) in queries() {
            // Warmup
            for _ in 0..WARMUP_RUNS {
                let df = ctx.sql(q_sql).await?;
                df.show().await?;
            }

            for _ in 0..SAMPLES {
                let df = ctx.sql(q_sql).await?.explain(true, true)?;
                let results = df.collect().await?;
                report.add_explain_results(q_name.to_string(), results);
            }
        }

        let mut file = tokio::fs::File::create(&args.out_path).await?;
        file.write_all(report.to_string().as_bytes()).await?;

        Ok(())
    })
}

async fn add_parquet_tables(ctx: &DataFusionSessionContext, dir_path: PathBuf) -> Result<()> {
    for table in ["part", "region", "nation", "supplier", "partsupp", "customer", "orders"] {
        let file_path = dir_path.join(table).with_extension("parquet");
        ctx.register_parquet(table, file_path.to_str().unwrap(), Default::default())
            .await?;
    }
    let file_path = dir_path.join("lineitem").with_extension("parquet");
    ctx.register_parquet("lineitem_ref", file_path.to_str().unwrap(), Default::default())
        .await?;

    Ok(())
}

#[derive(serde::Serialize)]
struct Report(HashMap<String, Vec<(f64, f64)>>);

impl Report {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn add_explain_results(&mut self, query_name: String, results: Vec<RecordBatch>) {
        assert_eq!(results.len(), 1);
        let results = &results[0];
        assert_eq!(results.num_rows(), 4);
        assert_eq!(results.num_columns(), 2);
        let runtime_string = results.column(1).as_string::<i32>().value(3);
        let total_runtime =
            parse_duration(runtime_string).expect(format!("valid time, got {}", runtime_string).as_str());

        let detailed_plan = results.column(1).as_string::<i32>().value(1);
        let anyblox_time = sum_anyblox_time_from_explain_output(detailed_plan);

        let result = (total_runtime.as_secs_f64(), anyblox_time.as_secs_f64());
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

fn queries() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "q1",
            r#"
select
        l_returnflag,
        l_linestatus,
        sum(l_quantity) as sum_qty,
        sum(l_extendedprice) as sum_base_price,
        sum(l_extendedprice * (1 - l_discount)) as sum_disc_price,
        sum(l_extendedprice * (1 - l_discount) * (1 + l_tax)) as sum_charge,
        avg(l_quantity) as avg_qty,
        avg(l_extendedprice) as avg_price,
        avg(l_discount) as avg_disc,
        count(*) as count_order
from
        lineitem
where
        l_shipdate <= date '1998-12-01' - interval '90' day
group by
        l_returnflag,
        l_linestatus
order by
        l_returnflag,
        l_linestatus;
        "#,
        ),
        (
            "q3",
            r#"
select
        l_orderkey,
        sum(l_extendedprice * (1 - l_discount)) as revenue,
        o_orderdate,
        o_shippriority
from
        customer,
        orders,
        lineitem
where
        c_mktsegment = 'BUILDING'
        and c_custkey = o_custkey
        and l_orderkey = o_orderkey
        and o_orderdate < date '1995-03-15'
        and l_shipdate > date '1995-03-15'
group by
        l_orderkey,
        o_orderdate,
        o_shippriority
order by
        revenue desc,
        o_orderdate
limit
        10;"#,
        ),
        (
            "q4",
            r#"
select
        o_orderpriority,
        count(*) as order_count
from
        orders
where
        o_orderdate >= date '1993-07-01'
        and o_orderdate < date '1993-07-01' + interval '3' month
        and exists (
                select
                        *
                from
                        lineitem
                where
                        l_orderkey = o_orderkey
                        and l_commitdate < l_receiptdate
        )
group by
        o_orderpriority
order by
        o_orderpriority;"#,
        ),
        (
            "q5",
            r#"
select
        n_name,
        sum(l_extendedprice * (1 - l_discount)) as revenue
from
        customer,
        orders,
        lineitem,
        supplier,
        nation,
        region
where
        c_custkey = o_custkey
        and l_orderkey = o_orderkey
        and l_suppkey = s_suppkey
        and c_nationkey = s_nationkey
        and s_nationkey = n_nationkey
        and n_regionkey = r_regionkey
        and r_name = 'ASIA'
        and o_orderdate >= date '1994-01-01'
        and o_orderdate < date '1994-01-01' + interval '1' year
group by
        n_name
order by
        revenue desc;"#,
        ),
        (
            "q6",
            r#"
select
        sum(l_extendedprice * l_discount) as revenue
from
        lineitem
where
        l_shipdate >= date '1994-01-01'
        and l_shipdate < date '1994-01-01' + interval '1' year
        and l_discount between 0.06 - 0.01 and 0.06 + 0.01
        and l_quantity < 24;"#,
        ),
        (
            "q7",
            r#"
select
        supp_nation,
        cust_nation,
        l_year,
        sum(volume) as revenue
from
        (
                select
                        n1.n_name as supp_nation,
                        n2.n_name as cust_nation,
                        extract(year from l_shipdate) as l_year,
                        l_extendedprice * (1 - l_discount) as volume
                from
                        supplier,
                        lineitem,
                        orders,
                        customer,
                        nation n1,
                        nation n2
                where
                        s_suppkey = l_suppkey
                        and o_orderkey = l_orderkey
                        and c_custkey = o_custkey
                        and s_nationkey = n1.n_nationkey
                        and c_nationkey = n2.n_nationkey
                        and (
                                (n1.n_name = 'FRANCE' and n2.n_name = 'GERMANY')
                                or (n1.n_name = 'GERMANY' and n2.n_name = 'FRANCE')
                        )
                        and l_shipdate between date '1995-01-01' and date '1996-12-31'
        ) as shipping
group by
        supp_nation,
        cust_nation,
        l_year
order by
        supp_nation,
        cust_nation,
        l_year;"#,
        ),
        (
            "q8",
            r#"
select
        o_year,
        sum(case
                when nation = 'BRAZIL' then volume
                else 0
        end) / sum(volume) as mkt_share
from
        (
                select
                        extract(year from o_orderdate) as o_year,
                        l_extendedprice * (1 - l_discount) as volume,
                        n2.n_name as nation
                from
                        part,
                        supplier,
                        lineitem,
                        orders,
                        customer,
                        nation n1,
                        nation n2,
                        region
                where
                        p_partkey = l_partkey
                        and s_suppkey = l_suppkey
                        and l_orderkey = o_orderkey
                        and o_custkey = c_custkey
                        and c_nationkey = n1.n_nationkey
                        and n1.n_regionkey = r_regionkey
                        and r_name = 'AMERICA'
                        and s_nationkey = n2.n_nationkey
                        and o_orderdate between date '1995-01-01' and date '1996-12-31'
                        and p_type = 'ECONOMY ANODIZED STEEL'
        ) as all_nations
group by
        o_year
order by
        o_year;"#,
        ),
        (
            "q9",
            r#"
select
        nation,
        o_year,
        sum(amount) as sum_profit
from
        (
                select
                        n_name as nation,
                        extract(year from o_orderdate) as o_year,
                        l_extendedprice * (1 - l_discount) - ps_supplycost * l_quantity as amount
                from
                        part,
                        supplier,
                        lineitem,
                        partsupp,
                        orders,
                        nation
                where
                        s_suppkey = l_suppkey
                        and ps_suppkey = l_suppkey
                        and ps_partkey = l_partkey
                        and p_partkey = l_partkey
                        and o_orderkey = l_orderkey
                        and s_nationkey = n_nationkey
                        and p_name like '%green%'
        ) as profit
group by
        nation,
        o_year
order by
        nation,
        o_year desc;"#,
        ),
        (
            "q10",
            r#"
select
        c_custkey,
        c_name,
        sum(l_extendedprice * (1 - l_discount)) as revenue,
        c_acctbal,
        n_name,
        c_address,
        c_phone,
        c_comment
from
        customer,
        orders,
        lineitem,
        nation
where
        c_custkey = o_custkey
        and l_orderkey = o_orderkey
        and o_orderdate >= date '1993-10-01'
        and o_orderdate < date '1993-10-01' + interval '3' month
        and l_returnflag = 'R'
        and c_nationkey = n_nationkey
group by
        c_custkey,
        c_name,
        c_acctbal,
        c_phone,
        n_name,
        c_address,
        c_comment
order by
        revenue desc
limit
        20;"#,
        ),
        (
            "q12",
            r#"
select
        l_shipmode,
        sum(case
                when o_orderpriority = '1-URGENT'
                        or o_orderpriority = '2-HIGH'
                        then 1
                else 0
        end) as high_line_count,
        sum(case
                when o_orderpriority <> '1-URGENT'
                        and o_orderpriority <> '2-HIGH'
                        then 1
                else 0
        end) as low_line_count
from
        orders,
        lineitem
where
        o_orderkey = l_orderkey
        and l_shipmode in ('MAIL', 'SHIP')
        and l_commitdate < l_receiptdate
        and l_shipdate < l_commitdate
        and l_receiptdate >= date '1994-01-01'
        and l_receiptdate < date '1994-01-01' + interval '1' year
group by
        l_shipmode
order by
        l_shipmode;"#,
        ),
        (
            "q14",
            r#"
select
        100.00 * sum(case
                when p_type like 'PROMO%'
                        then l_extendedprice * (1 - l_discount)
                else 0
        end) / sum(l_extendedprice * (1 - l_discount)) as promo_revenue
from
        lineitem,
        part
where
        l_partkey = p_partkey
        and l_shipdate >= date '1995-09-01'
        and l_shipdate < date '1995-09-01' + interval '1' month;"#,
        ),
        (
            "q15",
            r#"
with revenue (supplier_no, total_revenue) as (
        select
                l_suppkey,
                sum(l_extendedprice * (1 - l_discount))
        from
                lineitem
        where
                l_shipdate >= date '1996-01-01'
                and l_shipdate < date '1996-01-01' + interval '3' month
        group by
                l_suppkey)
select
        s_suppkey,
        s_name,
        s_address,
        s_phone,
        total_revenue
from
        supplier,
        revenue
where
        s_suppkey = supplier_no
        and total_revenue = (
                select
                        max(total_revenue)
                from
                        revenue
        )
order by
        s_suppkey;"#,
        ),
        (
            "q17",
            r#"
select
        sum(l_extendedprice) / 7.0 as avg_yearly
from
        lineitem,
        part
where
        p_partkey = l_partkey
        and p_brand = 'Brand#23'
        and p_container = 'MED BOX'
        and l_quantity < (
                select
                        0.2 * avg(l_quantity)
                from
                        lineitem
                where
                        l_partkey = p_partkey
        );"#,
        ),
        (
            "q18",
            r#"
select
        c_name,
        c_custkey,
        o_orderkey,
        o_orderdate,
        o_totalprice,
        sum(l_quantity)
from
        customer,
        orders,
        lineitem
where
        o_orderkey in (
                select
                        l_orderkey
                from
                        lineitem
                group by
                        l_orderkey having
                                sum(l_quantity) > 300
        )
        and c_custkey = o_custkey
        and o_orderkey = l_orderkey
group by
        c_name,
        c_custkey,
        o_orderkey,
        o_orderdate,
        o_totalprice
order by
        o_totalprice desc,
        o_orderdate
limit
        100;"#,
        ),
        (
            "q19",
            r#"
select
        sum(l_extendedprice* (1 - l_discount)) as revenue
from
        lineitem,
        part
where
        (
                p_partkey = l_partkey
                and p_brand = 'Brand#12'
                and p_container in ('SM CASE', 'SM BOX', 'SM PACK', 'SM PKG')
                and l_quantity >= 1 and l_quantity <= 1 + 10
                and p_size between 1 and 5
                and l_shipmode in ('AIR', 'AIR REG')
                and l_shipinstruct = 'DELIVER IN PERSON'
        )
        or
        (
                p_partkey = l_partkey
                and p_brand = 'Brand#23'
                and p_container in ('MED BAG', 'MED BOX', 'MED PKG', 'MED PACK')
                and l_quantity >= 10 and l_quantity <= 10 + 10
                and p_size between 1 and 10
                and l_shipmode in ('AIR', 'AIR REG')
                and l_shipinstruct = 'DELIVER IN PERSON'
        )
        or
        (
                p_partkey = l_partkey
                and p_brand = 'Brand#34'
                and p_container in ('LG CASE', 'LG BOX', 'LG PACK', 'LG PKG')
                and l_quantity >= 20 and l_quantity <= 20 + 10
                and p_size between 1 and 15
                and l_shipmode in ('AIR', 'AIR REG')
                and l_shipinstruct = 'DELIVER IN PERSON'
        );"#,
        ),
        (
            "q20",
            r#"
select
        s_name,
        s_address
from
        supplier,
        nation
where
        s_suppkey in (
                select
                        ps_suppkey
                from
                        partsupp
                where
                        ps_partkey in (
                                select
                                        p_partkey
                                from
                                        part
                                where
                                        p_name like 'forest%'
                        )
                        and ps_availqty > (
                                select
                                        0.5 * sum(l_quantity)
                                from
                                        lineitem
                                where
                                        l_partkey = ps_partkey
                                        and l_suppkey = ps_suppkey
                                        and l_shipdate >= date '1994-01-01'
                                        and l_shipdate < date '1994-01-01' + interval '1' year
                        )
        )
        and s_nationkey = n_nationkey
        and n_name = 'CANADA'
order by
        s_name;"#,
        ),
        (
            "q21",
            r#"
select
        s_name,
        count(*) as numwait
from
        supplier,
        lineitem l1,
        orders,
        nation
where
        s_suppkey = l1.l_suppkey
        and o_orderkey = l1.l_orderkey
        and o_orderstatus = 'F'
        and l1.l_receiptdate > l1.l_commitdate
        and exists (
                select
                        *
                from
                        lineitem l2
                where
                        l2.l_orderkey = l1.l_orderkey
                        and l2.l_suppkey <> l1.l_suppkey
        )
        and not exists (
                select
                        *
                from
                        lineitem l3
                where
                        l3.l_orderkey = l1.l_orderkey
                        and l3.l_suppkey <> l1.l_suppkey
                        and l3.l_receiptdate > l3.l_commitdate
        )
        and s_nationkey = n_nationkey
        and n_name = 'SAUDI ARABIA'
group by
        s_name
order by
        numwait desc,
        s_name
limit
        100;"#,
        ),
    ]
}
