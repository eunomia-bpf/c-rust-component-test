build: gen-binding /opt/wasi-sdk
	/opt/wasi-sdk/bin/clang c_side_binding.c c_side_binding_component_type.o impl.c -o c-side.wasm -mexec-model=reactor

gen-binding:
	wit-bindgen c c-side-binding.wit


DEL = rm -rf

clean:
	$(DEL) *.o
	$(DEL) c_side_binding.c
	$(DEL) c_side_binding.h
	$(DEL) c-side.wasm