import csv
import matplotlib.pyplot as plt
import numpy as np
import sys


def non_nan(x):
    return x[~np.isnan(x)]


PX_PER_IN = 250
QUARTILES = [0, 0.25, 0.5, 0.75, 1]

native_t1 = np.genfromtxt('native_taxpayer_all_columns_t1.csv', delimiter=',', skip_header=1)
native_t8 = np.genfromtxt('native_taxpayer_all_columns_t8.csv', delimiter=',', skip_header=1)
native_t16 = np.genfromtxt('native_taxpayer_all_columns_t16.csv', delimiter=',', skip_header=1)
native_t31 = np.genfromtxt('native_taxpayer_all_columns_t31.csv', delimiter=',', skip_header=1)
wasm_t1 = np.genfromtxt('wasm_taxpayer_all_columns_t1.csv', delimiter=',', skip_header=1)
wasm_t8 = np.genfromtxt('wasm_taxpayer_all_columns_t8.csv', delimiter=',', skip_header=1)
wasm_t16 = np.genfromtxt('wasm_taxpayer_all_columns_t16.csv', delimiter=',', skip_header=1)
wasm_t31 = np.genfromtxt('wasm_taxpayer_all_columns_t31.csv', delimiter=',', skip_header=1)
out_size = 256153554

native_total_t1 = out_size / non_nan(native_t1[0:,0] + native_t1[0:,1])
native_total_t8 = out_size / non_nan(native_t8[0:,0] + native_t8[0:,1])
native_total_t16 = out_size / non_nan(native_t16[0:,0] + native_t16[0:,1])
native_total_t31 = out_size / non_nan(native_t31[0:,0] + native_t31[0:,1])
wasm_total_t1 = out_size / non_nan(wasm_t1[0:,0] + wasm_t1[0:,1])
wasm_total_t8 = out_size / non_nan(wasm_t8[0:,0] + wasm_t8[0:,1])
wasm_total_t16 = out_size / non_nan(wasm_t16[0:,0] + wasm_t16[0:,1])
wasm_total_t31 = out_size / non_nan(wasm_t31[0:,0] + wasm_t31[0:,1])

wasm_startup_t1 = non_nan(wasm_t1[0:,0]) / 1000000.0
wasm_startup_t8 = non_nan(wasm_t8[0:,0]) / 1000000.0
wasm_startup_t16 = non_nan(wasm_t16[0:,0]) / 1000000.0
wasm_startup_t31 = non_nan(wasm_t31[0:,0]) / 1000000.0

figure, axes = plt.subplots()

bplot = axes.boxplot([native_total_t1, wasm_total_t1, native_total_t8, wasm_total_t8, native_total_t16, wasm_total_t16, native_total_t31, wasm_total_t31,], vert=True, patch_artist=True, widths=0.8)
axes.set_ylabel('thpt (GB/s)')
axes.set_ylim(ymin=0)
axes.set_title('FSST, 3 columns, total thpt')
axes.set_xticks([1, 2, 3, 4, 5, 6, 7, 8])
axes.set_xticklabels(['native1', 'wasm1', 'native8', 'wasm8','native16', 'wasm16','native31', 'wasm31',])
axes.grid(linestyle='--', axis='y')

for patch in bplot['boxes']:
    patch.set_facecolor('deepskyblue')

figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)
figure.savefig("taxpayer_fsst_total.svg")

figure, axes = plt.subplots()

figure, axes = plt.subplots()

bplot = axes.boxplot([native_total_t31, wasm_total_t31,], vert=True, patch_artist=True, widths=0.8)
axes.set_ylabel('thpt (GB/s)')
axes.set_ylim(ymin=0)
axes.set_title('FSST, 3 columns, total thpt t=31')
axes.set_xticks([1, 2])
axes.set_xticklabels(['native31', 'wasm31',])
axes.grid(linestyle='--', axis='y')

for patch in bplot['boxes']:
    patch.set_facecolor('deepskyblue')

figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)
figure.savefig("taxpayer_fsst_total_zoom_in.svg")

figure, axes = plt.subplots()

bplot = axes.boxplot([wasm_startup_t1,wasm_startup_t8,wasm_startup_t16,wasm_startup_t31,], vert=True, patch_artist=True, widths=0.8)
axes.set_ylabel('startup (ms)')
axes.set_ylim(ymin=0)
axes.set_title('FSST, 3 columns, wasm startup time')
axes.set_xticks([1, 2, 3, 4])
axes.set_xticklabels(['wasm (t1)', 'wasm (t8)', 'wasm (t16)', 'wasm (t31)',])
axes.grid(linestyle='--', axis='y')

for patch in bplot['boxes']:
    patch.set_facecolor('tomato')

figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)
figure.savefig("taxpayer_fsst_startup.svg")
