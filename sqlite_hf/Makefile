.PHONY: rust-version install format lint test run release all

rust-version:
	@echo "Rust command utility versions:"
	rustc --version			# Rust compiler
	cargo --version			# Rust package manager
	rustfmt --version		# Rust code formatter
	rustup --version		# Rust toolchain manager
	clippy-driver --version	# Rust linter

install:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run Cargo.toml

release:
	cargo build --release

all: format lint test run