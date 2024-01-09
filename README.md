# three-sat-solver-practice-2H
minisatを参考にして Rust で作った素朴な3satソルバー

minisat: https://github.com/niklasso/minisat

授業で発表した性能比較等の資料: [./slide.pdf](./slide.pdf)

# ビルド方法
Rust の公式サイトを参考に、rustup をインストール
- https://www.rust-lang.org/ja/tools/install
その後以下のコマンドを打つと、target/release 以下に実行ファイルが生成される
```
cargo build --release
```

# ファイル構成
- `src` - 自作の sat solver のソースコード
- `scripts` - テストを実行したり、グラフを生成したりするスクリプト
- `solvers` - ビルドした solver の実行ファイル
  - `minisat` - release ビルドした minisat
  - `simple_minisat` - 授業で与えられた 単純な minisat を release ビルドしたもの
  - `my_simple_minisat` - `simple_minisat` を Rust で実装しなおしたものを release ビルドしたもの
  - `my_solver_with_propagate` - `my_simple_minisat` に監視リテラルを実装して release ビルドしたもの

# 動作検証、ベンチマーク
使用したデータは SATLIB - Benchmark Problems (https://www.cs.ubc.ca/~hoos/SATLIB/benchm.html) からダウンロードした。

データを用意した状態で、`run.sh` を実行すると、後述するベンチマーク2つが走り、グラフが生成される。

## 動作検証
- `test` - テスト用の入力データ
  - `uf50-218` と `uuf50-218` から最初の10個をそれぞれ用意
- `test_log` - テストの出力データ
- `script/run_test.sh` で各 solver を実行して、ログを比較して同じ結果が出力されているか確認

## ベンチマーク1
- `simple_minisat` と `my_simple_solver` を比較
- `benchmark_input` - ベンチマーク用の入力データ
  - `uf50-218` を使用
- `benchmark_log` - ベンチマークの出力データ
- `script/run_benchmark1.sh` でベンチマークを実行、`timeout 1000` で実行
- `python3 benchmark_log benchmark_input benchmark.csv` で log ファイルを csv にまとめる
- `gnuplot script/cactus1.plt` で `figure` 下にグラフファイルを生成
- `simple_minisat` は70問、`my_simple_solver` は80問解けた

## ベンチマーク2
- `minisat` と `my_solver_with_propagate` を比較
- `benchmark_input2` - ベンチマーク用の入力データ
  - `uf100-430` を使用
- `benchmark_log2` - ベンチマークの出力データ
- `script/run_benchmark2.sh` でベンチマークを実行、`timeout 10` で実行
- `python3 benchmark_log2 benchmark_input2 benchmark2.csv` で log ファイルを csv にまとめる
- `gnuplot script/cactus2.plt` で `figure` 下にグラフファイルを生成
- `minisat` は404問、`my_solver_with_propagate` は35問解けた