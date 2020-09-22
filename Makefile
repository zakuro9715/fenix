run:
	cargo run

setup:
	cargo install bootimage
	rustup component add rust-src
	rustup component add llvm-tools-preview

build:
	cargo build

test:
	cargo test --lib

test-fail:
	cargo test --lib --features test_fail
