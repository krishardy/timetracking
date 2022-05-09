.PHONY: update-build
update-build: update build

.PHONY: build
build:
	cargo build

.PHONY: update
update:
	cargo update

.PHONY: install
install:
	cargo install --path .

.PHONY: test
test:
	cargo test
	target/debug/timetracking.exe example.csv
	
.PHONY: clean
clean:
	cargo clean