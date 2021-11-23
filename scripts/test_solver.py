# 与えられたsolver と 自作solverを動かして, 出力が同じことを確認する

import sys
import re
import os
import pathlib
import subprocess

# 実行部分
try:
    sample_solver = sys.argv[1]
    my_solver = sys.argv[2]
except:
    print("正しくsolverを指定してください")
    exit(-1)

try:
    test_data_dir = pathlib.Path(sys.argv[3])
except:
    print("正しくテスト用のデータを指定してください")
    exit(-1)

test_data_paths = test_data_dir.glob("*.cnf")

try:
    log_path = sys.argv[4]
except:
    log_path = "test_log"

log_path = pathlib.Path(log_path)

sample_log_dir = "sample_solver"
my_log_dir = "my_solver"

sample_log_path = pathlib.Path.joinpath(log_path, sample_log_dir)
my_log_path = pathlib.Path.joinpath(log_path, my_log_dir)

os.makedirs(sample_log_path, exist_ok=True)
os.makedirs(my_log_path, exist_ok=True)

for i, test_data_path in enumerate(test_data_paths):
    log_file = f"{i+1}.log"
    with open(pathlib.Path.joinpath(sample_log_path, log_file), 'w') as fp:
        subprocess.run(["./"+sample_solver, test_data_path], stdout=fp, stderr=fp)
    with open(pathlib.Path.joinpath(my_log_path, log_file), 'w') as fp:
        subprocess.run(["./"+my_solver, test_data_path], stdout=fp, stderr=fp)

# 出力結果の検証
unsat = "UNSATISFIABLE"
sat = "SATISFIABLE"
assigns_pattern = re.compile(r"Satisfying solution: (.+)")

sample_logs = sample_log_path.glob("*.cnf")
my_logs = my_log_path.glob("*.cnf")

for sample_log, my_log in zip(sample_logs, my_logs):
    assert(sample_log.name == my_log.name)

    with open(sample_log, 'r') as f:
        sample_log_data = f.read()

    with open(my_log, 'r') as f:
        my_log_data = f.read()

    try:
        if unsat in sample_log and unsat in my_log:
            continue
        elif sat in sample_log and sat in my_log:
            sample_assigns = assigns_pattern.match(sample_log)[1].strip()
            my_assigns = assigns_pattern.match(my_log)[1].strip()
            if sample_assigns == my_assigns:
                continue
            else:
                raise Exception
    except:
        print(f"invalid output at sample:{sample_log} my:{my_log}")