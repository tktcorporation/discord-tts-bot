lint:
	cargo clippy -Z unstable-options --all --all-targets --all-features --fix
watch:
	cargo watch --features tts,music
run:
	cargo run --features tts,music