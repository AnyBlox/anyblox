import csv
import matplotlib.pyplot as plt
import numpy as np
import sys

PX_PER_IN = 250


def non_nan(x):
    return x[~np.isnan(x)]


def rle():
    wasm_t1 = 45.8914
    native_t1 = 29.4949
    wasm_simd_t1 = 6.7357
    native_simd_128_t1 = 22.1832
    native_simd_256_t1 = 10.2001
    native_simd_512_t1 = 2.8586

    out_size = 296721856

    ## PLOT

    figure, axes = plt.subplots()

    width = 0.75
    x = np.arange(6)  # the label locations

    v_costs = np.array([wasm_t1, native_t1, wasm_simd_t1, native_simd_128_t1, native_simd_256_t1, native_simd_512_t1])

    rects_nv = axes.bar(x, v_costs, width, color="dodgerblue")

    axes.set_ylabel('latency (ms)')
    axes.set_ylim(ymin=0)
    axes.set_title('RLE, outsize=297MB, t=1, b=80000')

    axes.set_xticks(x)
    axes.set_xticklabels(['ignition', 'native', 'ignition v128', 'native v128', 'native v256', 'native v512'])
    axes.grid(linestyle='--', axis='y')

    figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)
    figure.savefig("baseline_rle.svg")


def fsst():
    wasm_t1 = 4393.3021
    wasm_t1_nv = 3336.6444
    native_t1 = 2694.5003
    native_t1_nv = 1755.9313

    wasm_v_cost = wasm_t1 - wasm_t1_nv
    native_v_cost = native_t1 - native_t1_nv

    out_size = 2471560908

    ## PLOT

    figure, axes = plt.subplots()

    width = 0.75
    x = np.arange(2)  # the label locations
    bottom = np.zeros(2)

    v_costs = np.array([wasm_v_cost, native_v_cost]) / 1000.0
    totals = np.array([wasm_t1_nv, native_t1_nv]) / 1000.0

    rects_utf8 = axes.bar(x, v_costs, width, color="orange")

    bottom += v_costs
    rects_nv = axes.bar(x, totals, width, color="dodgerblue", bottom=bottom)

    axes.set_ylabel('latency (s)')
    axes.set_ylim(ymin=0)
    axes.set_title('FSST, outsize=2.47GB, t=1, b=1024')

    axes.set_xticks(x)
    axes.set_xticklabels(['ignition', 'native'])
    axes.grid(linestyle='--', axis='y')

    axes.legend(
        [rects_utf8, rects_nv], 
        ['UTF8 validation', 'rest'],
        title='Legend')

    figure.set_size_inches(1920 / PX_PER_IN, 1080 / PX_PER_IN)
    figure.savefig("baseline_fsst.svg")


fsst()
rle()