run:
	cargo run

setup:
	cargo install bootimage
	rustup component add rust-src
	rustup component add llvm-tools-preview

build:
	cargo build

test:
	cargo test

test-fail:
	cargo test --features test_fail
