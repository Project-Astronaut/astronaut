# Makefile for Astronaut workspace

.PHONY: all build test fmt clippy clean run-server bench audit udeps coverage docker-build docker-run

all: build

build:
	cargo build --workspace

test:
	cargo test --workspace --all-features

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

clean:
	cargo clean

run-server:
	cargo run -p astronaut-server

bench:
	cargo bench --workspace || true

# Requires cargo-audit to be installed: cargo install cargo-audit
audit:
	cargo audit || true

# Requires cargo-udeps to be installed: cargo install cargo-udeps
udeps:
	cargo +nightly udeps --workspace || true

coverage:
	# Example using llvm-cov (requires cargo-llvm-cov)
	cargo llvm-cov --workspace --lcov --output-path lcov.info || true

# Docker
IMAGE?=astronaut-server:latest

docker-build:
	docker build -t $(IMAGE) .

docker-run:
	docker run --rm -p 8080:8080 -e RUST_LOG=info $(IMAGE)
