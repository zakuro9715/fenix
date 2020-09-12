setup:
	cargo install bootimage
	rustup component add rust-src
	rustup component add llvm-tools-preview
build:
	cargo build
run:
	cargo run
