all: fmt lint test
fmt:
	cargo fmt --all
lint:
	cargo +nightly clippy --all --all-targets --all-features --fix -Z unstable-options --allow-dirty --allow-staged
test:
	cargo test --all-features
watch:
	cargo watch --features tts,music -x fmt -x clippy
run:
	cargo run --all-features