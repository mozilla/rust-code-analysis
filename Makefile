.PHONY: build test check clean fmt

build:
	cargo build

build-release:
	cargo build --release

test:
	cargo test

check:
	cargo check

clean:
	cargo clean

fmt:
	cargo fmt
