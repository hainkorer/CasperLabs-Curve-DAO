prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p curve_token_v3 --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/curve_token_v3.wasm 2>/dev/null | true

test-only:
	cargo test -p curve_token_v3_tests -- --nocapture

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm curve-token-v3/curve-token-v3-tests/wasm

test: build-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf curve_token_v3_tests/wasm/*.wasm
