src_target = target/wasm32-unknown-unknown/release
# liquid_locker_des_wasm = liquid-locker/liquid-locker-tests/wasm
# liquid_helper_des_wasm = liquid-helper/liquid-helper-tests/wasm
child_streamer_des_wasm = child-streamer/child-streamer-tests/wasm

prepare:
	rustup target add wasm32-unknown-unknown

# build-contract-liquid-helper:
# 	cargo build --release -p liquid-helper -p liquid-helper-proxy --target wasm32-unknown-unknown
# build-contract-liquid-locker:
# 	cargo build --release -p liquid-locker -p liquid-locker-proxy -p erc20 --target wasm32-unknown-unknown

build-contract-child-streamer:
	cargo build --release -p child-streamer -p child-streamer-proxy --target wasm32-unknown-unknown

# test-only-liquid-helper:
# 	cargo test -p liquid-helper-tests
# test-only-liquid-locker:
# 	cargo test -p liquid-locker-tests
test-only-child-streamer:
	cargo test -p child-streamer-tests

# copy-wasm-file-liquid-helper:
# 	cp ${src_target}/liquid-helper.wasm ${liquid_helper_des_wasm}
# 	cp ${src_target}/liquid-helper-proxy.wasm ${liquid_helper_des_wasm}
# copy-wasm-file-liquid-locker:
# 	cp ${src_target}/liquid-locker.wasm ${liquid_locker_des_wasm}
# 	cp ${src_target}/liquid-locker-proxy.wasm ${liquid_locker_des_wasm}
# 	cp ${src_target}/erc20-token.wasm ${liquid_locker_des_wasm}

copy-wasm-file-chile-streamer:
	cp ${src_target}/child-streamer-token.wasm ${child_streamer_des_wasm}
	cp ${src_target}/child-streamer-proxy-token.wasm ${child_streamer_des_wasm}

# test-liquid-helper:
# 	make build-contract-liquid-helper && make copy-wasm-file-liquid-helper && make test-only-liquid-helper
# test-liquid-locker:
# 	make build-contract-liquid-locker && make copy-wasm-file-liquid-locker && make test-only-liquid-locker
test-child-streamer:
	make build-contract-child-streamer && make copy-wasm-file-chile-streamer

all:
	# make test-liquid-helper && make test-liquid-locker
	make test-child-streamer && make test-only-child-streamer

clean:
	cargo clean
	rm -rf liquid-helper/liquid-helper-tests/wasm/*.wasm
	rm -rf liquid-locker/liquid-locker-tests/wasm/*.wasm
	rm -rf Cargo.lock