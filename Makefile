wasm_src_path = ./target/wasm32-unknown-unknown/release/

curve_token_v3_des_wasm = ./curve-token-v3/curve-token-v3-tests/wasm
erc20_des_wasm = ./erc20/erc20-tests/wasm
erc20_crv_des_wasm = ./erc20-crv/erc20_crv_tests/wasm
fee_distributor_des_wasm = ./fee-distributor/fee-distributor-tests/wasm
gauge_controller_des_wasm = ./gauge-controller/gauge-controller-tests/wasm
gauge_proxy_des_wasm = ./gauge-proxy/gauge-proxy-tests/wasm
liquidity_gauge_reward_des_wasm = ./liquidity-gauge-reward/liquidity-gauge-reward-tests/wasm
liquidity_gauge_reward_wrapper_des_wasm = ./liquidity-gauge-reward-wrapper/liquidity-gauge-reward-wrapper-tests/wasm
liquidity_gauge_wrapper_des_wasm = ./liquidity-gauge-wrapper/liquidity-gauge-wrapper-tests/wasm
minter_des_wasm = ./minter/minter-tests/wasm
reward_only_gauge_des_wasm = ./reward-only-gauge/reward-only-gauge-tests/wasm
vesting_escrow_des_wasm = ./vesting-escrow/vesting-escrow-tests/wasm
vesting_escrow_factory_des_wasm = ./vesting-escrow-factory/vesting-escrow-factory-tests/wasm
liquidity_gauge_v3_des_wasm = ./liquidity-gauge-v3/liquidity-gauge-v3-tests/wasm/
voting_escrow_des_wasm = ./voting-escrow/voting-escrow-tests/wasm
ownable_des_wasm = ./ownable/ownable-tests/wasm/
i_reward_distribution_recipient_des_wasm = ./i-reward-distribution-recipient/i-reward-distribution-recipient-tests/wasm/
lp_token_wrapper_des_wasm = ./lp-token-wrapper/lp-token-wrapper-tests/wasm/
curve_rewards_des_wasm = ./curve-rewards/curve-rewards-tests/wasm/

prepare:
	rustup target add wasm32-unknown-unknown
build-contract-curve-token-v3:
	cargo build --release -p curve-token-v3 -p erc20 -p curve-rewards -p curve-token-v3-proxy --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/curve-token-v3.wasm 2>/dev/null | true
build-liquidity-gauge-reward-wrapper-session-code:
	cargo build --release -p liquidity-gauge-reward-wrapper-session-code --target wasm32-unknown-unknown
build-i-reward-distribution-recipient:
	cargo build --release -p i-reward-distribution-recipient --target wasm32-unknown-unknown
build-liquidity-gauge-wrapper-session-code:
	cargo build --release -p liquidity-gauge-wrapper-session-code --target wasm32-unknown-unknown	
build-contract-erc20:
	cargo build --release -p erc20-proxy -p erc20 --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/erc20-token.wasm 2>/dev/null | true
build-contract-erc20-crv:
	cargo build --release -p erc20-crv-session-code -p erc20_crv --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/erc20_crv.wasm 2>/dev/null | true
build-contract-fee-distributor:
	cargo build --release -p session-code -p erc20 -p voting-escrow -p fee-distributor --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/fee-distributor.wasm 2>/dev/null | true
build-contract-gauge-controller:
	cargo build --release -p gauge-controller-session-code -p erc20 -p voting-escrow -p gauge-controller --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/gauge-controller-token.wasm 2>/dev/null | true
build-contract-gauge-proxy:
	cargo build --release -p gauge-proxy --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/gauge-proxy.wasm 2>/dev/null | true
build-contract-liquidity-gauge-reward:
	cargo build --release -p curve-rewards -p session-code -p erc20_crv -p erc20 -p voting-escrow -p gauge-controller  -p minter -p liquidity-gauge-reward --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/liquidity-gauge-reward.wasm 2>/dev/null | true
build-contract-liquidity-gauge-reward-wrapper:
	cargo build --release -p erc20_crv -p erc20 -p minter -p voting-escrow -p gauge-controller -p liquidity-gauge-reward -p liquidity-gauge-reward-wrapper -p liquidity-gauge-reward-wrapper-session-code -p curve-rewards --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/liquidity-gauge-reward-wrapper.wasm 2>/dev/null | true
build-contract-liquidity-gauge-wrapper:
	cargo build --release -p erc20_crv -p erc20 -p minter -p liquidity-gauge-v3 -p voting-escrow -p gauge-controller -p liquidity-gauge-wrapper-session-code -p liquidity-gauge-wrapper --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/liquidity-gauge-wrapper.wasm 2>/dev/null | true
build-contract-minter:
	cargo build --release -p erc20 -p erc20_crv -p erc20-crv-session-code -p liquidity-gauge-reward -p voting-escrow -p gauge-controller -p minter --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/minter-token.wasm 2>/dev/null | true
build-contract-reward-only-gauge:
	cargo build --release -p erc20 -p erc20_crv -p curve-rewards -p reward-only-gauge -p reward-only-gauge-session-code --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/reward-only-gauge.wasm 2>/dev/null | true
build-contract-vesting-escrow:
	cargo build --release -p vesting-escrow-session-code -p erc20 -p vesting-escrow --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/vesting-escrow.wasm 2>/dev/null | true
build-contract-vesting-escrow-factory:
	cargo build --release -p vesting-escrow-simple -p erc20 -p vesting-escrow-factory --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/vesting-escrow-factory.wasm 2>/dev/null | true

build-contract-liquidity-gauge-v3:
	cargo build --release -p liquidity-gauge-v3-session-code -p liquidity-gauge-v3 -p erc20 -p minter -p voting-escrow -p gauge-controller -p erc20_crv  --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/liquidity-gauge-v3.wasm 2>/dev/null | true
build-contract-vesting-escrow-simple:
	cargo build --release -p erc20 -p vesting-escrow-simple --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/vesting-escrow-simple.wasm 2>/dev/null | true
build-contract-voting-escrow:
	cargo build --release -p session-code -p erc20 -p voting-escrow --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/vesting_escrow_simple.wasm 2>/dev/null | true
build-contract-ownable:
	cargo build --release -p ownable --target wasm32-unknown-unknown
build-contract-ownable-test-contract:
	cargo build --release -p test --target wasm32-unknown-unknown
build-lp-token-wrapper-session-code:
	cargo build --release -p lp-token-wrapper-session-code --target wasm32-unknown-unknown
build-lp-token-wrapper:
	cargo build --release -p lp-token-wrapper --target wasm32-unknown-unknown
build-curve-rewards-session-code:
	cargo build --release -p curve-rewards-session-code --target wasm32-unknown-unknown
build-curve-rewards:
	cargo build --release -p erc20 -p curve-rewards --target wasm32-unknown-unknown

test-only-curve-token-v3:
	cargo test -p curve-token-v3-tests
test-only-erc20:
	cargo test -p erc20-tests
test-only-erc20-crv:
	cargo test -p erc20_crv_tests
test-only-fee-distributor:
	cargo test -p fee-distributor-tests
test-only-gauge-controller:
	cargo test -p gauge-controller-tests
test-only-gauge-proxy:
	cargo test -p gauge-proxy-tests
test-only-liquidity-gauge-reward:
	cargo test -p liquidity-gauge-reward-tests
test-only-liquidity-gauge-reward-wrapper:
	cargo test -p liquidity-gauge-reward-wrapper-tests
test-only-liquidity-gauge-wrapper:
	cargo test -p liquidity-gauge-wrapper-tests
test-only-minter:
	cargo test -p minter-tests
test-only-reward-only-gauge:
	cargo test -p reward-only-gauge-tests
test-only-vesting-escrow:
	cargo test -p vesting-escrow-tests
test-only-vesting-escrow-factory:
	cargo test -p vesting-escrow-factory-tests
test-only-vesting-escrow-simple:
	cargo test -p vesting-escrow-simple-tests
test-only-voting-escrow:
	cargo test -p voting-escrow-tests
test-only-liquidity-gauge-v3:
	cargo test -p liquidity-gauge-v3-tests
test-only-i-reward-distribution-recipient:
	cargo test -p i-reward-distribution-recipient-tests
test-only-ownable:
	cargo test -p ownable-tests 
test-only-lp-token-wrapper:
	cargo test -p lp-token-wrapper-tests
test-only-curve-rewards:
	cargo test -p curve-rewards-tests 

copy-wasm-file-curve-token-v3:
	cp ${wasm_src_path}/curve-token-v3.wasm ${curve_token_v3_des_wasm}
	cp ${wasm_src_path}/crv3-proxy-token.wasm ${curve_token_v3_des_wasm}
	cp ${wasm_src_path}/erc20-token.wasm ${curve_token_v3_des_wasm}
	cp ${wasm_src_path}/curve-rewards.wasm ${curve_token_v3_des_wasm}
copy-wasm-file-erc20:
	cp ${wasm_src_path}/erc20-proxy-token.wasm ${erc20_des_wasm}
	cp ${wasm_src_path}/erc20-token.wasm ${erc20_des_wasm}
copy-wasm-file-erc20-crv:
	cp ${wasm_src_path}/erc20-crv-session-code.wasm ${erc20_crv_des_wasm}
	cp ${wasm_src_path}/erc20_crv.wasm ${erc20_crv_des_wasm}
copy-wasm-file-fee-distributor:
	cp ${wasm_src_path}/session-code.wasm ${fee_distributor_des_wasm}
	cp ${wasm_src_path}/erc20-token.wasm ${fee_distributor_des_wasm}
	cp ${wasm_src_path}/voting-escrow.wasm ${fee_distributor_des_wasm}
	cp ${wasm_src_path}/fee-distributor.wasm ${fee_distributor_des_wasm}
copy-wasm-file-gauge-controller:
	cp ${wasm_src_path}/erc20-token.wasm ${gauge_controller_des_wasm}
	cp ${wasm_src_path}/voting-escrow.wasm ${gauge_controller_des_wasm}
	cp ${wasm_src_path}/gauge-controller-token.wasm ${gauge_controller_des_wasm}
	cp ${wasm_src_path}/gauge-controller-session-code.wasm ${gauge_controller_des_wasm}
copy-wasm-file-gauge-proxy:
	cp ${wasm_src_path}/gauge-proxy.wasm ${gauge_proxy_des_wasm}
copy-wasm-file-liquidity-gauge-reward:
	cp ${wasm_src_path}/session-code.wasm ${liquidity_gauge_reward_des_wasm}
	cp ${wasm_src_path}/erc20_crv.wasm ${liquidity_gauge_reward_des_wasm}
	cp ${wasm_src_path}/erc20-token.wasm ${liquidity_gauge_reward_des_wasm}
	cp ${wasm_src_path}/voting-escrow.wasm ${liquidity_gauge_reward_des_wasm}
	cp ${wasm_src_path}/gauge-controller-token.wasm ${liquidity_gauge_reward_des_wasm}
	cp ${wasm_src_path}/minter-token.wasm ${liquidity_gauge_reward_des_wasm}
	cp ${wasm_src_path}/liquidity-gauge-reward.wasm ${liquidity_gauge_reward_des_wasm}
	cp ${wasm_src_path}/curve-rewards.wasm ${liquidity_gauge_reward_des_wasm}
copy-wasm-file-liquidity-gauge-reward-wrapper:
	cp ${wasm_src_path}/erc20-token.wasm ${liquidity_gauge_reward_wrapper_des_wasm}
	cp ${wasm_src_path}/liquidity-gauge-reward-wrapper.wasm ${liquidity_gauge_reward_wrapper_des_wasm}
	cp ${wasm_src_path}/liquidity-gauge-reward-wrapper-session-code.wasm ${liquidity_gauge_reward_wrapper_des_wasm}
	cp ${wasm_src_path}/minter-token.wasm ${liquidity_gauge_reward_wrapper_des_wasm}
	cp ${wasm_src_path}/liquidity-gauge-reward.wasm ${liquidity_gauge_reward_wrapper_des_wasm}
	cp ${wasm_src_path}/curve-rewards.wasm ${liquidity_gauge_reward_wrapper_des_wasm}
	cp ${wasm_src_path}/gauge-controller-token.wasm ${liquidity_gauge_reward_wrapper_des_wasm}
	cp ${wasm_src_path}/voting-escrow.wasm ${liquidity_gauge_reward_wrapper_des_wasm}
	cp ${wasm_src_path}/erc20_crv.wasm ${liquidity_gauge_reward_wrapper_des_wasm}
copy-wasm-file-liquidity-gauge-wrapper:
	cp ${wasm_src_path}/erc20-token.wasm ${liquidity_gauge_wrapper_des_wasm}
	cp ${wasm_src_path}/liquidity-gauge-wrapper.wasm ${liquidity_gauge_wrapper_des_wasm}
	cp ${wasm_src_path}/liquidity-gauge-wrapper-session-code.wasm ${liquidity_gauge_wrapper_des_wasm}
	cp ${wasm_src_path}/minter-token.wasm ${liquidity_gauge_wrapper_des_wasm}
	cp ${wasm_src_path}/liquidity-gauge-v3.wasm ${liquidity_gauge_wrapper_des_wasm}
	cp ${wasm_src_path}/gauge-controller-token.wasm ${liquidity_gauge_wrapper_des_wasm}
	cp ${wasm_src_path}/voting-escrow.wasm ${liquidity_gauge_wrapper_des_wasm}
	cp ${wasm_src_path}/erc20_crv.wasm ${liquidity_gauge_wrapper_des_wasm}
copy-wasm-file-minter:
	cp ${wasm_src_path}/erc20_crv.wasm ${minter_des_wasm}
	cp ${wasm_src_path}/erc20-token.wasm ${minter_des_wasm}
	cp ${wasm_src_path}/voting-escrow.wasm ${minter_des_wasm}
	cp ${wasm_src_path}/minter-token.wasm ${minter_des_wasm}
	cp ${wasm_src_path}/gauge-controller-token.wasm ${minter_des_wasm}
	cp ${wasm_src_path}/liquidity-gauge-reward.wasm ${minter_des_wasm}
	cp ${wasm_src_path}/curve-rewards.wasm ${minter_des_wasm}
copy-wasm-file-reward-only-gauge:
	cp ${wasm_src_path}/erc20_crv.wasm ${reward_only_gauge_des_wasm}
	cp ${wasm_src_path}/reward-only-gauge-token.wasm ${reward_only_gauge_des_wasm}
	cp ${wasm_src_path}/reward-only-gauge-session-code.wasm ${reward_only_gauge_des_wasm}
	cp ${wasm_src_path}/curve-rewards.wasm ${reward_only_gauge_des_wasm}
	
copy-wasm-file-vesting-escrow:
	cp ${wasm_src_path}/erc20-token.wasm ${vesting_escrow_des_wasm}
	cp ${wasm_src_path}/vesting-escrow-token.wasm ${vesting_escrow_des_wasm}
	cp ${wasm_src_path}/vesting-escrow-session-code.wasm ${vesting_escrow_des_wasm}
copy-wasm-file-vesting-escrow-factory:
	cp ${wasm_src_path}/erc20-token.wasm ${vesting_escrow_factory_des_wasm}
	cp ${wasm_src_path}/vesting-escrow-factory-token.wasm ${vesting_escrow_factory_des_wasm}
copy-wasm-file-voting-escrow:
	cp ${wasm_src_path}/session-code.wasm ${voting_escrow_des_wasm}
	cp ${wasm_src_path}/erc20-token.wasm ${voting_escrow_des_wasm}
	cp ${wasm_src_path}/voting-escrow.wasm ${voting_escrow_des_wasm}
	cp ${wasm_src_path}/*.wasm ${voting_escrow_des_wasm}

copy-wasm-file-liquidity-gauge-v3:
	cp ${root_directory}${wasm_src_path}liquidity-gauge-v3.wasm ${liquidity_gauge_v3_des_wasm}
	cp ${root_directory}${wasm_src_path}erc20-token.wasm ${liquidity_gauge_v3_des_wasm}
	cp ${root_directory}${wasm_src_path}erc20_crv.wasm ${liquidity_gauge_v3_des_wasm}
	cp ${root_directory}${wasm_src_path}gauge-controller-token.wasm ${liquidity_gauge_v3_des_wasm}
	cp ${root_directory}${wasm_src_path}minter-token.wasm ${liquidity_gauge_v3_des_wasm}
	cp ${root_directory}${wasm_src_path}voting-escrow.wasm ${liquidity_gauge_v3_des_wasm}
	cp ${root_directory}${wasm_src_path}liquidity_gauge_v3_session_code.wasm ${liquidity_gauge_v3_des_wasm}
copy-wasm-file-ownable:
	cp ${wasm_src_path}/ownable_test.wasm ${ownable_des_wasm}
	cp ${wasm_src_path}/ownable.wasm ${ownable_des_wasm}
copy-wasm-file-i-reward-distribution-recipient:
	cp ${wasm_src_path}/i-reward-distribution-recipient.wasm ${i_reward_distribution_recipient_des_wasm}
copy-wasm-file-lp-token-wrapper:
	cp ${wasm_src_path}/erc20-token.wasm ${lp_token_wrapper_des_wasm}
	cp ${wasm_src_path}/lp-token-wrapper.wasm ${lp_token_wrapper_des_wasm}
	cp ${wasm_src_path}/lp-token-wrapper-session-code.wasm ${lp_token_wrapper_des_wasm}
copy-wasm-file-curve-rewards:
	cp ${wasm_src_path}/erc20-token.wasm ${curve_rewards_des_wasm}
	cp ${wasm_src_path}/curve-rewards.wasm ${curve_rewards_des_wasm}
	cp ${wasm_src_path}/curve-rewards-session-code.wasm ${curve_rewards_des_wasm}

test-gauge-proxy:
	make build-contract-gauge-proxy && make copy-wasm-file-gauge-proxy && make test-only-gauge-proxy
test-liquidity-gauge-reward:
	make build-contract-liquidity-gauge-reward && make copy-wasm-file-liquidity-gauge-reward && make test-only-liquidity-gauge-reward
test-liquidity-gauge-reward-wrapper:
	make build-contract-liquidity-gauge-reward-wrapper && make copy-wasm-file-liquidity-gauge-reward-wrapper && make test-only-liquidity-gauge-reward-wrapper
test-liquidity-gauge-wrapper:
	make build-contract-liquidity-gauge-wrapper && make copy-wasm-file-liquidity-gauge-wrapper && make test-only-liquidity-gauge-wrapper
test-minter:
	make build-contract-minter && make copy-wasm-file-minter && make test-only-minter
test-reward-only-gauge:
	make build-contract-reward-only-gauge && make copy-wasm-file-reward-only-gauge && make test-only-reward-only-gauge
test-vesting-escrow:
	make build-contract-vesting-escrow && make copy-wasm-file-vesting-escrow && make test-only-vesting-escrow
test-vesting-escrow-factory:
	make build-contract-vesting-escrow-factory && make copy-wasm-file-vesting-escrow-factory && make test-only-vesting-escrow-factory
test-vesting-escrow-simple: 
	make build-contract-vesting-escrow-simple && make copy-wasm-file-vesting-escrow-simple && make test-only-vesting-escrow-simple
test-curve-token-v3: 
	make build-contract-curve-token-v3 && make copy-wasm-file-curve-token-v3 && make test-only-curve-token-v3
test-erc20:
	make build-contract-erc20 && make copy-wasm-file-erc20 && make test-only-erc20
test-erc20-crv: 
	make build-contract-erc20-crv && make copy-wasm-file-erc20-crv && make test-only-erc20-crv
test-fee-distributor:
	make build-contract-fee-distributor && make copy-wasm-file-fee-distributor && make test-only-fee-distributor
test-gauge-controller:
	make build-contract-gauge-controller && make copy-wasm-file-gauge-controller && make test-only-gauge-controller
test-voting-escrow:
	make build-contract-voting-escrow && make copy-wasm-file-voting-escrow && make test-only-voting-escrow
test-ownable:
	make build-contract-ownable && make build-contract-ownable-test-contract && make copy-wasm-file-ownable && make test-only-ownable
test-i-reward-distribution-recipient:
	make build-i-reward-distribution-recipient && make copy-wasm-file-i-reward-distribution-recipient && make test-only-i-reward-distribution-recipient
test-lp-token-wrapper:
	make build-contract-erc20 && make build-lp-token-wrapper-session-code && make build-lp-token-wrapper && make copy-wasm-file-lp-token-wrapper && make test-only-lp-token-wrapper
test-curve-rewards:
	make build-curve-rewards-session-code && make build-curve-rewards && make copy-wasm-file-curve-rewards && make test-only-curve-rewards

test-liquidity-gauge-v3: 
	make build-contract-liquidity-gauge-v3 && make copy-wasm-file-liquidity-gauge-v3 && make test-only-liquidity-gauge-v3
all:
	make test-curve-token-v3
	make test-erc20
	make test-erc20-crv
	make test-fee-distributor
	make test-gauge-controller
	make test-gauge-proxy
	make test-liquidity-gauge-reward
	make test-liquidity-gauge-reward-wrapper
	make test-liquidity-gauge-wrapper
	make test-minter
	make test-reward-only-gauge
	make test-vesting-escrow
	make test-vesting-escrow-factory
	make test-voting-escrow
	make test-ownable
	make test-i-reward-distribution-recipient
	make test-lp-token-wrapper
	make test-curve-rewards
	make test-liquidity-gauge-v3

clean:
	cargo clean
	rm -rf Cargo.lock
	rm -rf ${curve_token_v3_des_wasm}/*.wasm
	rm -rf ${erc20_des_wasm}/*.wasm
	rm -rf ${erc20_crv_des_wasm}/*.wasm
	rm -rf ${fee_distributor_des_wasm}/*.wasm
	rm -rf ${gauge_controller_des_wasm}/*.wasm
	rm -rf ${gauge_proxy_des_wasm}/*.wasm
	rm -rf ${liquidity_gauge_reward_des_wasm}/*.wasm
	rm -rf ${liquidity_gauge_reward_wrapper_des_wasm}/*.wasm
	rm -rf ${liquidity_gauge_wrapper_des_wasm}/*.wasm
	rm -rf ${minter_des_wasm}/*.wasm
	rm -rf ${reward_only_gauge_des_wasm}/*.wasm
	rm -rf ${vesting_escrow_des_wasm}/*.wasm
	rm -rf ${vesting_escrow_factory_des_wasm}/*.wasm
	rm -rf ${voting_escrow_des_wasm}/*.wasm
	rm -rf ${ownable_des_wasm}*.wasm
	rm -rf ${i_reward_distribution_recipient_des_wasm}*.wasm
	rm -rf ${lp_token_wrapper_des_wasm}*.wasm
	rm -rf ${curve_rewards_des_wasm}*.wasm
	rm -rf ${liquidity_gauge_v3_des_wasm}*.wasm

lint: clippy
	cargo fmt --all

check-lint: clippy
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-targets --all -- -D warnings

git-clean:
	git rm -rf --cached .
	git add .