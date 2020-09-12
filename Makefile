TARGET := x86_64-fenix.json
BUILD_ARGS = --target $(TARGET)

setup:
	rustup component add rust-src
	rustup component add llvm-tools-preview
build:
	cargo build
run:
	cargo run
