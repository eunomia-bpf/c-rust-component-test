RUST_TARGET_FILE = rust-side.wasm
C_TARGET_FILE = c-side.wasm

RUST_COMPONENT = rust-side-component.wasm
C_COMPONENT = c-side-component.wasm


DEL = rm -rf
run: build
	make -C runtime run

build: preinstall composed.wasm

composed.wasm: $(RUST_COMPONENT) $(C_COMPONENT)
	wasm-tools compose $(C_COMPONENT) -c compose.yml -o $@

$(RUST_TARGET_FILE):
	make -C rust-side build
	cp ./target/wasm32-unknown-unknown/release/$(RUST_TARGET_FILE) .

$(C_TARGET_FILE):
	make -C c-side build
	cp ./c-side/$(C_TARGET_FILE) .

$(RUST_COMPONENT): $(RUST_TARGET_FILE)
	wasm-tools component new $< -o $@

$(C_COMPONENT): $(C_TARGET_FILE)
	wasm-tools component new $< -o $@
	

preinstall:
	cargo install wasm-tools --git https://github.com/bytecodealliance/wasm-tools --rev c4c9125
	cargo install wit-bindgen-cli --git https://github.com/bytecodealliance/wit-bindgen --rev 0fb4018


clean:
	make -C rust-side clean
	make -C c-side clean
	$(DEL) $(RUST_TARGET_FILE)
	$(DEL) $(C_TARGET_FILE)
	$(DEL) $(RUST_COMPONENT)
	$(DEL) $(C_COMPONENT)
	$(DEL) composed.wasm
