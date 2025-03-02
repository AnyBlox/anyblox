import csv
import matplotlib.pyplot as plt
import numpy as np

def non_nan(x):
    return x[~np.isnan(x)]


PX_PER_IN = 250
COL_IGNITION_PRIMARY = '#CC0256'
COL_IGNITION_SECONDARY = '#66012B'
COL_NATIVE_PRIMARY = '#02CC78'
COL_NATIVE_SECONDARY = '#01663C'
QUARTILES = [0, 0.25, 0.5, 0.75, 1]
TAXPAYER_OUT_SIZE = 100
TO_MS = 1000000
TO_S = 1000000000
TAXPAYER_DIR = 'decoders/Taxpayer'
COMMON_GOVERNMENT_DIR = 'decoders/CommonGovernmentScaling'
THREADS = [1, 2, 4, 8, 16, 32]

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

wasm_data = [np.genfromtxt(f'{TAXPAYER_DIR}/wasm_taxpayer_fsst_Taxpayer_1.fsst_SELF-CONTAINED_t{t}_b30000_ufalse.csv', delimiter=',', skip_header=1) for t in THREADS]
native_data = [np.genfromtxt(f'{TAXPAYER_DIR}/native_taxpayer_fsst_Taxpayer_1.fsst_SELF-CONTAINED_t{t}_b30000_ufalse.csv', delimiter=',', skip_header=1) for t in THREADS]
wasm_totals = [(n[0:,0] + n[0:,1]) for n in wasm_data]
native_totals = [(n[0:,0] + n[0:,1]) for n in native_data]
wasm_means = [np.mean(n) / TO_MS for n in wasm_totals]
native_means = [np.mean(n) / TO_MS for n in native_totals]
SIZE = 247063866 / 1_000

figure, ax1 = plt.subplots()

width = 0.5
x = np.arange(len(wasm_means))  # the label locations

rects_native = ax1.bar(x, [(SIZE / m) / t for (t, m) in zip(THREADS, native_means)], width, width, color=COL_NATIVE_PRIMARY)
#axes.bar_label(rects_native, padding=2, fmt="%.2f")
rects_wasm = ax1.bar(x + width / 2, [(SIZE / m) / t for (t, m) in zip(THREADS, wasm_means)], width, color=COL_IGNITION_PRIMARY)
#axes.bar_label(rects_wasm, padding=2, fmt="%.2f")

ax1.set_ylabel('GB/s per thread')
ax1.set_title('FSST')
ax1.set_xticks(x + width/4)
ax1.set_xticklabels(THREADS)
ax1.set_xlabel('threads')
ax1.set_yticks(np.arange(0,75,5), minor=True)

#native_t1 = native_means[0]
wasm_t1 = wasm_means[0]

#native_ideal = [native_t1 / t for t in THREADS]
wasm_ideal = [wasm_t1 / t for t in THREADS]

#line_native, = ax1.plot(x, native_ideal, linewidth=2, color=COL_NATIVE_SECONDARY, linestyle='--', marker=1)
#line_wasm, = ax1.plot(x + width, wasm_ideal, linewidth=2, color=COL_IGNITION_SECONDARY, linestyle='--', marker=1)

ax1.set_ylim(ymin=0)
#ax1.legend([rects_native, rects_wasm, line_native, line_wasm], ['native', 'wasm', 'ideal (native)', 'ideal (wasm)'], title='Impl')

ax1.grid(linestyle='--', axis='y', which='both')
ax1.grid(which='minor', alpha=0.6)

def shrink(ax):
    box = ax.get_position()
    ax.set_position([box.x0, box.y0, box.width, box.height * 0.85])

figure.legend([rects_native, rects_wasm], ['native', 'wasm'], fontsize=LEGEND_FONT_SIZE)
figure.set_size_inches(1920 / PX_PER_IN, (1080 * 3 / 4) / PX_PER_IN)
figure.tight_layout()
shrink(ax1)
figure.savefig("scaling.pdf")
