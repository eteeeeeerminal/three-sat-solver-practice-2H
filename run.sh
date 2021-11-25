#!/bin/bash
./scripts/run_benchmark1.sh
./scripts/run_benchmark2.sh

sleep 1000

python3 ./scripts/shape_benchmark_data.py benchmark_log benchmark_input benchmark.csv
python3 ./scripts/shape_benchmark_data.py benchmark_log2 benchmark_input2 benchmark2.csv

gnuplot ./scripts/cactus1.plt
gnuplot ./scripts/cactus2.plt