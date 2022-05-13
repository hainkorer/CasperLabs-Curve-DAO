for file in target/wasm32-unknown-unknown/release/*.wasm
	do
  	wasm-strip "$file"
	done