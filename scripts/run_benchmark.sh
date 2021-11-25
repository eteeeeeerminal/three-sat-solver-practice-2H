#!/bin/bash
time_max=1000
test_data=benchmark_input
timeout $time_max python3 ./scripts/run_solver.py ./solvers/my_simple_solver $test_data ./benchmark_log/my_simple_solver &
timeout $time_max python3 ./scripts/run_solver.py ./solvers/my_solver_with_propagate $test_data ./benchmark_log/my_solver_with_propagate &
timeout $time_max python3 ./scripts/run_solver.py ./solvers/minisat_release $test_data ./benchmark_log/minisat_release &