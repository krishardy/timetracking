.PHONY: build
build:
	cargo update
	cargo build

.PHONY: install
install:
	cargo install --path .
	