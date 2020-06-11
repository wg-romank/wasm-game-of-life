# Convay's Game of Life in Rust/WASM/WebGL

Slightly tweaked version of original WASM tutorial. All compute is done on GPU (see `compute.frag`).
Main idea of computing state transition on GPU is inspired by common technique of rendering scene to texture.

Texture data follows layout where `r`-byte is a marker of whether cell is alive or dead and `gba`-bytes are used to store a counter
for how much time has passed since cell died. This counter only used for visualization in colored version.

This example uses `glsmrs` (https://github.com/wg-romank/glsmrs/) to simplify WebGL setup and make rendering to texture a bit easier.

# To build wasm

```
make wasm
```

# To bundle & start

```
make serve
```

# Check it out

https://wg-romank.github.io/wasm-game-of-life/