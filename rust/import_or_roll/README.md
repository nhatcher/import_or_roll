# Building

Compile with

```bash
$ cargo build --release --target wasm32-unknown-unknown
```

The output will be in `target/wasm32-unknown-unknown/release/import_or_roll.wasm`.

If you have `wasm-gc` installed you can reduce the size of the wasm file by

```bash
$ wasm-gc import_or_roll.wasm
```
