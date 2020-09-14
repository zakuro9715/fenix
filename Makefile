run:
	cargo run

setup:
	cargo install bootimage
	rustup component add rust-src
	rustup component add llvm-tools-preview
build:
	cargo build
