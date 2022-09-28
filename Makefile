all: fmt lint test
fmt:
	cargo fmt --all
lint:
	cargo +nightly clippy --all --all-targets --all-features --fix -Z unstable-options --allow-dirty --allow-staged
hack:
	cargo hack check --each-feature --no-dev-deps --all
test:
	cargo test --all-features
watch:
	cargo watch --features tts,music -x fmt -x clippy
run:
	cargo run --all-features
install:
	rustup update
	rustup install nightly
	rustup component add clippy rustfmt rust-analysis rust-src rls
	cargo install cargo-watch
	rustup component add clippy --toolchain nightly-x86_64-unknown-linux-gnu
	cargo install cargo-edit
	cargo install cargo-hack
