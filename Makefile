build:
	wasm-pack build
	cd www && npm i . && cd -

serve: build
	cd www && npm run start
