
play: build run

run:
	./out/gol

run-wasm: build-wasm
	wasmer ./target/wasm32-wasi/debug/gol.wasm


build-mac:
	rm -rf ./out
	mkdir out
	rustc -o ./out/gol ./src/main.rs

build-wasm:
	cargo build --quiet --target wasm32-wasi