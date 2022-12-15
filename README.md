# exp2-a02-part3 program-a

## Requirements

- [rust-toolchain](https://www.rust-lang.org/ja/tools/install) ... for build and run
- [go](https://go.dev/dl/) ... for process csv
- csv files ... `csv/geotag.csv`, `csv/tag.csv`

## Benchmark on local

1. Process geotag.csv and tag.csv by using `process-csv.go`. After processing, `csv/tag.json` will be created.

```shell
$ ls csv/
geotag.csv tag.csv
$ go run tools/process-csv.go
$ ls csv/
geotag.csv tag.csv tag.json
```

2. Build and launch server

```shell
$ cargo build --release
$ ./target/release/program-a
Listening on http://localhost:8080...
```