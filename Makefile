wasm:
	wasm-pack build

build: wasm
	cd www && npm i . && cd -

serve: build
	cd www && npm run start

package:
	rm -rf docs
	cd www && npm run build
