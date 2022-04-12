prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p vesting_escrow_simple --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/vesting_escrow_simple.wasm 2>/dev/null | true

test-only:
	cargo test -p vesting_escrow_simple_tests -- --nocapture

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm vesting_escrow_simple_tests/wasm

test: build-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf vesting_escrow_simple_tests/wasm/*.wasm
