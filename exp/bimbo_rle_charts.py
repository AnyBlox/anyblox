import csv
import matplotlib.pyplot as plt
import numpy as np

def non_nan(x):
    return x[~np.isnan(x)]


PX_PER_IN = 250
QUARTILES = [0, 0.25, 0.5, 0.75, 1]

native_t1 = np.genfromtxt('native_rle_bimbo_t1.csv', delimiter=',', skip_header=1)
native_t2 = np.genfromtxt('native_rle_bimbo_t2.csv', delimiter=',', skip_header=1)
native_t4 = np.genfromtxt('native_rle_bimbo_t4.csv', delimiter=',', skip_header=1)
native_t8 = np.genfromtxt('native_rle_bimbo_t8.csv', delimiter=',', skip_header=1)
native_t16 = np.genfromtxt('native_rle_bimbo_t16.csv', delimiter=',', skip_header=1)
native_t31 = np.genfromtxt('native_rle_bimbo_t31.csv', delimiter=',', skip_header=1)
native_simd_t1 = np.genfromtxt('native_rle_simd_bimbo_t1.csv', delimiter=',', skip_header=1)
native_simd_t2 = np.genfromtxt('native_rle_simd_bimbo_t2.csv', delimiter=',', skip_header=1)
native_simd_t4 = np.genfromtxt('native_rle_simd_bimbo_t4.csv', delimiter=',', skip_header=1)
native_simd_t8 = np.genfromtxt('native_rle_simd_bimbo_t8.csv', delimiter=',', skip_header=1)
native_simd_t16 = np.genfromtxt('native_rle_simd_bimbo_t16.csv', delimiter=',', skip_header=1)
native_simd_t31 = np.genfromtxt('native_rle_simd_bimbo_t31.csv', delimiter=',', skip_header=1)
wasm_t1 = np.genfromtxt('wasm_rle_bimbo_t1.csv', delimiter=',', skip_header=1)
wasm_t2 = np.genfromtxt('wasm_rle_bimbo_t2.csv', delimiter=',', skip_header=1)
wasm_t4 = np.genfromtxt('wasm_rle_bimbo_t4.csv', delimiter=',', skip_header=1)
wasm_t8 = np.genfromtxt('wasm_rle_bimbo_t8.csv', delimiter=',', skip_header=1)
wasm_t16 = np.genfromtxt('wasm_rle_bimbo_t16.csv', delimiter=',', skip_header=1)
wasm_t31 = np.genfromtxt('wasm_rle_bimbo_t31.csv', delimiter=',', skip_header=1)
wasm_simd_t1 = np.genfromtxt('wasm_rle_simd_bimbo_t1.csv', delimiter=',', skip_header=1)
wasm_simd_t2 = np.genfromtxt('wasm_rle_simd_bimbo_t2.csv', delimiter=',', skip_header=1)
wasm_simd_t4 = np.genfromtxt('wasm_rle_simd_bimbo_t4.csv', delimiter=',', skip_header=1)
wasm_simd_t8 = np.genfromtxt('wasm_rle_simd_bimbo_t8.csv', delimiter=',', skip_header=1)
wasm_simd_t16 = np.genfromtxt('wasm_rle_simd_bimbo_t16.csv', delimiter=',', skip_header=1)
wasm_simd_t31 = np.genfromtxt('wasm_rle_simd_bimbo_t31.csv', delimiter=',', skip_header=1)
out_size = 296721856

native_total_t1 = out_size / non_nan(native_t1[0:,0] + native_t1[0:,1])
native_total_t2 = out_size / non_nan(native_t2[0:,0] + native_t2[0:,1])
native_total_t4 = out_size / non_nan(native_t4[0:,0] + native_t4[0:,1])
native_total_t8 = out_size / non_nan(native_t8[0:,0] + native_t8[0:,1])
native_total_t16 = out_size / non_nan(native_t16[0:,0] + native_t16[0:,1])
native_total_t31 = out_size / non_nan(native_t31[0:,0] + native_t31[0:,1])
native_simd_total_t1 = out_size / non_nan(native_simd_t1[0:,0] + native_simd_t1[0:,1])
native_simd_total_t2 = out_size / non_nan(native_simd_t8[0:,0] + native_simd_t8[0:,1])
native_simd_total_t4 = out_size / non_nan(native_simd_t8[0:,0] + native_simd_t8[0:,1])
native_simd_total_t8 = out_size / non_nan(native_simd_t8[0:,0] + native_simd_t8[0:,1])
native_simd_total_t16 = out_size / non_nan(native_simd_t16[0:,0] + native_simd_t16[0:,1])
native_simd_total_t31 = out_size / non_nan(native_simd_t31[0:,0] + native_simd_t31[0:,1])
wasm_total_t1 = out_size / non_nan(wasm_t1[0:,0] + wasm_t1[0:,1])
wasm_total_t2 = out_size / non_nan(wasm_t8[0:,0] + wasm_t8[0:,1])
wasm_total_t4 = out_size / non_nan(wasm_t8[0:,0] + wasm_t8[0:,1])
wasm_total_t8 = out_size / non_nan(wasm_t8[0:,0] + wasm_t8[0:,1])
wasm_total_t16 = out_size / non_nan(wasm_t16[0:,0] + wasm_t16[0:,1])
wasm_total_t31 = out_size / non_nan(wasm_t31[0:,0] + wasm_t31[0:,1])
wasm_simd_total_t1 = out_size / non_nan(wasm_simd_t1[0:,0] + wasm_simd_t1[0:,1])
wasm_simd_total_t2 = out_size / non_nan(wasm_simd_t8[0:,0] + wasm_simd_t8[0:,1])
wasm_simd_total_t4 = out_size / non_nan(wasm_simd_t8[0:,0] + wasm_simd_t8[0:,1])
wasm_simd_total_t8 = out_size / non_nan(wasm_simd_t8[0:,0] + wasm_simd_t8[0:,1])
wasm_simd_total_t16 = out_size / non_nan(wasm_simd_t16[0:,0] + wasm_simd_t16[0:,1])
wasm_simd_total_t31 = out_size / non_nan(wasm_simd_t31[0:,0] + wasm_simd_t31[0:,1])

figure, axes = plt.subplots()

bplot = axes.boxplot([native_total_t1, wasm_total_t1, native_total_t8, wasm_total_t8, native_total_t16, wasm_total_t16, native_total_t31, wasm_total_t31,], vert=True, patch_artist=True, widths=0.8)
axes.set_ylabel('thpt (GB/s)')
axes.set_ylim(ymin=0)
axes.set_title('RLE, non vectorised total runtime')
axes.set_xticks([1, 2, 3, 4, 5, 6, 7, 8])
axes.set_xticklabels(['native1', 'wasm1', 'native8', 'wasm8','native16', 'wasm16','native31', 'wasm31',])
axes.grid(linestyle='--', axis='y')

for patch in bplot['boxes']:
    patch.set_facecolor('deepskyblue')

figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)
figure.savefig("rle_bimbo_rle_total.svg")

figure, axes = plt.subplots()

bplot = axes.boxplot([native_simd_total_t1, wasm_simd_total_t1, native_simd_total_t8, wasm_simd_total_t8, native_simd_total_t16, wasm_simd_total_t16, native_simd_total_t31, wasm_simd_total_t31,], vert=True, patch_artist=True, widths=0.8)
axes.set_ylabel('thpt (GB/s)')
axes.set_ylim(ymin=0)
axes.set_title('RLE, SIMD vectorised total runtime')
axes.set_xticks([1, 2, 3, 4, 5, 6, 7, 8])
axes.set_xticklabels(['native1', 'wasm1', 'native8', 'wasm8','native16', 'wasm16','native31', 'wasm31',])
axes.grid(linestyle='--', axis='y')

for patch in bplot['boxes']:
    patch.set_facecolor('deepskyblue')

figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)
figure.savefig("rle_bimbo_rle_simd_total.svg")

figure, axes = plt.subplots()

bplot = axes.boxplot([native_total_t31, native_simd_total_t31, wasm_total_t31, wasm_simd_total_t31], vert=True, patch_artist=True, widths=0.8)
axes.set_ylabel('thpt (GB/s)')
axes.set_ylim(ymin=0)
axes.set_title('RLE, total runtime t=31')
axes.set_xticks([1, 2, 3,4 ])
axes.set_xticklabels(['native31', 'native31_simd', 'wasm31', 'wasm31_simd',])
axes.grid(linestyle='--', axis='y')

for patch in bplot['boxes']:
    patch.set_facecolor('deepskyblue')

figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)
figure.savefig("rle_bimbo_zoom_in.svg")

figure, axes = plt.subplots(layout='constrained')

native_t1_startup = np.mean(native_t1[0:,0]) / 1000000
native_t2_startup = np.mean(native_t2[0:,0]) / 1000000
native_t4_startup = np.mean(native_t4[0:,0]) / 1000000
native_t8_startup = np.mean(native_t8[0:,0]) / 1000000
native_t16_startup = np.mean(native_t16[0:,0]) / 1000000
native_t31_startup = np.mean(native_t31[0:,0]) / 1000000
native_t1_decoding = np.mean(native_t1[0:,1]) / 1000000
native_t2_decoding = np.mean(native_t2[0:,1]) / 1000000
native_t4_decoding = np.mean(native_t4[0:,1]) / 1000000
native_t8_decoding = np.mean(native_t8[0:,1]) / 1000000
native_t16_decoding = np.mean(native_t16[0:,1]) / 1000000
native_t31_decoding = np.mean(native_t31[0:,1]) / 1000000
wasm_t1_startup = np.mean(wasm_t1[0:,0]) / 1000000
wasm_t2_startup = np.mean(wasm_t2[0:,0]) / 1000000
wasm_t4_startup = np.mean(wasm_t4[0:,0]) / 1000000
wasm_t8_startup = np.mean(wasm_t8[0:,0]) / 1000000
wasm_t16_startup = np.mean(wasm_t16[0:,0]) / 1000000
wasm_t31_startup = np.mean(wasm_t31[0:,0]) / 1000000
wasm_t1_decoding = np.mean(wasm_t1[0:,1]) / 1000000
wasm_t2_decoding = np.mean(wasm_t2[0:,1]) / 1000000
wasm_t4_decoding = np.mean(wasm_t4[0:,1]) / 1000000
wasm_t8_decoding = np.mean(wasm_t8[0:,1]) / 1000000
wasm_t16_decoding = np.mean(wasm_t16[0:,1]) / 1000000
wasm_t31_decoding = np.mean(wasm_t31[0:,1]) / 1000000

native_total_t1 = non_nan(native_t1[0:,0] + native_t1[0:,1]) / 1000000
native_total_t2 = non_nan(native_t2[0:,0] + native_t2[0:,1]) / 1000000
native_total_t4 = non_nan(native_t4[0:,0] + native_t4[0:,1]) / 1000000
native_total_t8 = non_nan(native_t8[0:,0] + native_t8[0:,1]) / 1000000
native_total_t16 = non_nan(native_t16[0:,0] + native_t16[0:,1]) / 1000000
native_total_t31 = non_nan(native_t31[0:,0] + native_t31[0:,1]) / 1000000
native_simd_total_t1 = non_nan(native_simd_t1[0:,0] + native_simd_t1[0:,1]) / 1000000
native_simd_total_t2 = non_nan(native_simd_t2[0:,0] + native_simd_t2[0:,1]) / 1000000
native_simd_total_t4 = non_nan(native_simd_t4[0:,0] + native_simd_t4[0:,1]) / 1000000
native_simd_total_t8 = non_nan(native_simd_t8[0:,0] + native_simd_t8[0:,1]) / 1000000
native_simd_total_t16 = non_nan(native_simd_t16[0:,0] + native_simd_t16[0:,1]) / 1000000
native_simd_total_t31 = non_nan(native_simd_t31[0:,0] + native_simd_t31[0:,1]) / 1000000
wasm_total_t1 = non_nan(wasm_t1[0:,0] + wasm_t1[0:,1]) / 1000000
wasm_total_t2 = non_nan(wasm_t2[0:,0] + wasm_t2[0:,1]) / 1000000
wasm_total_t4 = non_nan(wasm_t4[0:,0] + wasm_t4[0:,1]) / 1000000
wasm_total_t8 = non_nan(wasm_t8[0:,0] + wasm_t8[0:,1]) / 1000000
wasm_total_t16 = non_nan(wasm_t16[0:,0] + wasm_t16[0:,1]) / 1000000
wasm_total_t31 = non_nan(wasm_t31[0:,0] + wasm_t31[0:,1]) / 1000000
wasm_simd_total_t1 = non_nan(wasm_simd_t1[0:,0] + wasm_simd_t1[0:,1]) / 1000000
wasm_simd_total_t2 = non_nan(wasm_simd_t2[0:,0] + wasm_simd_t2[0:,1]) / 1000000
wasm_simd_total_t4 = non_nan(wasm_simd_t4[0:,0] + wasm_simd_t4[0:,1]) / 1000000
wasm_simd_total_t8 = non_nan(wasm_simd_t8[0:,0] + wasm_simd_t8[0:,1]) / 1000000
wasm_simd_total_t16 = non_nan(wasm_simd_t16[0:,0] + wasm_simd_t16[0:,1]) / 1000000
wasm_simd_total_t31 = non_nan(wasm_simd_t31[0:,0] + wasm_simd_t31[0:,1]) / 1000000

native = [native_total_t1, native_total_t2, native_total_t4, native_total_t8, native_total_t16, native_total_t31]
wasm = [wasm_total_t1, wasm_total_t2, wasm_total_t4, wasm_total_t8, wasm_total_t16, wasm_total_t31]

for i in range(6):
    native[i] = np.median(native[i])
    wasm[i] = np.median(wasm[i])


width = 0.33
x = np.arange(6)  # the label locations

rects_native = axes.bar(x, native, width, color="orange")
axes.bar_label(rects_native, padding=2, fmt="%.2f")
rects_wasm = axes.bar(x + width, wasm, width, color="dodgerblue")
axes.bar_label(rects_wasm, padding=2, fmt="%.2f")

axes.set_ylabel('latency (ms)')
axes.set_ylim(ymin=0)
axes.set_title('RLE, median latency by no. of threads')
axes.set_xticks(x + width/2)
axes.set_xticklabels(['t=1', 't=2', 't=4', 't=8', 't=16', 't=31',])

native_ideal = [61.37, 61.37 / 2, 61.37 / 4, 61.37 / 8, 61.37 / 16, 61.37 / 31]
wasm_ideal = [211.7, 211.7 / 2, 211.7 / 4, 211.7 / 8, 211.7 / 16, 211.7 / 31]

line_native = axes.plot(x, native_ideal, linewidth=2, color='orangered', linestyle='--', marker='x')
line_wasm = axes.plot(x + width, wasm_ideal, linewidth=2, color='darkblue', linestyle='--', marker='x')

axes.legend([rects_native, rects_wasm, line_native, line_wasm], ['native', 'wasm', 'ideal (native)', 'ideal (wasm)'], title='Impl')

axes.grid(linestyle='--', axis='y')

figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)
figure.savefig("rle_bimbo_latency.svg")


native = [native_simd_total_t1, native_simd_total_t2, native_simd_total_t4, native_simd_total_t8, native_simd_total_t16, native_simd_total_t31]
wasm = [wasm_simd_total_t1, wasm_simd_total_t2, wasm_simd_total_t4, wasm_simd_total_t8, wasm_simd_total_t16, wasm_simd_total_t31]

for i in range(6):
    native[i] = np.median(native[i])
    wasm[i] = np.median(wasm[i])


width = 0.33
x = np.arange(6)  # the label locations

rects_native = axes.bar(x, native, width, color="orange")
axes.bar_label(rects_native, padding=2, fmt="%.2f")
rects_wasm = axes.bar(x + width, wasm, width, color="dodgerblue")
axes.bar_label(rects_wasm, padding=2, fmt="%.2f")

axes.set_ylabel('latency (ms)')
axes.set_ylim(ymin=0)
axes.set_title('RLE with SIMD, median latency by no. of threads')
axes.set_xticks(x + width/2)
axes.set_xticklabels(['t=1', 't=2', 't=4', 't=8', 't=16', 't=31',])

native_ideal = [61.37, 61.37 / 2, 61.37 / 4, 61.37 / 8, 61.37 / 16, 61.37 / 31]
wasm_ideal = [211.7, 211.7 / 2, 211.7 / 4, 211.7 / 8, 211.7 / 16, 211.7 / 31]

line_native = axes.plot(x, native_ideal, linewidth=2, color='orangered', linestyle='--', marker='x')
line_wasm = axes.plot(x + width, wasm_ideal, linewidth=2, color='darkblue', linestyle='--', marker='x')

axes.legend([rects_native, rects_wasm, line_native, line_wasm], ['native', 'wasm', 'ideal (native)', 'ideal (wasm)'], title='Impl')

axes.grid(linestyle='--', axis='y')

figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)
figure.savefig("rle_bimbo_simd_latency.svg")