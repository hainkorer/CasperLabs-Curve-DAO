root_directory = ./

wasm_src_path = target/wasm32-unknown-unknown/release/
wasm_dest_voting_escrow_path = ./voting-escrow/voting-escrow-tests/wasm/

build-contract:
	cargo build --release -p voting-escrow -p erc20 --target wasm32-unknown-unknown

copy-wasm-file:
	cp ${root_directory}${wasm_src_path}*.wasm ${wasm_dest_voting_escrow_path}

test-only:
	cargo test -p voting-escrow-tests

test:
	make build-contract && make copy-wasm-file && make test-only

clean:
	cargo clean
	rm -rf Cargo.lock
	rm -rf ${wasm_dest_voting_escrow_path}*.wasm