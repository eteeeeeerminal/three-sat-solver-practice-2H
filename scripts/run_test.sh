#!/bin/bash
test=test

python3 ./scripts/run_solver.py solvers/my_solver_with_propagate $test ./test_log/my_solver_with_propagate
python3 ./scripts/run_solver.py solvers/minisat_release $test ./test_log/minisat_release

python3 ./scripts/test_logs.py ./test_log/minisat_release ./test_log/my_solver_with_propagate


python3 ./scripts/run_solver.py solvers/my_simple_solver $test ./test_log/my_simple_solver
python3 ./scripts/test_logs.py ./test_log/minisat_release ./test_log/my_simple_solver
