.PHONY: build
build:
	cargo update
	cargo build

.PHONY: install
install:
	cargo install --path .

.PHONY: test
test:
	cargo test
	target/debug/timetracking.exe example.
	
.PHONY: clean
clean:
	cargo clean