# Demonstration on linking components written in different languages together

Run `make run` and see the output.

- `c-side` is a C program, it uses `wit-bindgen` to generate bindings for `c-side/c-side-binding.wit`. The wit file describes an import for interface `gcd2-outer`, which contains an function `gcd2` used to calculate the greatest common divisor of two integers. It exports a function `gcd4`, which uses the imported `gcd2` function to calculate the greatest common divisor of four integers.

- `rust-side` is a Rust program. It implements and exports the `gcd2` function, with bindings generated by `wit-bindgen`

- `c-side` and `rust-side` will be compiled individually, and composed together through `wasmtools-compose` (See `compose.yml` and `Makefile` for details)

- `runtime` is a Rust program invoking `wasmtime` to run the composed module.


Notes:
- Imported things must be interface. Only import to interfaces will be compiled to instance import, which could be recognized by `wasm-tools compose`