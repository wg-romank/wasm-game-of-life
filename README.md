# Convay's Game of Life in Rust/WASM/WebGL

Slightly tweaked version of original WASM tutorial. All compute is done on GPU (see `compute.frag`).
Main idea of computing state transition on GPU is inspired by common technique of rendering scene to texture.

This example uses `glsmrs` (https://github.com/wg-romank/glsmrs/) to simplify WebGL setup and make rendering to texture a bit easier.

# To build wasm

In root directory

```
wasm-pack build
```

# To bundle & start

In `www/`

```
npm i .
npm run start
```
