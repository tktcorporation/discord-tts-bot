lint:
	cargo +nightly clippy --all --all-targets --all-features --fix -Z unstable-options --allow-dirty
watch:
	cargo watch --features tts,music -x fmt
run:
	cargo run --features tts,music