# 今回のソルバーは'%'等が入っていると止まるので、それを取り除く

import sys
import re
import os
import pathlib

file_ext = '.cnf'

input_dir = sys.argv[1]
if not input_dir:
    print("error invalid input dir")
    exit(0)

if len(sys.argv) == 3:
    output_dir = sys.argv[2]
else:
    output_dir = "test"

input_dir = pathlib.Path(input_dir)
output_dir = pathlib.Path(output_dir)
files = input_dir.glob(f"*{file_ext}")

os.makedirs(output_dir, exist_ok=True)

def remove_invalid_char(file_path: pathlib.Path) -> None:
    with open(file_path, 'r', encoding='utf-8') as f:
        text = f.read()
    text = re.sub(r"%[\s\S]*", "", text)
    file_name = file_path.name
    with open(pathlib.Path.joinpath(output_dir, file_name), 'w', encoding='utf-8') as f:
        f.write(text)

for file in files:
    remove_invalid_char(file)