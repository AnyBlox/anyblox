import json
import matplotlib.pyplot as plt
import matplotlib.patheffects as pe
import numpy as np
import sys

PX_PER_IN = 250
DUCKDB_TPCH_IGNITION = "./duckdb-results-s20"
DUCKDB_TPCH_PARQUET = "./duckdb-results-parquet-s20"
DUCKDB_TPCH_RAW = "./duckdb-results-raw-s20"

DUCKDB_CLICKBENCH_IGNITION = "./duckdb-results-clickbench"
DUCKDB_CLICKBENCH_PARQUET = "./duckdb-results-parquet-clickbench"
DUCKDB_CLICKBENCH_RAW = "./duckdb-results-raw-clickbench"

SPARK_TPCH_IGNITION = "./spark-results-ignition-s20"
SPARK_TPCH_PARQUET = "./spark-results-parquet-s20"

COL_DUCKDB_PRIMARY = '#FFF100'
COL_DUCKDB_SECONDARY = '#999100'
COL_DUCKDB_TERTIARY = '#FFF766'
COL_IGNITION_PRIMARY = '#CC0256'
COL_IGNITION_SECONDARY = '#67012B'
COL_IGNITION_TERTIARY = '#FD378A'
COL_PARQUET_PRIMARY = '#069AF3'
COL_PARQUET_SECONDARY = '#045A8F'
COL_PARQUET_TERTIARY = '#64C1FB'
LEGEND_FONT_SIZE = 7

TPCH_QS = ["1", "3", "4", "5", "6", "7", "8", "9", "10",
      "12", "14", "15", "17", "18", "19", "20", "21"]
tpch_xs = np.arange(len(TPCH_QS))

CLICKBENCH_QS = ["6", "11", "12", "13", "14", "15", "17", "18", "19", "21", "22", "23", "24", "26", "27", "29", "31", "32", "33", "34", "37", "38", "39", "40"]
clickbench_xs = np.arange(len(CLICKBENCH_QS))

FROM_MS = 1000
FROM_μS = 1000000
FROM_NS = 1000000000
FROM_B = 1000000

plt.rcParams["font.family"] = "Libertinus Sans"

def plot_tpch_duckdb_multithread(threads):
    with open(f"{DUCKDB_TPCH_IGNITION}/tpch-t{threads}-results.json", 'r') as file:
        ignition_data = json.load(file)

    with open(f"{DUCKDB_TPCH_PARQUET}/tpch-t{threads}-results.json", 'r') as file:
        parquet_data = json.load(file)

    with open(f"{DUCKDB_TPCH_RAW}/tpch-t{threads}-results.json", 'r') as file:
        raw_data = json.load(file)

    ignition = [
        np.median(list(x for [x, y] in ignition_data[q])) for q in TPCH_QS]
    parquet = [np.median(list(x for [x, y] in parquet_data[q]))
                for q in TPCH_QS]
    raw = [np.median(list(x for [x, y] in raw_data[q]))
            for q in TPCH_QS]

    figure, axes = plt.subplots()
    width = 0.25

    rects_raw = axes.bar(tpch_xs, raw, width, color=COL_DUCKDB_PRIMARY)
    rects_ignition = axes.bar(tpch_xs + width, ignition, width, color=COL_IGNITION_PRIMARY)
    rects_parquet = axes.bar(tpch_xs + 2 * width, parquet, width, color=COL_PARQUET_PRIMARY)

    axes.set_ylabel('latency (s)')
    axes.set_ylim(ymin=0)
    axes.set_title(f'TPC-H (sf=20, t={threads}) DuckDB')
    axes.set_xticks(tpch_xs + width/2)
    axes.set_xticklabels([f"q{q}" for q in TPCH_QS])

    axes.legend([rects_ignition, rects_parquet, rects_raw], [
                'ignition', 'parquet', "table"], title='lineitem', fontsize=LEGEND_FONT_SIZE, loc='upper left')
    axes.grid(linestyle='--', axis='y')

    figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)
    figure.savefig(f"duckdb_tpch_s20_t{threads}.pdf")


def plot_tpch_duckdb_single_thread():
    with open(f"{DUCKDB_TPCH_IGNITION}/tpch-t1-results.json", 'r') as file:
        ignition_t1_data = json.load(file)

    with open(f"{DUCKDB_TPCH_PARQUET}/tpch-t1-results.json", 'r') as file:
        parquet_t1_data = json.load(file)

    with open(f"results-parquet-uncompressed-s20/tpch-t1-results.json", 'r') as file:
        parquet_uncompressed_t1_data = json.load(file)
    with open(f"results-parquet-zstd-s20/tpch-t1-results.json", 'r') as file:
        parquet_zstd_t1_data = json.load(file)
    with open(f"results-parquet-gzip-s20/tpch-t1-results.json", 'r') as file:
        parquet_gzip_t1_data = json.load(file)

    with open(f"{DUCKDB_TPCH_RAW}/tpch-t1-results.json", 'r') as file:
        raw_t1_data = json.load(file)

    ignition_scan_t1 = [
        np.median(list(y for [x, y] in ignition_t1_data[q])) for q in TPCH_QS]
    parquet_scan_t1 = [
        np.median(list(y for [x, y] in parquet_t1_data[q])) for q in TPCH_QS]
    parquet_uncompressed_scan_t1 = [
        np.median(list(y for [x, y] in parquet_uncompressed_t1_data[q])) for q in TPCH_QS]
    parquet_zstd_scan_t1 = [
        np.median(list(y for [x, y] in parquet_zstd_t1_data[q])) for q in TPCH_QS]
    parquet_gzip_scan_t1 = [
        np.median(list(y for [x, y] in parquet_gzip_t1_data[q])) for q in TPCH_QS]
    raw_scan_t1 = [
        np.median(list(y for [x, y] in raw_t1_data[q])) for q in TPCH_QS]
    ignition_query_t1 = [
        np.median(list(x - y for [x, y] in ignition_t1_data[q])) for q in TPCH_QS]
    parquet_query_t1 = [
        np.median(list(x - y for [x, y] in parquet_t1_data[q])) for q in TPCH_QS]
    parquet_uncompressed_query_t1 = [
        np.median(list(x - y for [x, y] in parquet_uncompressed_t1_data[q])) for q in TPCH_QS]
    parquet_zstd_query_t1 = [
        np.median(list(x - y for [x, y] in parquet_zstd_t1_data[q])) for q in TPCH_QS]
    parquet_gzip_query_t1 = [
        np.median(list(x - y for [x, y] in parquet_gzip_t1_data[q])) for q in TPCH_QS]
    raw_query_t1 = [
        np.median(list(x - y for [x, y] in raw_t1_data[q])) for q in TPCH_QS]

    figure, axes = plt.subplots()
    width = 1 / 7

    rects_raw_query = axes.bar(
        tpch_xs, raw_query_t1, width, color=COL_DUCKDB_PRIMARY)
    rects_raw_scan = axes.bar(
        tpch_xs, raw_scan_t1, width, color=COL_DUCKDB_SECONDARY, bottom=raw_query_t1)

    rects_ignition_query = axes.bar(
        tpch_xs + width, ignition_query_t1, width, color=COL_IGNITION_PRIMARY)
    rects_ignition_scan = axes.bar(tpch_xs + width, ignition_scan_t1, width, color=COL_IGNITION_SECONDARY, bottom=ignition_query_t1)

    rects_parquet_query = axes.bar(
        tpch_xs + 2 * width, parquet_query_t1, width, color=COL_PARQUET_PRIMARY)
    rects_parquet_scan = axes.bar(
        tpch_xs + 2 * width, parquet_scan_t1, width, color=COL_PARQUET_SECONDARY, bottom=parquet_query_t1)
    rects_parquet_uncompressed_query = axes.bar(
        tpch_xs + 3 * width, parquet_uncompressed_query_t1, width, color=COL_PARQUET_PRIMARY, hatch="/")
    rects_parquet_uncompressed_scan = axes.bar(
        tpch_xs + 3 * width, parquet_uncompressed_scan_t1, width, color=COL_PARQUET_SECONDARY, bottom=parquet_uncompressed_query_t1, hatch="/")
    rects_parquet_zstd_query = axes.bar(
        tpch_xs + 4 * width, parquet_zstd_query_t1, width, color=COL_PARQUET_PRIMARY, hatch="\\")
    rects_parquet_zstd_scan = axes.bar(
        tpch_xs + 4 * width, parquet_zstd_scan_t1, width, color=COL_PARQUET_SECONDARY, bottom=parquet_zstd_query_t1, hatch="\\")
    rects_parquet_gzip_query = axes.bar(
        tpch_xs + 5 * width, parquet_gzip_query_t1, width, color=COL_PARQUET_PRIMARY, hatch="|")
    rects_parquet_gzip_scan = axes.bar(
        tpch_xs + 5 * width, parquet_gzip_scan_t1, width, color=COL_PARQUET_SECONDARY, bottom=parquet_gzip_query_t1, hatch="|")

    axes.set_ylabel('latency (s)')
    axes.set_ylim(ymin=0)
    axes.set_title('TPC-H (sf=20, t=1) DuckDB')
    axes.set_xticks(tpch_xs + width/2)
    axes.set_xticklabels([f"q{q}" for q in TPCH_QS])

    axes.legend([rects_ignition_scan, rects_ignition_query, 
    rects_parquet_scan, rects_parquet_query, 
    rects_parquet_uncompressed_scan, rects_parquet_uncompressed_query, 
    rects_parquet_zstd_scan, rects_parquet_zstd_query, 
    rects_parquet_gzip_scan, rects_parquet_gzip_query, 
    rects_raw_scan, rects_raw_query], [
                'ignition scan', 'ignition query',
                'parquet scan', 'parquet query',
                'parquet un scan', 'parquet un query',
                'parquet zstd scan', 'parquet zstd query',
                'parquet gzip scan', 'parquet gzip query',
                'table scan', 'table query'],
                title='lineitem', fontsize=LEGEND_FONT_SIZE, loc='upper left')
    axes.grid(linestyle='--', axis='y')

    figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)
    figure.savefig("duckdb_tpch_s20_t1_ulala.pdf")


def plot_tpch_spark_single_thread():
    with open(f"{SPARK_TPCH_IGNITION}/tpch-t1-results.json", 'r') as file:
        ignition_t1_data = json.load(file)

    with open(f"{SPARK_TPCH_PARQUET}/tpch-t1-results.json", 'r') as file:
        parquet_t1_data = json.load(file)

    ignition_scan_t1 = [
        np.median(list(y / 1000.0 for [x, y] in ignition_t1_data[q])) for q in TPCH_QS]
    parquet_scan_t1 = [
        np.median(list(y / 1000.0 for [x, y] in parquet_t1_data[q])) for q in TPCH_QS]
    ignition_query_t1 = [
        np.median(list((x - y) / 1000.0 for [x, y] in ignition_t1_data[q])) for q in TPCH_QS]
    parquet_query_t1 = [
        np.median(list((x - y) / 1000.0 for [x, y] in parquet_t1_data[q])) for q in TPCH_QS]

    total_ignition_scan = sum(ignition_scan_t1)
    total_parquet_scan = sum(parquet_scan_t1)
    total_ignition_query = sum(ignition_query_t1)
    total_parquet_query = sum(parquet_query_t1)
    print(f"spark tpch: { {
        "is": total_ignition_scan,
        "ps": total_parquet_scan,
        "iq": total_ignition_query,
        "pq": total_parquet_query,
    } }")

    figure, axes = plt.subplots()
    width = 0.33

    rects_ignition_query = axes.bar(
        tpch_xs, ignition_query_t1, width, color=COL_IGNITION_PRIMARY)
    rects_ignition_scan = axes.bar(tpch_xs, ignition_scan_t1, width, color=COL_IGNITION_SECONDARY, bottom=ignition_query_t1)

    rects_parquet_query = axes.bar(
        tpch_xs + width, parquet_query_t1, width, color=COL_PARQUET_PRIMARY)
    rects_parquet_scan = axes.bar(
        tpch_xs + width, parquet_scan_t1, width, color=COL_PARQUET_SECONDARY, bottom=parquet_query_t1)

    axes.set_ylabel('latency (s)')
    axes.set_ylim(ymin=0)
    axes.set_title('TPC-H (sf=20, t=1) Spark')
    axes.set_xticks(tpch_xs + width/2)
    axes.set_xticklabels([f"q{q}" for q in TPCH_QS])

    axes.legend([rects_ignition_scan, rects_ignition_query, rects_parquet_scan, rects_parquet_query], [
                'ignition scan', 'ignition query', 'parquet scan', 'parquet query'],
                title='lineitem', fontsize=LEGEND_FONT_SIZE, loc='upper left')
    axes.grid(linestyle='--', axis='y')

    figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)
    figure.savefig("spark_tpch_s20_t1.pdf")


def plot_tpch_spark_multithread(threads):
    with open(f"{SPARK_TPCH_IGNITION}/tpch-t{threads}-results.json", 'r') as file:
        ignition_data = json.load(file)

    with open(f"{SPARK_TPCH_PARQUET}/tpch-t{threads}-results.json", 'r') as file:
        parquet_data = json.load(file)

    ignition = [
        np.median(list(x / 1000.0 for [x, y] in ignition_data[q])) for q in TPCH_QS]
    parquet = [np.median(list(x / 1000.0 for [x, y] in parquet_data[q]))
                for q in TPCH_QS]

    figure, axes = plt.subplots()
    width = 0.33

    rects_ignition = axes.bar(tpch_xs, ignition, width, color=COL_IGNITION_PRIMARY)
    rects_parquet = axes.bar(tpch_xs + width, parquet, width, color=COL_PARQUET_PRIMARY)

    axes.set_ylabel('latency (s)')
    axes.set_ylim(ymin=0)
    axes.set_title(f'TPC-H (sf=20, t={threads}) Spark')
    axes.set_xticks(tpch_xs + width/2)
    axes.set_xticklabels([f"q{q}" for q in TPCH_QS])

    axes.legend([rects_ignition, rects_parquet], [
                'ignition', 'parquet'], title='lineitem', fontsize=LEGEND_FONT_SIZE, loc='upper left')
    axes.grid(linestyle='--', axis='y')

    figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)
    figure.savefig(f"spark_tpch_s20_t{threads}.pdf")


def plot_clickbench_duckdb_multithread(threads):
    with open(f"{DUCKDB_CLICKBENCH_IGNITION}/clickbench-t{threads}-results.json", 'r') as file:
        ignition_data = json.load(file)

    with open(f"{DUCKDB_CLICKBENCH_PARQUET}/clickbench-t{threads}-results.json", 'r') as file:
        parquet_data = json.load(file)

    with open(f"{DUCKDB_CLICKBENCH_RAW}/clickbench-t{threads}-results.json", 'r') as file:
        raw_data = json.load(file)

    ignition = [
        np.median(list(x for [x, y] in ignition_data[q])) for q in CLICKBENCH_QS]
    parquet = [np.median(list(x for [x, y] in parquet_data[q]))
                for q in CLICKBENCH_QS]
    raw = [np.median(list(x for [x, y] in raw_data[q]))
            for q in CLICKBENCH_QS]

    figure, axes = plt.subplots()
    width = 0.25

    rects_raw = axes.bar(clickbench_xs, raw, width, color=COL_DUCKDB_PRIMARY)
    rects_ignition = axes.bar(clickbench_xs + width, ignition, width, color=COL_IGNITION_PRIMARY)
    rects_parquet = axes.bar(clickbench_xs + 2 * width, parquet, width, color=COL_PARQUET_PRIMARY)

    axes.set_ylabel('latency (s)')
    axes.set_ylim(ymin=0)
    axes.set_title(f'ClickBench (t={threads}) DuckDB')
    axes.set_xticks(clickbench_xs + width/2)
    axes.set_xticklabels([f"q{q}" for q in CLICKBENCH_QS])

    axes.legend([rects_ignition, rects_parquet, rects_raw], [
                'ignition', 'parquet', "table"], title='hits_strings', fontsize=LEGEND_FONT_SIZE, loc='upper left')
    axes.grid(linestyle='--', axis='y')

    figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)
    figure.savefig(f"duckdb_clickbench_t{threads}.pdf")


def plot_clickbench_duckdb_single_thread():
    with open(f"{DUCKDB_CLICKBENCH_IGNITION}/clickbench-t1-results.json", 'r') as file:
        ignition_t1_data = json.load(file)

    with open(f"{DUCKDB_CLICKBENCH_PARQUET}/clickbench-t1-results.json", 'r') as file:
        parquet_t1_data = json.load(file)

    with open(f"{DUCKDB_CLICKBENCH_RAW}/clickbench-t1-results.json", 'r') as file:
        raw_t1_data = json.load(file)

    ignition_scan_t1 = [
        np.median(list(y for [x, y] in ignition_t1_data[q])) for q in CLICKBENCH_QS]
    parquet_scan_t1 = [
        np.median(list(y for [x, y] in parquet_t1_data[q])) for q in CLICKBENCH_QS]
    raw_scan_t1 = [
        np.median(list(y for [x, y] in raw_t1_data[q])) for q in CLICKBENCH_QS]
    ignition_query_t1 = [
        np.median(list(x - y for [x, y] in ignition_t1_data[q])) for q in CLICKBENCH_QS]
    parquet_query_t1 = [
        np.median(list(x - y for [x, y] in parquet_t1_data[q])) for q in CLICKBENCH_QS]
    raw_query_t1 = [
        np.median(list(x - y for [x, y] in raw_t1_data[q])) for q in CLICKBENCH_QS]

    total_ignition_scan = sum(ignition_scan_t1)
    total_parquet_scan = sum(parquet_scan_t1)
    total_raw_scan = sum(raw_scan_t1)
    total_ignition_query = sum(ignition_query_t1)
    total_parquet_query = sum(parquet_query_t1)
    total_raw_query = sum(raw_query_t1)
    print(f"duckdb clickbench: { {
        "is": total_ignition_scan,
        "ps": total_parquet_scan,
        "rs": total_raw_scan,
        "iq": total_ignition_query,
        "pq": total_parquet_query,
        "rq": total_raw_query
    } }")

    figure, axes = plt.subplots()
    width = 0.25

    rects_raw_query = axes.bar(
        clickbench_xs, raw_query_t1, width, color=COL_DUCKDB_PRIMARY)
    rects_raw_scan = axes.bar(
        clickbench_xs, raw_scan_t1, width, color=COL_DUCKDB_SECONDARY, bottom=raw_query_t1)

    rects_ignition_query = axes.bar(
        clickbench_xs + width, ignition_query_t1, width, color=COL_IGNITION_PRIMARY)
    rects_ignition_scan = axes.bar(clickbench_xs + width, ignition_scan_t1, width, color=COL_IGNITION_SECONDARY, bottom=ignition_query_t1)

    rects_parquet_query = axes.bar(
        clickbench_xs + 2 * width, parquet_query_t1, width, color=COL_PARQUET_PRIMARY)
    rects_parquet_scan = axes.bar(
        clickbench_xs + 2 * width, parquet_scan_t1, width, color=COL_PARQUET_SECONDARY, bottom=parquet_query_t1)

    axes.set_ylabel('latency (s)')
    axes.set_ylim(ymin=0)
    axes.set_title('ClickBench (t=1) DuckDB')
    axes.set_xticks(clickbench_xs + width/2)
    axes.set_xticklabels([f"q{q}" for q in CLICKBENCH_QS])

    axes.legend([rects_ignition_scan, rects_ignition_query, rects_parquet_scan, rects_parquet_query, rects_raw_scan, rects_raw_query], [
                'ignition scan', 'ignition query', 'parquet scan', 'parquet query', 'table scan', 'table query'],
                title='lineitem', fontsize=LEGEND_FONT_SIZE, loc='upper left')
    axes.grid(linestyle='--', axis='y')

    figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)
    figure.savefig("duckdb_clickbench_t1.pdf")


def plot_totals_once(ax, data, colors_primary, colors_secondary, qs, ratio=1):
    scans = [sum([np.median(list(y / ratio for [x, y] in d[q])) for q in qs]) for d in data]
    queries = [sum([np.median(list(x / ratio - y / ratio for [x, y] in d[q])) for q in qs]) for d in data]
    ratios = [scan / (scan + query) for (scan, query) in zip(scans, queries)]

    width = 1 / (len(data) + 1)
    xs = np.arange(1)

    rects_scans = [ax.bar(xs + i * width, [scan], width, color=colors_secondary[i]) for i, scan in enumerate(scans)]
    rects_queries = [ax.bar(xs + i * width, [query], width, color=colors_primary[i], bottom=scans[i]) for i, query in enumerate(queries)]
    
    ax.set_ylim(ymin=0)
    ax.set_xticks([(len(data) - 1) * width / 2])
    ax.grid(linestyle='--', axis='y')
    
    for i, scan in enumerate(scans):
        txt = ax.annotate(f'{ratios[i]*100:.0f}%', (xs + i * width, scan), color="white",
            xytext=(-10, 0), textcoords='offset points',
            horizontalalignment='left', verticalalignment='bottom')
        txt.set_path_effects([pe.withStroke(linewidth=3, foreground=colors_secondary[i])]),

    return (rects_scans, rects_queries)

def plot_totals():
    figure, (ax1, ax2, ax3) = plt.subplots(ncols=3)

    with open(f"{DUCKDB_TPCH_IGNITION}/tpch-t1-results.json", 'r') as file:
        ignition_t1_data = json.load(file)

    with open(f"{DUCKDB_TPCH_PARQUET}/tpch-t1-results.json", 'r') as file:
        parquet_t1_data = json.load(file)

    with open(f"{DUCKDB_TPCH_RAW}/tpch-t1-results.json", 'r') as file:
        raw_t1_data = json.load(file)

    plot_totals_once(ax1,
        [raw_t1_data, ignition_t1_data, parquet_t1_data],
        [COL_DUCKDB_PRIMARY, COL_IGNITION_PRIMARY, COL_PARQUET_PRIMARY],
        [COL_DUCKDB_SECONDARY, COL_IGNITION_SECONDARY, COL_PARQUET_SECONDARY],
        TPCH_QS
    )
    ax1.set_xticklabels(['TPC-H (DuckDB)'])
    ax1.set_ylabel('latency (s)')
    
    with open(f"{SPARK_TPCH_IGNITION}/tpch-t1-results.json", 'r') as file:
        ignition_t1_data = json.load(file)

    with open(f"{SPARK_TPCH_PARQUET}/tpch-t1-results.json", 'r') as file:
        parquet_t1_data = json.load(file)

    plot_totals_once(ax3,
        [ignition_t1_data, parquet_t1_data],
        [COL_IGNITION_PRIMARY, COL_PARQUET_PRIMARY],
        [COL_IGNITION_SECONDARY, COL_PARQUET_SECONDARY],
        TPCH_QS,
        1000
    )
    ax3.set_xticklabels(['TPC-H (Spark)'])

    with open(f"{DUCKDB_CLICKBENCH_IGNITION}/clickbench-t1-results.json", 'r') as file:
        ignition_t1_data = json.load(file)

    with open(f"{DUCKDB_CLICKBENCH_PARQUET}/clickbench-t1-results.json", 'r') as file:
        parquet_t1_data = json.load(file)

    with open(f"{DUCKDB_CLICKBENCH_RAW}/clickbench-t1-results.json", 'r') as file:
        raw_t1_data = json.load(file)

    (scan_rects, query_rects) = plot_totals_once(ax2,
        [raw_t1_data, ignition_t1_data, parquet_t1_data],
        [COL_DUCKDB_PRIMARY, COL_IGNITION_PRIMARY, COL_PARQUET_PRIMARY],
        [COL_DUCKDB_SECONDARY, COL_IGNITION_SECONDARY, COL_PARQUET_SECONDARY],
        CLICKBENCH_QS
    )
    ax2.set_xticklabels(['ClickBench (DuckDB)'])

    ax1.set_ylim(ymax=180)
    ax2.set_ylim(ymax=600)
    ax3.set_ylim(ymax=1000)

    legend_rects = [item for sublist in [[s, r] for s, r in zip(scan_rects, query_rects)] for item in sublist]

    figure.legend(legend_rects, [
                'table scan', 'table rest', 'anyblox scan', 'anyblox rest', 'parquet scan', 'parquet rest'],
                fontsize=LEGEND_FONT_SIZE, loc='center right')

    figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)
    figure.tight_layout()
    figure.savefig("totals.pdf")


def plot_2d():
    figure, (ax1, ax2) = plt.subplots(ncols=2)
    figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)

    with open(f"{DUCKDB_TPCH_IGNITION}/tpch-t1-results.json", 'r') as file:
        ignition_t1_data = json.load(file)

    with open(f"{DUCKDB_TPCH_PARQUET}/tpch-t1-results.json", 'r') as file:
        parquet_t1_data = json.load(file)

    with open(f"{DUCKDB_TPCH_RAW}/tpch-t1-results.json", 'r') as file:
        raw_t1_data = json.load(file)

    query_tuple_counts = np.genfromtxt("tpch-query-sizes.csv", delimiter=',', skip_header=1)[0:,1]
    tpch_tuple_count = sum(query_tuple_counts)

    ignition_scan_t1 = [
        np.median(list(y for [x, y] in ignition_t1_data[q])) for q in TPCH_QS]
    parquet_scan_t1 = [
        np.median(list(y for [x, y] in parquet_t1_data[q])) for q in TPCH_QS]
    raw_scan_t1 = [
        np.median(list(y for [x, y] in raw_t1_data[q])) for q in TPCH_QS]
    ignition_query_t1 = [
        np.median(list(x - y for [x, y] in ignition_t1_data[q])) for q in TPCH_QS]
    parquet_query_t1 = [
        np.median(list(x - y for [x, y] in parquet_t1_data[q])) for q in TPCH_QS]
    raw_query_t1 = [
        np.median(list(x - y for [x, y] in raw_t1_data[q])) for q in TPCH_QS]

    ignition_thpt = tpch_tuple_count / 1000000 / (sum(ignition_scan_t1) + sum(ignition_query_t1))
    parquet_thpt = tpch_tuple_count / 1000000 / (sum(parquet_scan_t1) + sum(parquet_query_t1))
    raw_thpt = tpch_tuple_count / 1000000 / (sum(raw_scan_t1) + sum(raw_query_t1))

    labels = ['DuckDB', 'Parquet (DuckDB)', 'AnyBlox (DuckDB)']
    sizes = 15685400985 / np.array([11307593728 - 3793235968, 5497795005, 3883788826])
    thpt = [raw_thpt, parquet_thpt, ignition_thpt]
    colors = [COL_DUCKDB_SECONDARY, COL_PARQUET_PRIMARY, COL_IGNITION_PRIMARY]
    vert_off = [-15, -15, -15]

    ax1.scatter(sizes, thpt, c=colors)
    ax1.grid(linestyle='--')
    ax1.set_xlim(xmin=1)
    ax1.set_ylim(ymin=0, ymax=50)
    ax1.set_xlabel('compression rate')
    ax1.set_ylabel('throughput (Mtuples)')
    ax1.set_title('TPC-H lineitem')

    for i, txt in enumerate(labels):
        ax1.annotate(txt, (sizes[i], thpt[i]), color=colors[i], fontsize=12,
        xytext=(-6*len(txt), vert_off[i]), textcoords='offset points',
        horizontalalignment='left', verticalalignment='bottom')

    ax1.annotate('better', (ax1.get_xlim()[1], ax1.get_ylim()[1]), arrowprops=dict(facecolor='black', shrink=0.05),
        xytext=(-42, -18), textcoords='offset points')

    with open(f"{DUCKDB_CLICKBENCH_IGNITION}/clickbench-t1-results.json", 'r') as file:
        ignition_t1_data = json.load(file)

    with open(f"{DUCKDB_CLICKBENCH_PARQUET}/clickbench-t1-results.json", 'r') as file:
        parquet_t1_data = json.load(file)

    with open(f"{DUCKDB_CLICKBENCH_RAW}/clickbench-t1-results.json", 'r') as file:
        raw_t1_data = json.load(file)

    clickbench_tuple_count = 99997497
    
    ignition_scan_t1 = [
        np.median(list(y for [x, y] in ignition_t1_data[q])) for q in CLICKBENCH_QS]
    parquet_scan_t1 = [
        np.median(list(y for [x, y] in parquet_t1_data[q])) for q in CLICKBENCH_QS]
    raw_scan_t1 = [
        np.median(list(y for [x, y] in raw_t1_data[q])) for q in CLICKBENCH_QS]
    ignition_query_t1 = [
        np.median(list(x - y for [x, y] in ignition_t1_data[q])) for q in CLICKBENCH_QS]
    parquet_query_t1 = [
        np.median(list(x - y for [x, y] in parquet_t1_data[q])) for q in CLICKBENCH_QS]
    raw_query_t1 = [
        np.median(list(x - y for [x, y] in raw_t1_data[q])) for q in CLICKBENCH_QS]

    ignition_thpt = clickbench_tuple_count / 1000000 / (sum(ignition_scan_t1) + sum(ignition_query_t1))
    parquet_thpt = clickbench_tuple_count / 1000000 / (sum(parquet_scan_t1) + sum(parquet_query_t1))
    raw_thpt = clickbench_tuple_count / 1000000 / (sum(raw_scan_t1) + sum(raw_query_t1))

    labels = ['DuckDB', 'Parquet (DuckDB)', 'AnyBlox (DuckDB)']
    sizes = 27880202753 / np.array([
        23884738560 - 12010401792,
        1559023870 + 1734778686 + 2157040318 + 1954124606,
        1299557751 + 1463737454 + 1723556603 + 1586653422])
    thpt = [raw_thpt, parquet_thpt, ignition_thpt]
    colors = [COL_DUCKDB_SECONDARY, COL_PARQUET_PRIMARY, COL_IGNITION_PRIMARY]
    vert_off = [-15, -15, 5]

    ax2.scatter(sizes, thpt, c=colors)
    ax2.grid(linestyle='--')
    ax2.set_xlim(xmin=1)
    ax2.set_ylim(ymin=0, ymax=0.25)
    ax2.set_xlabel('compression rate')
    ax2.set_title('ClickBench strings')
    #ax2.set_ylabel('throughput (Mtuples)')

    for i, txt in enumerate(labels):
        ax2.annotate(txt, (sizes[i], thpt[i]), color=colors[i], fontsize=12,
        xytext=(-6*len(txt), vert_off[i]), textcoords='offset points',
        horizontalalignment='left', verticalalignment='bottom')

    ax2.annotate('better', (ax2.get_xlim()[1], ax2.get_ylim()[1]), arrowprops=dict(facecolor='black', shrink=0.05),
        xytext=(-42, -18), textcoords='offset points')
    
    figure.tight_layout()
    figure.savefig("plot2d.pdf")


# DUCKDB - TPC-H

for threads in [2, 4, 8, 16, 32, 64]:
    plot_tpch_duckdb_multithread(threads)

plot_tpch_duckdb_single_thread()

# SPARK - TPC-H
plot_tpch_spark_multithread(32)

plot_tpch_spark_single_thread()

# DUCKDB - CLICKBENCH

for threads in [2, 4, 8, 16, 32, 64]:
    plot_clickbench_duckdb_multithread(threads)

plot_clickbench_duckdb_single_thread()

# Combined

plot_totals()
plot_2d()

query_tuple_counts = np.genfromtxt("tpch-query-sizes.csv", delimiter=',', skip_header=1)[0:,1]
tpch_tuple_count = sum(query_tuple_counts)