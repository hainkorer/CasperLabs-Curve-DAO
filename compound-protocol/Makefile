prepare:
	rustup target add wasm32-unknown-unknown

# builds all compound protocol contracts
# wasm-strips all wasms
build-contract:
	cd contract && cargo build --release --target wasm32-unknown-unknown
	chmod +x wasm-strip-cmd.sh && ./wasm-strip-cmd.sh

test: build-contract
	mkdir -p tests/wasm
	cp contract/target/wasm32-unknown-unknown/release/contract.wasm tests/wasm
	cd tests && cargo test

clippy:
	cd contract && cargo clippy --all-targets -- -D warnings
	cd tests && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd contract && cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy
	cd contract && cargo fmt
	cd tests && cargo fmt

clean:
	cd contract && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
