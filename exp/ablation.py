import csv
import matplotlib.pyplot as plt
import numpy as np
import sys


def non_nan(x):
    return x[~np.isnan(x)]


PX_PER_IN = 250
COL_IGNITION_PRIMARY = '#D03389'
COL_IGNITION_SECONDARY = '#A02568'
COL_NATIVE_PRIMARY = '#33D07A'
COL_NATIVE_SECONDARY = '#25A05D'
ABLATION_DIR = '.'
CALL_TIME_PATH = 'wasm_ablation_calltime.csv'
CALL_TIME_SAMPLES = 100000
COMPILATION_NAMES = ['rle', 'rle_simd', 'taxpayer_fsst', 'tpch_vortex']
TO_MS = 1000000
TO_μS = 1000

LEGEND_FONT_SIZE = 10
TITLES_FONT_SIZE = 18
LABELS_FONT_SIZE = 16
ANNOTATIONS_FONT_SIZE = 16

plt.rcParams["font.family"] = "Libertinus Sans"
plt.rcParams["xtick.labelsize"] = LABELS_FONT_SIZE
plt.rcParams["ytick.labelsize"] = LABELS_FONT_SIZE
plt.rcParams["axes.labelsize"] = LABELS_FONT_SIZE
plt.rcParams["axes.titlesize"] = TITLES_FONT_SIZE
plt.rcParams["legend.fontsize"] = LEGEND_FONT_SIZE

def plot_call_time():
    data_t1 = np.genfromtxt(f"{ABLATION_DIR}/wasm_ablation_calltime_t1_s{CALL_TIME_SAMPLES}.csv", delimiter=',', skip_header=1)[0:,1] / CALL_TIME_SAMPLES
    data_t32 = np.genfromtxt(f"{ABLATION_DIR}/wasm_ablation_calltime_t32_s{CALL_TIME_SAMPLES}.csv", delimiter=',', skip_header=1)[0:,1] / CALL_TIME_SAMPLES
    
    figure, axes = plt.subplots()
    bplot = axes.boxplot([data_t1, data_t32], vert=True, patch_artist=True)
    axes.set_ylabel('latency (ns)')
    axes.set_ylim(ymin=0)
    axes.set_title('Call time into wasmtime')
    axes.set_xticklabels(['t1', 't32'])
    axes.grid(linestyle='--', axis='y')

    for patch in bplot['boxes']:
        patch.set_facecolor(COL_IGNITION_PRIMARY)

    figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)
    figure.tight_layout()
    figure.savefig("ablation_call_time.pdf")


def plot_utf8():
    data_t1_disabled = np.genfromtxt(f"{ABLATION_DIR}/wasm_ablation_utf8_t1_b30000_disabled.csv", delimiter=',', skip_header=1)
    data_t32_disabled = np.genfromtxt(f"{ABLATION_DIR}/wasm_ablation_utf8_t32_b30000_disabled.csv", delimiter=',', skip_header=1)
    data_t1_enabled = np.genfromtxt(f"{ABLATION_DIR}/wasm_ablation_utf8_t1_b30000_enabled.csv", delimiter=',', skip_header=1)
    data_t32_enabled = np.genfromtxt(f"{ABLATION_DIR}/wasm_ablation_utf8_t32_b30000_enabled.csv", delimiter=',', skip_header=1)
    mean_t1_disabled = np.mean(data_t1_disabled / TO_MS)
    mean_t32_disabled = np.mean(data_t32_disabled / TO_MS)
    mean_t1_enabled = np.mean(data_t1_enabled / TO_MS)
    mean_t32_enabled = np.mean(data_t32_enabled / TO_MS)

    t1_diff = mean_t1_enabled - mean_t1_disabled
    t32_diff = mean_t32_enabled - mean_t32_disabled

    figure, axes = plt.subplots()

    rects_diff = axes.bar([0, 1], [t1_diff, t32_diff], color=COL_IGNITION_SECONDARY)
    rects_common = axes.bar([0, 1], [mean_t1_disabled, mean_t32_disabled], color=COL_IGNITION_PRIMARY, bottom=[t1_diff, t32_diff])

    axes.set_ylabel('latency (ms)')
    axes.set_ylim(ymin=0)
    axes.set_title('Impact of UTF-8 Validation')
    axes.set_xticks([0, 1])
    axes.set_xticklabels(['t1', 't32'])

    axes.legend([rects_diff, rects_common], ['UTF-8 validation', 'Decoder run'], fontsize=LEGEND_FONT_SIZE, loc='upper right')
    axes.grid(linestyle='--', axis='y')

    figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)
    figure.tight_layout()
    figure.savefig("ablation_utf8.pdf")


def plot_compile_time():
    data = [np.genfromtxt(f"{ABLATION_DIR}/wasm_ablation_compilation_{x}.csv", delimiter=',', skip_header=1)[0:,1] / TO_MS for x in COMPILATION_NAMES]

    figure, (ax1, ax2) = plt.subplots(ncols=2)

    bplot1 = ax1.boxplot(data[:3], vert=True, patch_artist=True)
    bplot2 = ax2.boxplot(data[3:], vert=True, patch_artist=True)
    ax1.set_ylabel('latency (ms)')
    ax1.set_ylim(ymin=0)
    ax2.set_ylim(ymin=0, ymax=300)
    ax1.set_xticks(np.arange(3) + 1)
    ax1.set_xticklabels(COMPILATION_NAMES[:3])
    ax2.set_xticks(np.arange(len(data[3:])) + 1)
    ax2.set_xticklabels(COMPILATION_NAMES[3:])

    for patch in bplot1['boxes']:
        patch.set_facecolor(COL_IGNITION_PRIMARY)
    for patch in bplot2['boxes']:
        patch.set_facecolor(COL_IGNITION_PRIMARY)

    figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)
    figure.tight_layout()
    figure.savefig("ablation_compilation.pdf")


def plot_init():
    data_thread_init_t1 = np.genfromtxt(f"{ABLATION_DIR}/wasm_ablation_thread_init_urn:anyblox:taxpayer_fsst_t1_s1000.csv", delimiter=',', skip_header=1) / TO_MS
    data_bundle_init_t1 = np.genfromtxt(f"{ABLATION_DIR}/wasm_ablation_bundle_init_urn:anyblox:taxpayer_fsst_t1.csv", delimiter=',', skip_header=1)[0:,1] / TO_μS
    data_job_init_t1 = np.genfromtxt(f"{ABLATION_DIR}/wasm_ablation_job_init_urn:anyblox:taxpayer_fsst_t1.csv", delimiter=',', skip_header=1)[0:,1] / TO_μS
    data_thread_init_t32 = np.genfromtxt(f"{ABLATION_DIR}/wasm_ablation_thread_init_urn:anyblox:taxpayer_fsst_t32_s1000.csv", delimiter=',', skip_header=1) / TO_MS
    data_bundle_init_t32 = np.genfromtxt(f"{ABLATION_DIR}/wasm_ablation_bundle_init_urn:anyblox:taxpayer_fsst_t32.csv", delimiter=',', skip_header=1)[0:,1] / TO_μS
    data_job_init_t32 = np.genfromtxt(f"{ABLATION_DIR}/wasm_ablation_job_init_urn:anyblox:taxpayer_fsst_t32.csv", delimiter=',', skip_header=1)[0:,1] / TO_μS

    figure, (ax1, ax2, ax3) = plt.subplots(nrows=3)

    bplot1 = ax1.boxplot([data_thread_init_t1, data_thread_init_t32], vert=False, patch_artist=True, widths=0.5, whis=(1,99))
    bplot2 = ax2.boxplot([data_bundle_init_t1, data_bundle_init_t32], vert=False, patch_artist=True, widths=0.5, whis=(1,99))
    bplot3 = ax3.boxplot([data_job_init_t1, data_job_init_t32], vert=False, patch_artist=True, widths=0.5, whis=(1,99))

    ax1.set_xlabel('latency (ms)')
    ax1.set_xlim(xmin=0)
    ax1.set_yticks([1, 2])
    ax1.set_yticklabels(['t1', 't32'])
    ax1.set_ylabel('Thread init')
    ax1.grid(linestyle='--', axis='x')
    ax2.set_xlabel('latency (μs)')
    ax2.set_xlim(xmin=0)
    ax2.set_yticks([1, 2])
    ax2.set_yticklabels(['t1', 't32'])
    ax2.set_ylabel('Bundle init')
    ax2.grid(linestyle='--', axis='x')
    ax3.set_xlabel('latency (μs)')
    ax3.set_xlim(xmin=0)
    ax3.set_yticks([1, 2])
    ax3.set_yticklabels(['t1', 't32'])
    ax3.set_ylabel('Job init')
    ax3.grid(linestyle='--', axis='x')

    bplot1['boxes'][0].set_facecolor(COL_IGNITION_PRIMARY)
    bplot1['boxes'][1].set_facecolor(COL_NATIVE_PRIMARY)
    bplot2['boxes'][0].set_facecolor(COL_IGNITION_PRIMARY)
    bplot2['boxes'][1].set_facecolor(COL_NATIVE_PRIMARY)
    bplot3['boxes'][0].set_facecolor(COL_IGNITION_PRIMARY)
    bplot3['boxes'][1].set_facecolor(COL_NATIVE_PRIMARY)

    figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)
    figure.tight_layout()
    figure.savefig("ablation_init.pdf")


#plot_call_time()
#plot_utf8()
#plot_compile_time()
plot_init()