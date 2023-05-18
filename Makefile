all:
	cd rust/import_or_roll && cargo build --release --target wasm32-unknown-unknown
	cp rust/import_or_roll/target/wasm32-unknown-unknown/release/import_or_roll.wasm docs/
	wasm-gc docs/import_or_roll.wasm