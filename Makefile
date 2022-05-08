root_directory = ./

wasm_src_path = target/wasm32-unknown-unknown/release/
wasm_dest_voting_escrow_path = ./voting-escrow/voting-escrow-tests/wasm/
wasm_dest_fee_distributor_path = ./fee-distributor/fee-distributor-tests/wasm/
wasm_dest_liquidity_gauge_reward_path = ./liquidity-gauge-reward/liquidity-gauge-reward-tests/wasm/

build-session-code:
	cargo build --release -p session-code --target wasm32-unknown-unknown

build-contract-voting-escrow:
	cargo build --release -p voting-escrow -p erc20 --target wasm32-unknown-unknown
build-contract-fee-distributor:
	cargo build --release -p fee-distributor --target wasm32-unknown-unknown
build-contract-liquidity-gauge-reward:
	cargo build --release -p liquidity-gauge-reward --target wasm32-unknown-unknown

copy-wasm-file-voting-escrow:
	cp ${root_directory}${wasm_src_path}*.wasm ${wasm_dest_voting_escrow_path}
copy-wasm-file-fee-distributor:
	cp ${root_directory}${wasm_src_path}*.wasm ${wasm_dest_fee_distributor_path}
copy-wasm-file-liquidity-gauge-reward:
	cp ${root_directory}${wasm_src_path}*.wasm ${wasm_dest_liquidity_gauge_reward_path}

test-only-voting-escrow:
	cargo test -p voting-escrow-tests
test-only-fee-distributor:
	cargo test -p fee-distributor-tests
test-only-liquidity-gauge-reward:
	cargo test -p liquidity-gauge-reward-tests

test-voting-escrow:
	make build-session-code && make build-contract-voting-escrow && make copy-wasm-file-voting-escrow && make test-only-voting-escrow
test-fee-distributor:
	make build-session-code && make build-contract-fee-distributor && make copy-wasm-file-fee-distributor && make test-only-fee-distributor
test-liquidity-gauge-reward:
	make build-session-code && make build-contract-liquidity-gauge-reward && make copy-wasm-file-liquidity-gauge-reward && make test-only-liquidity-gauge-reward

all:
	make test-voting-escrow && make test-fee-distributor && make test-liquidity-gauge-reward

clean:
	cargo clean
	rm -rf Cargo.lock
	rm -rf ${wasm_dest_fee_distributor_path}*.wasm
	rm -rf ${wasm_dest_voting_escrow_path}*.wasm
	rm -rf ${wasm_dest_liquidity_gauge_reward_path}*.wasm