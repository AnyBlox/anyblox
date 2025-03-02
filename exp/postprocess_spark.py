import os
import json
import sys

TMP_DIR = "/tmp/duckdb-ignition-profiling"
QUERIES = [1, 3, 4, 5, 6, 7, 8, 9, 10, 12, 14, 15, 17, 18, 19, 20, 21]
SAMPLES = 10


def extract_latency(sample):
    return sample["totalTime"]


def is_ignition_scan(node):
    return "nodeName" in node and node["nodeName"].startswith("BatchScan anyblox")


def is_parquet_lineitem_scan(node):
    if "nodeName" in node and node["nodeName"].startswith("Scan parquet"):
        for candidate in ["lineitem.parquet", "hits-strings-0.parquet", "hits-strings-1.parquet", "hits-strings-2.parquet", "hits-strings-3.parquet"]:
          if candidate in node["metadata"]["Location"]:
            return True
    return False


def extract_scan_cputime(node):
    total = 0
    if is_ignition_scan(node) or is_parquet_lineitem_scan(node):
        # Ignition reports time in ns, other operators in ms
        factor = 1_000_000.0 if is_ignition_scan(node) else 1.0
        for metric in node["metrics"]:
            if metric["metricName"].startswith("time spent") or metric["metricName"] == "scan time":
                total += metric["value"] / factor
    if "children" in node:
        for child in node["children"]:
            total += extract_scan_cputime(child)
    return round(total)


if len(sys.argv) != 3:
    print(f"Usage: {sys.argv[0]} REPORT_DIR OUT_FILE_NAME")
    exit(1)

report_dir = sys.argv[1]
output_file_name = sys.argv[2]

samples = {}

for name in os.listdir(report_dir):
    if name.startswith('q') and name.endswith('.json'):
        q = name.removesuffix('.json').removeprefix('q')
        samples[f"{q}"] = []
        input_file = f"{report_dir}/{name}"
        with open(input_file, 'r') as file:
            jsonObj = json.load(file)
        for sample in jsonObj:
            latency = extract_latency(sample)
            scan_cputime = extract_scan_cputime(sample["plan"])
            samples[f"{q}"].append([latency, scan_cputime])

with open(f"{report_dir}/{output_file_name}", 'w') as file:
    json.dump(samples, file)