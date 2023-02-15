#![no_main]

wit_bindgen::generate!("rust-side-binding");

struct RustSideImpl;
impl RustSideBinding for RustSideImpl {
    fn gcd2(a: i64, b: i64) -> i64 {
        if b == 0 {
            return a;
        } else {
            return Self::gcd2(b, a % b);
        }
    }
}

export_rust_side_binding!(RustSideImpl);
