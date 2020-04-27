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

# Tweaks
- [x] Remove redundant JS interop and do most of the logic in Rust
- [x] Efficient re-drawing for only changed cells
- [ ] WegbGL rendering instead of 2d canvas
- [ ] More interesting interactive mode (spray evens instead of single taps)
