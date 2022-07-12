# exp2-a02-part3 program-a

## Requirements

- [rust-toolchain](https://www.rust-lang.org/ja/tools/install) ... for build and run
- [go](https://go.dev/dl/) ... for process csv
- csv files ... `csv/geotag.csv`, `csv/tag.csv`

## Benchmark on local

1. Process geotag.csv and tag.csv by using `process-csv.go`. After processing, `csv/new_geotag.csv` and `csv/tag.csv` will be created.

```shell
$ go run process-csv
$ ls csv/
geotag.csv  new_geotag.csv  new_tag.csv  tag.csv
```

2. Build and launch server

```shell
$ cargo build --release
$ ./target/release/program-a
tags: 41309136[B], geotgas: 499069008[B]
Li&stening on http://localhost:8080...
```