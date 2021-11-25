#!/bin/bash
time_max=10
test_data=benchmark_input2
timeout $time_max python3 ./scripts/run_solver.py ./solvers/my_solver_with_propagate $test_data ./benchmark_log2/my_solver_with_propagate &
timeout $time_max python3 ./scripts/run_solver.py ./solvers/minisat $test_data ./benchmark_log2/minisat &