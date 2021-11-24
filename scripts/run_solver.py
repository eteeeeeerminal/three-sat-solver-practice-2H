import sys
import os
import pathlib
import subprocess

# 実行部分
try:
    solver = sys.argv[1]
except:
    print("正しくsolverを指定してください")
    exit(-1)

try:
    test_data_dir = pathlib.Path(sys.argv[2])
except:
    print("正しくテスト用のデータを指定してください")
    exit(-1)

test_data_paths = test_data_dir.glob("*.cnf")

try:
    log_path = sys.argv[3]
except:
    print("正しくログファイルのあるディレクトリを指定してください")
    exit(-1)

log_path = pathlib.Path(log_path)

os.makedirs(log_path, exist_ok=True)

for test_data_path in test_data_paths:
    log_file = f"{test_data_path.name}.log"
    with open(pathlib.Path.joinpath(log_path, log_file), 'w') as fp:
        subprocess.run(["./"+solver, test_data_path], stdout=fp, stderr=fp)
