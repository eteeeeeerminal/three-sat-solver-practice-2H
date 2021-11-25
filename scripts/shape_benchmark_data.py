# benchmark のログを csv に変換
# solver_name, input_file, time

import sys
import pathlib
import re

try:
    log_root = pathlib.Path(sys.argv[1])
    benchmark_input = pathlib.Path(sys.argv[2])
    output_file = pathlib.Path(sys.argv[3])
except:
    print("引数error")
    exit(-1)

benchmark_input_files = list(benchmark_input.glob("*.cnf"))
log_dirs = [p for p in log_root.iterdir() if p.is_dir()]

output_text=""
cpu_time_pattern = re.compile(r"[\s\S]*CPU time\s+:\s*([\d.]+)\s*s[\s\S]*")
for log_dir in log_dirs:
    solver_name = log_dir.name
    log_files = log_dir.glob("*.log")
    for log_file, input_file in zip(log_files, benchmark_input_files):
        assert(f"{input_file.name}.log" == log_file.name)
        with open(log_file, 'r') as f:
            log_data = f.read()
        time_match = cpu_time_pattern.match(log_data)
        try:
            cpu_time = float(time_match[1])
        except:
            continue

        output_text += f"{solver_name},{input_file},{cpu_time}\n"

with open(output_file, 'w') as f:
    f.write(output_text)