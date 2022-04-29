root_directory = ./

wasm_src_path = target/wasm32-unknown-unknown/release/
wasm_dest_voting_escrow_path = ./voting-escrow/voting-escrow-tests/wasm/
wasm_dest_fee_distributor_path = ./fee-distributor/fee-distributor-tests/wasm/

build-session-code:
	cargo build --release -p session-code --target wasm32-unknown-unknown

build-contract-voting-escrow:
	cargo build --release -p voting-escrow -p erc20 --target wasm32-unknown-unknown
build-contract-fee-distributor:
	cargo build --release -p fee-distributor --target wasm32-unknown-unknown

copy-wasm-file-voting-escrow:
	cp ${root_directory}${wasm_src_path}*.wasm ${wasm_dest_voting_escrow_path}
copy-wasm-file-fee-distributor:
	cp ${root_directory}${wasm_src_path}*.wasm ${wasm_dest_fee_distributor_path}

test-only-voting-escrow:
	cargo test -p voting-escrow-tests
test-only-fee-distributor:
	cargo test -p fee-distributor-tests

test-voting-escrow:
	make build-session-code && make build-contract-voting-escrow && make copy-wasm-file-voting-escrow && make test-only-voting-escrow
test-fee-distributor:
	make build-session-code && make build-contract-fee-distributor && make copy-wasm-file-fee-distributor && make test-only-fee-distributor

all:
	make test-voting-escrow && make test-fee-distributor

clean:
	cargo clean
	rm -rf Cargo.lock
	rm -rf ${wasm_dest_fee_distributor_path}*.wasm
	rm -rf ${wasm_dest_voting_escrow_path}*.wasm