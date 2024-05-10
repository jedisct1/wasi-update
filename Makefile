bin/wasi-update: src/src/precomp/wasi_snapshot_preview1.command.wasm src/src/precomp/wasi_snapshot_preview1.reactor.wasm src/Cargo.toml src/src/main.rs
	cd src && if which cargo-zigbuild ; then cargo-zigbuild build --release $$CARGO_FLAGS ; else cargo build --release $$CARGO_FLAGS ; fi
	@ echo
	@ install -d bin
	@ find src/target -type f \( -name 'wasi-update' -o -name 'wasi-update.exe' -o -name 'wasi-update.wasm' \) -print -exec install {} bin/ \;
	-@ strip bin/wasi-update 2> /dev/null
	ls -l bin/wasi-update*
	@ echo
	- bin/wasi-update --help 2> /dev/null

src/src/precomp/wasi_snapshot_preview1.command.wasm: adapters/Cargo.toml adapters/src/descriptors.rs adapters/src/lib.rs adapters/src/macros.rs
	cd adapters && cargo build --release --no-default-features --target=wasm32-unknown-unknown --features=command
	install -m 0644 adapters/target/wasm32-unknown-unknown/release/wasi02_adapter.wasm src/src/precomp/wasi_snapshot_preview1.command.wasm

src/src/precomp/wasi_snapshot_preview1.reactor.wasm: adapters/Cargo.toml adapters/src/descriptors.rs adapters/src/lib.rs adapters/src/macros.rs
	cd adapters && cargo build --release --no-default-features --target=wasm32-unknown-unknown --features=reactor
	install -m 0644 adapters/target/wasm32-unknown-unknown/release/wasi02_adapter.wasm src/src/precomp/wasi_snapshot_preview1.reactor.wasm

clean-app:
	rm -fr src/target    
	rm -f bin/wasi-update
	touch */Cargo.toml

clean: clean-app
	rm -fr adapters/byte-array-literals/target
	rm -fr adapters/target
	rm -fr src/src/ext/wit-component/target

	
