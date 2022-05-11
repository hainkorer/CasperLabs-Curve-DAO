root_directory = ./

wasm_src_path = target/wasm32-unknown-unknown/release/
wasm_dest_liquidity_gauge_path = ./gauges/liquidity-gauge/liquidity-gauge-tests/wasm/
wasm_dest_voting_escrow_path = ./voting-escrow/voting-escrow-tests/wasm/

copy-wasm-file:
	cp ${root_directory}${wasm_src_path}*.wasm ${wasm_dest_liquidity_gauge_path}
	cp ${root_directory}${wasm_src_path}*.wasm ${wasm_dest_voting_escrow_path}

build-contract:
	cargo build --release -p voting-escrow --target wasm32-unknown-unknown

test:
	make build-contract && make copy-wasm-file && cargo test -p voting-escrow-tests

clean:
	cargo clean
	rm -rf Cargo.lock