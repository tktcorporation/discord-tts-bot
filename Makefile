# デフォルトターゲット
.PHONY: install
install:
	# Rustupの更新とインストール
	rustup update
	rustup install stable
	# 必要なコンポーネントの追加
	rustup component add clippy rustfmt rust-analysis rust-src rls
	rustup component add clippy
	# cargo-binstallのインストール
	cargo install cargo-binstall
	# sccacheのインストールと環境変数の設定
	cargo binstall sccache --locked && export RUSTC_WRAPPER=$$(which sccache)
	# その他ツールのインストール
	cargo binstall cargo-watch cargo-edit cargo-hack
	cargo install cargo-audit --features=fix

# 環境変数のエクスポート（必要に応じて使用）
.PHONY: setup-env
setup-env:
	@echo "Exporting environment variables..."
	export CARGO_HOME=${CARGO_HOME:-$$HOME/.cargo}
	export PATH=$$CARGO_HOME/bin:$$PATH

# テスト用ターゲット（オプション）
.PHONY: test-install
test-install: install
	@echo "Checking installed tools..."
	which rustup && rustup --version
	which cargo && cargo --version
	which sccache && sccache --version || echo "sccache not installed"
	@echo "All tools installed successfully."

all: fmt lint test-all
ci: lint hack test
fmt:
	cargo fmt --all
lint:
	cargo clippy --all --all-targets --all-features --fix --allow-dirty --allow-staged
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
