import csv
import matplotlib.pyplot as plt
import numpy as np
import sys


def non_nan(x):
    return x[~np.isnan(x)]


PX_PER_IN = 250
QUARTILES = [0, 0.25, 0.5, 0.75, 1]

ignition = np.array([506722251,
282518605,
202063499,
163760811,
141825915,
126617468,
123014708,
114767507,
109651840,
106084337,
106321678,
113863237,
120340092,
113558237,
110381657,
111909924,
])

spark = np.array([
    974435185,
796389054,
798660203,
869751855,
792563130,
800152411,
796927198,
794635862,
795832267,
797714706,
801445250,
797204627,
798149274,
796715447,
796789969,
874486159,
])

native_only = np.array([
    32045964,
    15953454,
    10606854,
    8095492,
    6640560,
    5432032,
    4843487,
    4213960,
    3677985,
    3304325,
    3055827,
    2832652,
    2623820,
    2422317,
    2396392,
    2329972
])

out_size = 296721856


figure, axes = plt.subplots()

width = 0.33
x = np.arange(16)  # the label locations
threads = np.arange(1, 17)

rects_spark = axes.bar(x, spark / 1000000.0, width, color="red")
#axes.bar_label(rects_spark, rotation=45, padding=2, fmt="%.2f")

bottom = np.zeros(16)
rects_ignition_native = axes.bar(x + width, native_only / 1000000.0, width, color = "orange", bottom=bottom)
bottom += native_only / 1000000.0
rects_ignition = axes.bar(x + width, (ignition - native_only) / 1000000.0, width, color="dodgerblue", bottom=bottom)

import matplotlib.ticker as mtick
plabs = ['{:,.2%}'.format(x) for x in native_only/ignition]
axes.bar_label(rects_ignition_native, labels=plabs, rotation=45, padding=2)
#axes.bar_label(rects_ignition, rotation=45, padding=2, fmt="%.2f")

axes.set_xlabel('threads')
axes.set_ylabel('latency (ms)')
axes.set_ylim(ymin=0)
axes.set_title('''RLE spark .parquet vs .ignition
query: SELECT col1, COUNT(col1) FROM 1_Agencia_ID GROUP BY col1 ORDER BY COUNT(col1) DESC
''')
axes.set_xticks(x + width/2)
axes.set_xticklabels(threads)

spark_ideal = (spark / 1000000.0) / threads
ignition_ideal = (ignition / 1000000.0) / threads

#native_ideal = [61.37, 61.37 / 2, 61.37 / 4, 61.37 / 8, 61.37 / 16, 61.37 / 31]
#wasm_ideal = [211.7, 211.7 / 2, 211.7 / 4, 211.7 / 8, 211.7 / 16, 211.7 / 31]

line_spark, = axes.plot(x, spark_ideal, linewidth=2, color='lightcoral', linestyle='--', marker='x')
line_ignition, = axes.plot(x + width, ignition_ideal, linewidth=2, color='paleturquoise', linestyle='--', marker='x')

axes.legend(
    [rects_spark, rects_ignition_native, rects_ignition, line_spark, line_ignition], 
    ['spark', 'ignition runtime', 'ignition-spark', 'ideal (spark)', 'ideal (ignition)'],
    title='Legend')

axes.grid(linestyle='--', axis='y')

figure.set_size_inches(1920 / PX_PER_IN + 1, 1080 / PX_PER_IN + 1)
figure.savefig("rle_bimbo_vs_spark.svg")