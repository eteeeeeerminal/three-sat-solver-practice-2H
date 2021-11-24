# 指定した2つのログファイルディレクトリのログファイルに記載されている
# 実行結果が一致していることを確認する

import sys
import re
import pathlib

try:
    sample_log_path = pathlib.Path(sys.argv[1])
    my_log_path = pathlib.Path(sys.argv[2])
except:
    print("正しくログファイルのあるディレクトリを指定してください")
    exit(-1)

# 出力結果の検証
unsat = "UNSATISFIABLE"
sat = "SATISFIABLE"
assigns_pattern = re.compile(r"[\s\S]*Satisfying solution: ([\s\S]+)")

sample_logs = sample_log_path.glob("*.log")
my_logs = my_log_path.glob("*.log")

for sample_log, my_log in zip(sample_logs, my_logs):
    assert(sample_log.name == my_log.name)

    with open(sample_log, 'r') as f:
        sample_log_data = f.read()

    with open(my_log, 'r') as f:
        my_log_data = f.read()

    try:
        if unsat in sample_log_data and unsat in my_log_data:
            continue
        elif sat in sample_log_data and sat in my_log_data:
            sample_assigns = assigns_pattern.match(sample_log_data)[1].strip()
            my_assigns = assigns_pattern.match(my_log_data)[1].strip()
            if sample_assigns == my_assigns:
                continue
            else:
                raise Exception
    except:
        print(f"invalid output at sample:{sample_log} my:{my_log}")

print("complete all test done")