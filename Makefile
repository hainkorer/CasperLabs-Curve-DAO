src_target = target/wasm32-unknown-unknown/release
# liquid_locker_des_wasm = liquid-locker/liquid-locker-tests/wasm
# liquid_helper_des_wasm = liquid-helper/liquid-helper-tests/wasm
minter_des_wasm = minter/minter-tests/wasm
gauge_controller_des_wasm = gauge-controller/gauge-controller-tests/wasm
reward_only_gauge_des_wasm = reward-only-gauge/reward-only-gauge-tests/wasm
vesting_escrow_des_wasm = vesting-escrow/vesting-escrow-tests/wasm


prepare:
	rustup target add wasm32-unknown-unknown

build-contract-minter:
	cargo build --release -p minter -p minter-proxy --target wasm32-unknown-unknown
build-contract-gauge-controller:
	cargo build --release -p gauge-controller -p gauge-controller-proxy --target wasm32-unknown-unknown
build-contract-reward-only-gauge:
	cargo build --release -p reward-only-gauge -p reward-only-gauge-proxy --target wasm32-unknown-unknown
build-contract-vesting-escrow:
	cargo build --release -p vesting-escrow -p vesting-escrow-proxy --target wasm32-unknown-unknown


test-only-minter:
	cargo test -p minter-tests
test-only-gauge-controller:
	cargo test -p gauge-controller-tests
test-only-reward-only-gauge:
	cargo test -p reward-only-gauge-tests
test-only-vesting-escrow:
	cargo test -p vesting-escrow-tests


copy-wasm-file-minter:
	cp ${src_target}/minter-token.wasm ${minter_des_wasm}
	cp ${src_target}/minter-proxy-token.wasm ${minter_des_wasm}
copy-wasm-file-gauge-controller:
	cp ${src_target}/gauge-controller-token.wasm ${gauge_controller_des_wasm}
	cp ${src_target}/gauge-controller-proxy-token.wasm ${gauge_controller_des_wasm}
copy-wasm-file-reward-only-gauge:
	cp ${src_target}/reward-only-gauge-token.wasm ${reward_only_gauge_des_wasm}
	cp ${src_target}/reward-only-gauge-proxy-token.wasm ${reward_only_gauge_des_wasm}
copy-wasm-file-vesting-escrow:
	cp ${src_target}/vesting-escrow-token.wasm ${vesting_escrow_des_wasm}
	cp ${src_target}/vesting-escrow-proxy-token.wasm ${vesting_escrow_des_wasm}

test-minter:
	make build-contract-minter && make copy-wasm-file-minter
test-gauge-controller:
	make build-contract-gauge-controller && make copy-wasm-file-gauge-controller
test-reward-only-gauge:
	make build-contract-reward-only-gauge && make copy-wasm-file-reward-only-gauge
test-vesting-escrow:
	make build-contract-vesting-escrow && make copy-wasm-file-vesting-escrow

all:
	make test-minter && make test-only-minter
	make test-gauge-controller && make test-only-gauge-controller
	make test-reward-only-gauge && make test-only-reward-only-gauge
	make test-vesting-escrow && make test-only-vesting-escrow

clean:
	cargo clean
	rm -rf minter/minter-tests/wasm/*.wasm
	rm -rf gauge-controller/gauge-controller-tests/wasm/*.wasm
	rm -rf reward-only-gauge/reward-only-gauge-tests/wasm/*.wasm
	rm -rf vesting-escrow/vesting-escrow-tests/wasm/*.wasm
	rm -rf Cargo.lock