all: fmt lint test-all
ci: lint hack test
fmt:
	cargo fmt --all
lint:
	cargo +nightly clippy --all --all-targets --all-features --fix -Z unstable-options --allow-dirty --allow-staged
hack:
	cargo hack check --each-feature --no-dev-deps --all
test-all:
	cargo test --all-features -- --include-ignored
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
	rustup component add clippy --toolchain nightly-x86_64-unknown-linux-gnu
	cargo install cargo-binstall
	cargo binstall sccache --locked && export RUSTC_WRAPPER=$(which sccache)
	cargo binstall cargo-watch cargo-edit cargo-hack
	cargo install cargo-audit --features=fix
