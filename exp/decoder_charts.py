import csv
import matplotlib.pyplot as plt
import numpy as np
import sys


def non_nan(x):
    return x[~np.isnan(x)]


PX_PER_IN = 250
QUARTILES = [0, 0.25, 0.5, 0.75, 1]
COL_IGNITION_PRIMARY = '#CC0256'
COL_IGNITION_SECONDARY = '#66012B'
COL_NATIVE_PRIMARY = '#02CC78'
COL_NATIVE_SECONDARY = '#01663C'
COMMON_GOVERNMENT_DIR = './decoders/CommonGovernment'
TAXPAYER_DIR = './decoders/Taxpayer'
TAXPAYER_BATCHSIZE_DIR = './decoders/TaxpayerBatchSize'
NATIVE_RLE_PATH = 'native_rle_CommonGovernment_45_rle_EXTENSION_t1_b9000000_utrue.csv'
NATIVE_RLE_SIMD_PATH = 'native_rle_simd_CommonGovernment_45_rle_EXTENSION_t1_b200000_utrue.csv'
NATIVE_RLE_SIMD_AVX2_PATH = 'native_rle_simd_avx2_CommonGovernment_45_rle_EXTENSION_t1_b200000_utrue.csv'
NATIVE_RLE_SIMD_SSE2_PATH = 'native_rle_simd_sse2_CommonGovernment_45_rle_EXTENSION_t1_b200000_utrue.csv'
WASM_RLE_PATH = 'wasm_rle_CommonGovernment_45_rle_EXTENSION_t1_b9000000_utrue.csv'
WASM_RLE_SIMD_PATH = 'wasm_rle_simd_CommonGovernment_45_rle_EXTENSION_t1_b200000_utrue.csv'
WASM_RLE_SIMD_STATELESS_PATH = 'wasm_rle_simd_stateless_CommonGovernment_45_rle_EXTENSION_t1_b200000_utrue.csv'
NATIVE_TAXPAYER_PATH = 'native_taxpayer_fsst_Taxpayer_1.fsst_SELF-CONTAINED_t1_b30000_ufalse.csv'
WASM_TAXPAYER_PATH = 'wasm_taxpayer_fsst_Taxpayer_1.fsst_SELF-CONTAINED_t1_b30000_ufalse.csv'
TO_MS = 1000000
TO_MB = 1000000
RLE_ROW_COUNT = 141123827
TAXPAYER_BATCH_SIZES = [
    #100, 200, 300, 400, 500, 600, 700, 800, 900,
    1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000,
    10000, 20000, 30000, 40000, 50000, 60000, 70000, 80000, 90000,
    100000, 200000, 300000, 400000, 500000, 600000, 700000, 800000, 900000,
    1000000, 2000000, 3000000, 4000000, 5000000, 6000000, 7000000, 8000000, 9000000,
    10000000, 20000000, 30000000
    ]
RLE_BATCH_SIZES = [
    #100, 200, 300, 400, 500, 600, 700, 800, 900,
    1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000,
    10000, 20000, 30000, 40000, 50000, 60000, 70000, 80000, 90000,
    100000, 200000, 300000, 400000, 500000, 600000, 700000, 800000, 900000,
    1000000, 2000000, 3000000, 4000000, 5000000, 6000000, 7000000, 8000000, 9000000,
    10000000, 20000000, 30000000, 40000000, 50000000, 60000000, 70000000, 80000000, 90000000,
    141123827
    ]
TAXPAYER_DATA_SIZE = 1774415124
RLE_DATA_SIZE = 215142016

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

def plot_native_against_wasm(ax, title, native, wasm, row_count):
    native_csvs = [np.genfromtxt(n['path'], delimiter=',', skip_header=1) for n in native]
    wasm_csvs = [np.genfromtxt(w['path'], delimiter=',', skip_header=1) for w in wasm]

    native_totals = [row_count / non_nan(native_csv[0:,0] + native_csv[0:,1]) for native_csv in native_csvs]
    wasm_totals = [row_count / non_nan(wasm_csv[0:,0] + wasm_csv[0:,1]) for wasm_csv in wasm_csvs]
    base_total = max(max(x * row_count) for x in native_totals + wasm_totals)
    base_thpt = min(min(x) for x in native_totals + wasm_totals)
    xs = np.arange(len(native_totals) + len(wasm_totals))
    medians = [np.median(x) for x in native_totals + wasm_totals]
    colors = [COL_NATIVE_PRIMARY for x in native_csvs] + [COL_IGNITION_PRIMARY for x in wasm_csvs]

    native_labels = [n['label'] for n in native]
    wasm_labels = [w['label'] for w in wasm]
    print(medians)

    objs = [(m, l, c) for (m, c, l) in zip(medians, colors, native_labels + wasm_labels)]
    objs.sort()

    bplot = ax.bar(xs, [m for (m, _, _) in objs], color=[c for (_, _, c) in objs])
    ax.set_ylim(ymin=0)
    ax.set_title(title)
    ax.set_xticks(xs)
    ax.set_xticklabels([l for (_, l, _) in objs])
    ax.grid(linestyle='--', axis='y')

    return bplot


def plot_isolated_decoders():
    figure, (ax1, ax2) = plt.subplots(ncols=2, gridspec_kw={'width_ratios': [3, 1]})

    plot_native_against_wasm(
        ax1,
        'REE',
        [
            {'path': f'{COMMON_GOVERNMENT_DIR}/{NATIVE_RLE_PATH}', 'label': 'native'},
            {'path': f'{COMMON_GOVERNMENT_DIR}/{NATIVE_RLE_SIMD_SSE2_PATH}', 'label': 'native\n(SSE2)'},
            {'path': f'{COMMON_GOVERNMENT_DIR}/{NATIVE_RLE_SIMD_AVX2_PATH}', 'label': 'native\n(AVX2)'},
            {'path': f'{COMMON_GOVERNMENT_DIR}/{NATIVE_RLE_SIMD_PATH}', 'label': 'native\n(AVX512)'}
        ],
        [
            {'path': f'{COMMON_GOVERNMENT_DIR}/{WASM_RLE_PATH}', 'label': 'wasm'},
            {'path': f'{COMMON_GOVERNMENT_DIR}/{WASM_RLE_SIMD_PATH}', 'label': 'wasm\n(V128)'},
            #{'path': f'{COMMON_GOVERNMENT_DIR}/{WASM_RLE_SIMD_STATELESS_PATH}', 'label': 'wasm (V128, no state)'}
        ],
        RLE_ROW_COUNT,
    )
    bplot = plot_native_against_wasm(
        ax2,
        'FSST',
        [
            {'path': f'{TAXPAYER_DIR}/{NATIVE_TAXPAYER_PATH}', 'label': 'native'},
        ],
        [
            {'path': f'{TAXPAYER_DIR}/{WASM_TAXPAYER_PATH}', 'label': 'wasm'},
            #{'path': f'{COMMON_GOVERNMENT_DIR}/{WASM_RLE_SIMD_STATELESS_PATH}', 'label': 'wasm (V128, no state)'}
        ],
        TAXPAYER_DATA_SIZE
    )
    
    ax1.set_ylabel('gigatuples/s')
    #figure.legend([bplot[0], bplot[1]], ['native', 'wasm'], fontsize=LEGEND_FONT_SIZE)
    figure.set_size_inches(1920 / PX_PER_IN, (1080 / 2) / PX_PER_IN)
    figure.tight_layout()
    figure.savefig('decoder_charts.pdf')


def plot_batchsizes():
    MARKER_SIZE=5
    taxpayer_data = [np.genfromtxt(f"{TAXPAYER_BATCHSIZE_DIR}/wasm_taxpayer_fsst_Taxpayer_1.fsst_SELF-CONTAINED_t1_b{b}_ufalse.csv", delimiter=',', skip_header=1) for b in TAXPAYER_BATCH_SIZES]
    taxpayer_mean_times = [np.mean(d[0:,1] / TO_MS) for d in taxpayer_data]
    taxpayer_min_time = min(taxpayer_mean_times)
    taxpayer_normalized_times = np.array(taxpayer_mean_times) / taxpayer_min_time
    rle_data = [np.genfromtxt(f"{COMMON_GOVERNMENT_DIR}/wasm_rle_CommonGovernment_45_rle_EXTENSION_t1_b{b}_utrue.csv", delimiter=',', skip_header=1) for b in RLE_BATCH_SIZES]
    rle_mean_times = [np.mean(d[0:,1] / TO_MS) for d in rle_data]
    rle_min_time = min(rle_mean_times)
    rle_normalized_times = np.array(rle_mean_times) / rle_min_time
    rle_simd_data = [np.genfromtxt(f"{COMMON_GOVERNMENT_DIR}/wasm_rle_simd_CommonGovernment_45_rle_EXTENSION_t1_b{b}_utrue.csv", delimiter=',', skip_header=1) for b in RLE_BATCH_SIZES]
    rle_simd_mean_times = [np.mean(d[0:,1] / TO_MS) for d in rle_simd_data]
    rle_simd_min_time = min(rle_simd_mean_times)
    rle_simd_normalized_times = np.array(rle_simd_mean_times) / rle_simd_min_time

    taxpayer_peak_memory = [(np.max(d[0:,2]) - TAXPAYER_DATA_SIZE) / TO_MB for d in taxpayer_data]
    rle_peak_memory = [(np.max(d[0:,2]) - RLE_DATA_SIZE) / TO_MB for d in rle_data]
    rle_simd_peak_memory = [(np.max(d[0:,2]) - RLE_DATA_SIZE) / TO_MB for d in rle_simd_data]
    
    figure, (ax1, ax2, ax3) = plt.subplots(nrows=3)
    fsst_plot, = ax1.plot(TAXPAYER_BATCH_SIZES, taxpayer_normalized_times, '+-', markersize=MARKER_SIZE, color=COL_IGNITION_PRIMARY)
    ree_plot, = ax2.plot(RLE_BATCH_SIZES, rle_normalized_times, '+-', markersize=MARKER_SIZE, color=COL_IGNITION_PRIMARY)
    ree_simd_plot, = ax3.plot(RLE_BATCH_SIZES, rle_simd_normalized_times, '+-', markersize=MARKER_SIZE, color=COL_IGNITION_PRIMARY)
    ax2.set_ylabel('normalized time')
    ax1.set_ylim(ymin=0.99)
    ax2.set_ylim(ymin=0.99)
    ax3.set_ylim(ymin=0.99)
    ax1.grid(linestyle='--')
    ax2.grid(linestyle='--')
    ax3.grid(linestyle='--')
    ax1.set_xscale('log')
    ax2.set_xscale('log')
    ax3.set_xscale('log')
    ax3.set_xlabel('batch size (tuples)')

    ax1m = ax1.twinx()
    ax2m = ax2.twinx()
    ax3m = ax3.twinx()
    ax2m.set_ylabel('memory usage (MB)')

    fsst_mem_plot, = ax1m.plot(TAXPAYER_BATCH_SIZES, taxpayer_peak_memory, 'x--', markersize=MARKER_SIZE, color=COL_NATIVE_PRIMARY)
    ree_mem_plot, = ax2m.plot(RLE_BATCH_SIZES, rle_peak_memory, 'x--', markersize=MARKER_SIZE, color=COL_NATIVE_PRIMARY)
    ree_simd_mem_plot, = ax3m.plot(RLE_BATCH_SIZES, rle_simd_peak_memory, 'x--', markersize=MARKER_SIZE, color=COL_NATIVE_PRIMARY)

    ax1.legend([fsst_plot, fsst_mem_plot], ['FSST time', 'FSST memory'], fontsize=LEGEND_FONT_SIZE, loc='upper center', ncol=2)
    ax2.legend([ree_plot, ree_mem_plot], ['REE time', 'REE memory'], fontsize=LEGEND_FONT_SIZE, loc='upper center', ncol=2)
    ax3.legend([ree_simd_plot, ree_simd_mem_plot], ['REE(V128) time', 'REE(V128) memory'], fontsize=LEGEND_FONT_SIZE, loc='upper center', ncol=2)
    # ax1.legend(
    #     [fsst_plot, ree_plot, ree_simd_plot, fsst_mem_plot, ree_mem_plot, ree_simd_mem_plot],
    #     ['FSST time', 'REE time', 'REE (V128) time', 'FSST memory', 'REE memory', 'REE (V128) memory', ],
    #     title='Decoder resource',
    #     fontsize=LEGEND_FONT_SIZE, loc='upper center')

    taxpayer_best_bs = TAXPAYER_BATCH_SIZES[taxpayer_mean_times.index(taxpayer_min_time)]
    rle_best_bs = RLE_BATCH_SIZES[rle_mean_times.index(rle_min_time)]
    rle_simd_best_bs = RLE_BATCH_SIZES[rle_simd_mean_times.index(rle_simd_min_time)]
    print(f"mins: fsst {taxpayer_min_time} at {taxpayer_best_bs}, \
    rle {rle_min_time} at {rle_best_bs}, rle_simd {rle_simd_min_time} at {rle_simd_best_bs}")

    figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)
    figure.tight_layout()
    figure.savefig('batch_sizes.pdf')


plot_isolated_decoders()
plot_batchsizes()
