wasm:
	wasm-pack build

build: wasm
	cd www && npm i . && cd -

serve: build
	cd www && npm run start
