prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p erc20_crv --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/erc20_crv.wasm 2>/dev/null | true

build-session-code:
	cargo build --release -p session-code --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/session-code.wasm 2>/dev/null | true

test-only:
	cargo test -p erc20_crv_tests -- --nocapture

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm erc20_crv_tests/wasm

test: build-contract build-session-code  copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf erc20_crv_tests/wasm/*.wasm
