bin/wasi-update: .ts wasi-update/src/precomp/wasi_snapshot_preview1.command.wasm wasi-update/src/precomp/wasi_snapshot_preview1.reactor.wasm wasi-update/Cargo.toml wasi-update/src/main.rs
	cd wasi-update && cargo build --release $$CARGO_FLAGS
	install -d bin
	install -s wasi-update/target/release/wasi-update bin/wasi-update
	ls -l bin/wasi-update
	bin/wasi-update --help

wasi-update/src/precomp/wasi_snapshot_preview1.command.wasm: adapters/Cargo.toml adapters/src/descriptors.rs adapters/src/lib.rs adapters/src/macros.rs
	cd adapters && cargo build --release --no-default-features --target=wasm32-unknown-unknown --features=command
	install -m 0644 adapters/target/wasm32-unknown-unknown/release/wasi02_adapter.wasm wasi-update/src/precomp/wasi_snapshot_preview1.command.wasm

wasi-update/src/precomp/wasi_snapshot_preview1.reactor.wasm: adapters/Cargo.toml adapters/src/descriptors.rs adapters/src/lib.rs adapters/src/macros.rs
	cd adapters && cargo build --release --no-default-features --target=wasm32-unknown-unknown --features=reactor
	install -m 0644 adapters/target/wasm32-unknown-unknown/release/wasi02_adapter.wasm wasi-update/src/precomp/wasi_snapshot_preview1.reactor.wasm

clean:
	rm -fr adapters/byte-array-literals/target
	rm -fr adapters/target
	rm -fr wasi-update/target
	rm -f bin/wasi-update
	touch .ts

	