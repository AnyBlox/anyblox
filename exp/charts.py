import json
import matplotlib.pyplot as plt
import matplotlib.patheffects as pe
import numpy as np
import sys

PX_PER_IN = 250
DUCKDB_TPCH_ANYBLOX = "./duckdb-results-s20"
DUCKDB_TPCH_PARQUET = "./duckdb-results-parquet-snappy-s20"
DUCKDB_TPCH_PARQUET_UNCOMPRESSED = "./duckdb-results-parquet-uncompressed-s20"
DUCKDB_TPCH_PARQUET_ZSTD = "./duckdb-results-parquet-zstd-s20"
DUCKDB_TPCH_PARQUET_GZIP = "./duckdb-results-parquet-gzip-s20"
DUCKDB_TPCH_RAW = "./duckdb-results-raw-s20"

DUCKDB_CLICKBENCH_ANYBLOX = "./duckdb-results-clickbench"
DUCKDB_CLICKBENCH_PARQUET = "./duckdb-results-parquet-snappy-clickbench"
DUCKDB_CLICKBENCH_PARQUET_UNCOMPRESSED = "./duckdb-results-parquet-uncompressed-clickbench"
DUCKDB_CLICKBENCH_PARQUET_ZSTD = "./duckdb-results-parquet-zstd-clickbench"
DUCKDB_CLICKBENCH_PARQUET_GZIP = "./duckdb-results-parquet-gzip-clickbench"
DUCKDB_CLICKBENCH_RAW = "./duckdb-results-raw-clickbench"

SPARK_TPCH_ANYBLOX = "./spark-results-anyblox-s20"
SPARK_TPCH_PARQUET = "./spark-results-parquet-s20"
SPARK_CLICKBENCH_ANYBLOX = "./spark-results-clickbench-t1"
SPARK_CLICKBENCH_PARQUET = "./spark-results-clickbench-parquet-t1"

COL_NATIVE_PRIMARY = '#02CC78'
COL_NATIVE_SECONDARY = '#01663C'
COL_DUCKDB_PRIMARY = '#FFF100'
COL_DUCKDB_SECONDARY = '#807900'
COL_DUCKDB_TERTIARY = '#FFF766'
COL_ANYBLOX_PRIMARY = '#CC0256'
COL_ANYBLOX_SECONDARY = '#66012B'
COL_ANYBLOX_TERTIARY = '#E681AB'
COL_PARQUET_PRIMARY = '#069AF3'
COL_PARQUET_SECONDARY = '#034D7A'
COL_PARQUET_TERTIARY = '#6AC2F8'
COL_MISSING_NATIVE = '#888888'

TPCH_QS = ["1", "3", "4", "5", "6", "7", "8", "9", "10",
      "12", "14", "15", "17", "18", "19", "20", "21"]
TPCH_QS_SELECTED = ["1", "10", "12", "21"]
tpch_xs = np.arange(len(TPCH_QS))
tpch_xs_selected = np.arange(len(TPCH_QS_SELECTED))

CLICKBENCH_QS = ["6", "11", "12", "13", "14", "15", "17", "18", "19", "21", "22", "23", "24", "26", "27", "29", "31", "32", "33", "34", "37", "38", "39", "40"]
CLICKBENCH_SPARK_QS = ["6", "11", "12", "13", "14", "15", "17", "18", "19", "21", "22", "23", "26", "27", "31", "32", "33", "34"]
CLICKBENCH_QS_SELECTED = ["24", "29"]
clickbench_xs = np.arange(len(CLICKBENCH_QS))
clickbench_xs_selected = np.arange(len(CLICKBENCH_QS_SELECTED))

FROM_MS = 1000
FROM_μS = 1000000
FROM_NS = 1000000000
FROM_B = 1000000

LEGEND_FONT_SIZE = 16
TITLES_FONT_SIZE = 18
LABELS_FONT_SIZE = 16
ANNOTATIONS_FONT_SIZE = 16

plt.rcParams["font.family"] = "Libertinus Sans"
plt.rcParams["xtick.labelsize"] = LABELS_FONT_SIZE
plt.rcParams["ytick.labelsize"] = LABELS_FONT_SIZE
plt.rcParams["axes.labelsize"] = LABELS_FONT_SIZE
plt.rcParams["axes.titlesize"] = TITLES_FONT_SIZE
plt.rcParams["legend.fontsize"] = LEGEND_FONT_SIZE

def plot_totals_once(ax, data, colors_primary, colors_secondary, qs, no_native=False, ratio=1):
    scans = [sum([np.median(list(y / ratio for [x, y] in d[q])) for q in qs]) for d in data]
    queries = [sum([np.median(list(x / ratio - y / ratio for [x, y] in d[q])) for q in qs]) for d in data]
    ratios = [scan / (scan + query) for (scan, query) in zip(scans, queries)]

    width = 1 / (len(data) + 1)
    xs = np.arange(1)

    if no_native:
        artificial_scans = [0.0] + scans
        artificial_queries = [0.0] + queries
        artificial_colors_secondary = [COL_MISSING_NATIVE] + colors_secondary
        artificial_colors_primary = [COL_MISSING_NATIVE] + colors_primary
        hatches = ['/'] + ([''] * len(scans))
        artificial_rects_scans = [ax.bar(xs + i * width, [scan], width * 0.9, color=artificial_colors_secondary[i], hatch=hatches[i]) for i, scan in enumerate(artificial_scans)]
        artificial_rects_queries = [ax.bar(xs + i * width, [query], width * 0.9, color=artificial_colors_primary[i], hatch=hatches[i], bottom=artificial_scans[i]) for i, query in enumerate(artificial_queries)]

        rects_scans = artificial_rects_scans[1:]
        rects_queries = artificial_rects_queries[1:]
    else:
        rects_scans = [ax.bar(xs + i * width, [scan], width * 0.9, color=colors_secondary[i]) for i, scan in enumerate(scans)]
        rects_queries = [ax.bar(xs + i * width, [query], width * 0.9, color=colors_primary[i], bottom=scans[i]) for i, query in enumerate(queries)]
    
    ax.set_ylim(ymin=0)
    ax.grid(linestyle='--', axis='y')
    ax.set_xticks([])
    
    for i, scan in enumerate(scans):
        off = 1 if no_native else 0
        txt = ax.annotate(f'{ratios[i]*100:.0f}%', (xs + (i + off) * width, scan), color="white", fontsize=ANNOTATIONS_FONT_SIZE,
            xytext=(-10, 0), textcoords='offset points',
            horizontalalignment='left', verticalalignment='bottom')
        txt.set_path_effects([pe.withStroke(linewidth=3, foreground=colors_secondary[i])]),
    
    if no_native:
        ax.text(0, ax.get_ylim()[1] / 2, 'no native table', color = 'black', ha = 'center', va = 'center', rotation = 90.0, size=22) 

    return (rects_scans, rects_queries)


def plot_selected_once(ax, data, colors_primary, colors_secondary, qs, ratio=1):
    scans = [[np.median(list(y / ratio for [x, y] in d[q])) for q in qs] for d in data]
    queries = [[np.median(list(x / ratio - y / ratio for [x, y] in d[q])) for q in qs] for d in data]

    width = 1 / (len(data) + 1)
    xs = np.arange(len(qs))

    rects_scans = [ax.bar(xs + i * width, scan, 0.9 * width , color=colors_secondary[i]) for i, scan in enumerate(scans)]
    rects_queries = [ax.bar(xs + i * width, query, 0.9 * width, color=colors_primary[i], bottom=scans[i]) for i, query in enumerate(queries)]
    
    ax.set_ylim(ymin=0)
    ax.set_xticks(xs + (len(data) - 1) * width / 2)
    ax.grid(linestyle='--', axis='y')
    
    for i, scan in enumerate(scans):
        for j, xtick in enumerate(xs):
            x = xtick + i * width
            y = scans[i][j]
            ratio = y / (y + queries[i][j])
            txt = ax.annotate(f'{ratio*100:.0f}%', (x, y), color="white", fontsize=ANNOTATIONS_FONT_SIZE,
                xytext=(-10, 0), textcoords='offset points',
                horizontalalignment='left', verticalalignment='bottom')
            txt.set_path_effects([pe.withStroke(linewidth=3, foreground=colors_secondary[i])]),

    return (rects_scans, rects_queries)


def plot_main():
    # figure, (ax1, ax2, ax3, ax4, ax5) = plt.subplots(ncols=5, gridspec_kw={'width_ratios': [2, 2, 2, 3, 3]})
    figure_total, (ax1, ax2, ax3, ax4) = plt.subplots(ncols=4)

    with open(f"{DUCKDB_TPCH_ANYBLOX}/tpch-t1-results.json", 'r') as file:
        anyblox_t1_data = json.load(file)

    with open(f"{DUCKDB_TPCH_PARQUET}/tpch-t1-results.json", 'r') as file:
        parquet_t1_data = json.load(file)

    with open(f"{DUCKDB_TPCH_RAW}/tpch-t1-results.json", 'r') as file:
        raw_t1_data = json.load(file)

    plot_totals_once(ax1,
        [raw_t1_data, anyblox_t1_data, parquet_t1_data],
        [COL_DUCKDB_PRIMARY, COL_ANYBLOX_PRIMARY, COL_PARQUET_PRIMARY],
        [COL_DUCKDB_SECONDARY, COL_ANYBLOX_SECONDARY, COL_PARQUET_SECONDARY],
        TPCH_QS
    )
    ax1.set_title('TPC-H (DuckDB)')
    ax1.set_ylabel('latency (s)')
    #ax1.set_xticks([1/4])
    #ax1.set_xticklabels(["total"])

    with open(f"{DUCKDB_CLICKBENCH_ANYBLOX}/clickbench-t1-results.json", 'r') as file:
        anyblox_t1_data = json.load(file)

    with open(f"{DUCKDB_CLICKBENCH_PARQUET}/clickbench-t1-results.json", 'r') as file:
        parquet_t1_data = json.load(file)

    with open(f"{DUCKDB_CLICKBENCH_RAW}/clickbench-t1-results.json", 'r') as file:
        raw_t1_data = json.load(file)

    (scan_rects, query_rects) = plot_totals_once(ax2,
        [raw_t1_data, anyblox_t1_data, parquet_t1_data],
        [COL_DUCKDB_PRIMARY, COL_ANYBLOX_PRIMARY, COL_PARQUET_PRIMARY],
        [COL_DUCKDB_SECONDARY, COL_ANYBLOX_SECONDARY, COL_PARQUET_SECONDARY],
        CLICKBENCH_QS
    )
    ax2.set_title('ClickBench (DuckDB)')
    #ax2.set_xticks([1/4])
    #ax2.set_xticklabels(["total"])

    with open(f"{SPARK_TPCH_ANYBLOX}/tpch-t1-results.json", 'r') as file:
       anyblox_t1_data = json.load(file)

    with open(f"{SPARK_TPCH_PARQUET}/tpch-t1-results.json", 'r') as file:
       parquet_t1_data = json.load(file)

    plot_totals_once(ax3,
       [anyblox_t1_data, parquet_t1_data],
       [COL_ANYBLOX_PRIMARY, COL_PARQUET_PRIMARY],
       [COL_ANYBLOX_SECONDARY, COL_PARQUET_SECONDARY],
       TPCH_QS,
       ratio=1000,
       no_native=True
    )
    ax3.set_title('TPC-H (Spark)')
    #ax3.set_xticks([1/6])
    #ax3.set_xticklabels(["total"])

    with open(f"{SPARK_CLICKBENCH_ANYBLOX}/clickbench-t1-results.json", 'r') as file:
       anyblox_t1_data = json.load(file)
    with open(f"{SPARK_CLICKBENCH_PARQUET}/clickbench-t1-results.json", 'r') as file:
       parquet_t1_data = json.load(file)

    plot_totals_once(ax4,
       [anyblox_t1_data, parquet_t1_data],
       [COL_ANYBLOX_PRIMARY, COL_PARQUET_PRIMARY],
       [COL_ANYBLOX_SECONDARY, COL_PARQUET_SECONDARY],
       CLICKBENCH_SPARK_QS,
       ratio=1000,
       no_native=True
    )

    ax4.set_title('ClickBench (Spark)†')
    #ax4.set_xticks([1/6])
    #ax4.set_xticklabels(["total"])

    def shrink(ax):
        box = ax.get_position()
        ax.set_position([box.x0, box.y0, box.width, box.height * 0.85])

    legend_rects = [item for sublist in [[s, r] for s, r in zip(scan_rects, query_rects)] for item in sublist]
    figure_total.legend(legend_rects, [
                'native table scan', 'native table rest', 'anyblox scan', 'anyblox rest', 'parquet scan', 'parquet rest'],
                loc='upper center', ncol=6)

    figure_total.set_size_inches(2 * 1920 / PX_PER_IN, 1080 / PX_PER_IN)
    figure_total.tight_layout()
    shrink(ax1)
    shrink(ax2)
    shrink(ax3)
    shrink(ax4)
    figure_total.savefig("totals.pdf")
    
    # SELECTED
    figure_selected, (ax1, ax2) = plt.subplots(ncols=2, gridspec_kw={'width_ratios': [3, 2]})

    with open(f"{DUCKDB_TPCH_ANYBLOX}/tpch-t1-results.json", 'r') as file:
        anyblox_t1_data = json.load(file)

    with open(f"{DUCKDB_TPCH_PARQUET}/tpch-t1-results.json", 'r') as file:
        parquet_t1_data = json.load(file)

    with open(f"{DUCKDB_TPCH_RAW}/tpch-t1-results.json", 'r') as file:
        raw_t1_data = json.load(file)

    plot_selected_once(ax1,
        [raw_t1_data, anyblox_t1_data, parquet_t1_data],
        [COL_DUCKDB_PRIMARY, COL_ANYBLOX_PRIMARY, COL_PARQUET_PRIMARY],
        [COL_DUCKDB_SECONDARY, COL_ANYBLOX_SECONDARY, COL_PARQUET_SECONDARY],
        TPCH_QS_SELECTED
    )
    ax1.set_title('TPC-H (selected)')
    ax1.set_xticklabels([f"Q{q}" for q in TPCH_QS_SELECTED])
    ax1.set_ylabel('latency (s)')

    with open(f"{DUCKDB_CLICKBENCH_ANYBLOX}/clickbench-t1-results.json", 'r') as file:
        anyblox_t1_data = json.load(file)

    with open(f"{DUCKDB_CLICKBENCH_PARQUET}/clickbench-t1-results.json", 'r') as file:
        parquet_t1_data = json.load(file)

    with open(f"{DUCKDB_CLICKBENCH_RAW}/clickbench-t1-results.json", 'r') as file:
        raw_t1_data = json.load(file)

    (scan_rects, query_rects) = plot_selected_once(ax2,
        [raw_t1_data, anyblox_t1_data, parquet_t1_data],
        [COL_DUCKDB_PRIMARY, COL_ANYBLOX_PRIMARY, COL_PARQUET_PRIMARY],
        [COL_DUCKDB_SECONDARY, COL_ANYBLOX_SECONDARY, COL_PARQUET_SECONDARY],
        CLICKBENCH_QS_SELECTED
    )

    ax2.set_title('ClickBench (selected)')
    ax2.set_xticklabels([f"Q{q}" for q in CLICKBENCH_QS_SELECTED])

    legend_rects = [item for sublist in [[s, r] for s, r in zip(scan_rects, query_rects)] for item in sublist]

    def shrink(ax):
        box = ax.get_position()
        ax.set_position([box.x0, box.y0, box.width, box.height * 0.75])

    figure_selected.legend(legend_rects, [
                'native table scan', 'native table rest', 'anyblox scan', 'anyblox rest', 'parquet scan', 'parquet rest'],
                loc='upper center', ncol=3)

    figure_selected.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)
    figure_selected.tight_layout()
    shrink(ax1)
    shrink(ax2)
    figure_selected.savefig("selected.pdf")


def plot_scatter_once(ax, data, sizes, labels, colors_primary, hori_off, vert_off, qs, tuple_count, ratio=1, ymax=None):
    scans = [[np.median(list(y / ratio for [x, y] in d[q])) for q in qs] for d in data]
    queries = [[np.median(list(x / ratio - y / ratio for [x, y] in d[q])) for q in qs] for d in data]
    thpts = [tuple_count / 1000000 / (sum(scan) + sum(query)) for (scan, query) in zip(scans, queries)]

    print(list(zip(thpts, labels)))

    ax.scatter(sizes, thpts, c=colors_primary)
    ax.grid(linestyle='--')
    ax.set_xlim(xmin=1)
    ax.set_ylim(ymin=0)
    ax.set_xlabel('compression rate')
    if ymax:
        ax.set_ylim(ymax=ymax)
    #ax2.set_ylabel('throughput (Mtuples)')

    for i, txt in enumerate(labels):
        ax.annotate(txt, (sizes[i], thpts[i]), color=colors_primary[i], fontsize=ANNOTATIONS_FONT_SIZE,
            #arrowprops=dict(facecolor=colors_primary[i], arrowstyle='simple'),
            xytext=(hori_off[i], vert_off[i]), textcoords='offset points',
            horizontalalignment='left', verticalalignment='bottom')

    ax.annotate('better', (ax.get_xlim()[1], ax.get_ylim()[1]), arrowprops=dict(facecolor='black', shrink=0.05),
        xytext=(-36, -18), textcoords='offset points')


def plot_scatter_impls():
    figure, (ax1, ax2) = plt.subplots(ncols=2)
    figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)

    with open(f"{DUCKDB_TPCH_ANYBLOX}/tpch-t32-results.json", 'r') as file:
        anyblox_t1_data = json.load(file)
    with open(f"{DUCKDB_TPCH_PARQUET}/tpch-t32-results.json", 'r') as file:
        parquet_t1_data = json.load(file)
    with open(f"{DUCKDB_TPCH_PARQUET_UNCOMPRESSED}/tpch-t1-results.json", 'r') as file:
        parquet_uncompressed_t1_data = json.load(file)
    with open(f"{DUCKDB_TPCH_PARQUET_ZSTD}/tpch-t1-results.json", 'r') as file:
        parquet_zstd_t1_data = json.load(file)
    with open(f"{DUCKDB_TPCH_PARQUET_GZIP}/tpch-t1-results.json", 'r') as file:
        parquet_gzip_t1_data = json.load(file)
    with open(f"{DUCKDB_TPCH_RAW}/tpch-t32-results.json", 'r') as file:
        raw_t1_data = json.load(file)

    query_tuple_counts = np.genfromtxt("tpch-query-sizes.csv", delimiter=',', skip_header=1)[0:,1]
    tpch_tuple_count = sum(query_tuple_counts)

    data = [raw_t1_data, parquet_t1_data, parquet_uncompressed_t1_data, parquet_zstd_t1_data, parquet_gzip_t1_data, anyblox_t1_data]
    labels = ['Native', 'Parquet (Snappy)', 'Parquet (None)', 'Parquet (zstd)', 'Parquet (gzip)', 'AnyBlox (Vortex)']
    sizes = 15685400985 / np.array([11307593728 - 3793235968, 5497795005, 11364351974, 3585494701, 3424338161, 3883788826])
    colors = [COL_NATIVE_PRIMARY, COL_PARQUET_PRIMARY, COL_PARQUET_PRIMARY, COL_PARQUET_PRIMARY, COL_PARQUET_PRIMARY, COL_ANYBLOX_PRIMARY]
    hori_off = [-60, 0, 0, -100, -100, -120]
    vert_off = [-18, 6, 6, -18, -18, -18]

    plot_scatter_once(ax1, data, sizes, labels, colors, hori_off, vert_off, TPCH_QS, tpch_tuple_count)
    ax1.set_ylabel('throughput (Mtuples)')
    ax1.set_title('TPC-H lineitem')

    with open(f"{DUCKDB_CLICKBENCH_ANYBLOX}/clickbench-t1-results.json", 'r') as file:
        anyblox_t1_data = json.load(file)
    with open(f"{DUCKDB_CLICKBENCH_PARQUET}/clickbench-t1-results.json", 'r') as file:
        parquet_t1_data = json.load(file)
    with open(f"{DUCKDB_CLICKBENCH_PARQUET_UNCOMPRESSED}/clickbench-t1-results.json", 'r') as file:
        parquet_uncompressed_t1_data = json.load(file)
    with open(f"{DUCKDB_CLICKBENCH_PARQUET_ZSTD}/clickbench-t1-results.json", 'r') as file:
        parquet_zstd_t1_data = json.load(file)
    with open(f"{DUCKDB_CLICKBENCH_PARQUET_GZIP}/clickbench-t1-results.json", 'r') as file:
        parquet_gzip_t1_data = json.load(file)
    with open(f"{DUCKDB_CLICKBENCH_RAW}/clickbench-t1-results.json", 'r') as file:
        raw_t1_data = json.load(file)

    clickbench_tuple_count = 99997497

    data = [raw_t1_data, parquet_t1_data, 
    parquet_uncompressed_t1_data, parquet_zstd_t1_data, parquet_gzip_t1_data, 
    anyblox_t1_data]
    labels = ['Native', 'Parquet (Snappy)', 
        'Parquet (None)', 'Parquet (zstd)', 'Parquet (gzip)', 
        'AnyBlox (Vortex)']
    sizes = 27880202753 / np.array([
        23884738560 - 12010401792,
        1298789948 + 1459277781 + 1724359849 + 1591886904,
        2438165601 + 2733948222 + 3276826779 + 3009951175,
        757506686 + 833199262 + 932574333 + 891460430,
        768458232 + 867045466 + 1027745831 + 3009951175,
        1559023870 + 1734778686 + 2157040318 + 947629660])
    colors = [COL_NATIVE_PRIMARY, COL_PARQUET_PRIMARY, 
        COL_PARQUET_PRIMARY, COL_PARQUET_PRIMARY, COL_PARQUET_PRIMARY, 
        COL_ANYBLOX_PRIMARY]
    vert_off = [6, -20, 6, -20, -18, 6]
    hori_off = [0, -110, -100, -80, -100, 0]

    plot_scatter_once(ax2, data, sizes, labels, colors, hori_off, vert_off, CLICKBENCH_QS, clickbench_tuple_count, ymax=0.275)

    ax2.set_xlabel('compression rate')
    ax2.set_title('ClickBench strings')
    
    figure.tight_layout()
    figure.savefig("scatter_impls.pdf")


def plot_selected():
    figure, (ax1, ax2) = plt.subplots(ncols=2)

    with open(f"{DUCKDB_TPCH_ANYBLOX}/tpch-t1-results.json", 'r') as file:
        anyblox_t1_data = json.load(file)

    with open(f"{DUCKDB_TPCH_PARQUET}/tpch-t1-results.json", 'r') as file:
        parquet_t1_data = json.load(file)

    with open(f"{DUCKDB_TPCH_RAW}/tpch-t1-results.json", 'r') as file:
        raw_t1_data = json.load(file)
        
    plot_selected_once(ax1,
        [raw_t1_data, anyblox_t1_data, parquet_t1_data],
        [COL_DUCKDB_PRIMARY, COL_ANYBLOX_PRIMARY, COL_PARQUET_PRIMARY],
        [COL_DUCKDB_SECONDARY, COL_ANYBLOX_SECONDARY, COL_PARQUET_SECONDARY],
        TPCH_QS_SELECTED
    )

    ax1.set_ylabel('latency (s)')
    ax1.set_title('TPC-H (sf=20) DuckDB')
    ax1.set_xticklabels([f"Q{q}" for q in TPCH_QS_SELECTED])

    with open(f"{DUCKDB_CLICKBENCH_ANYBLOX}/clickbench-t1-results.json", 'r') as file:
        anyblox_t1_data = json.load(file)

    with open(f"{DUCKDB_CLICKBENCH_PARQUET}/clickbench-t1-results.json", 'r') as file:
        parquet_t1_data = json.load(file)

    with open(f"{DUCKDB_CLICKBENCH_RAW}/clickbench-t1-results.json", 'r') as file:
        raw_t1_data = json.load(file)

    print(parquet_t1_data["29"])

    (scan_rects, query_rects) = plot_selected_once(ax2,
        [raw_t1_data, anyblox_t1_data, parquet_t1_data],
        [COL_DUCKDB_PRIMARY, COL_ANYBLOX_PRIMARY, COL_PARQUET_PRIMARY],
        [COL_DUCKDB_SECONDARY, COL_ANYBLOX_SECONDARY, COL_PARQUET_SECONDARY],
        CLICKBENCH_QS_SELECTED
    )

    ax2.set_ylabel('latency (s)')
    ax2.set_title('ClickBench DuckDB')
    ax2.set_xticklabels([f"Q{q}" for q in CLICKBENCH_QS_SELECTED])
    
    legend_rects = [item for sublist in [[s, r] for s, r in zip(scan_rects, query_rects)] for item in sublist]

    figure.legend(legend_rects, [
                'table scan', 'table rest', 'anyblox scan', 'anyblox rest', 'parquet scan', 'parquet rest'],
                fontsize=LEGEND_FONT_SIZE, loc='center right')

    figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)
    figure.tight_layout()
    figure.savefig("selected.pdf")


plot_main()
plot_scatter_impls()

query_tuple_counts = np.genfromtxt("tpch-query-sizes.csv", delimiter=',', skip_header=1)[0:,1]
tpch_tuple_count = sum(query_tuple_counts)
