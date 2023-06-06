csv/tag.json:
	@echo 'Generating json file which is merged tag.csv and geotag.csv...'
	@go run ./tools/process-csv.go

.PHONY: run
run: csv/tag.json
	@echo 'Launching server...'
	@cargo run --release
