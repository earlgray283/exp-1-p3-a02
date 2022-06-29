# exp2-a02-part3 program-a

## Requirements

- [rust-toolchain](https://www.rust-lang.org/ja/tools/install) ... for build and run
- [hyperfine](https://github.com/sharkdp/hyperfine) ... for benchmark on local
- csv files ... `csv/geotag.csv`, `csv/tag.csv`

## Benchmark on local

```shell
$ cargo build --release && hyperfine --min-runs 1 --show-output "./target/release/program-a $(shuf -n 1 ../csv/tag.csv | cut -d, -f2)" 
```