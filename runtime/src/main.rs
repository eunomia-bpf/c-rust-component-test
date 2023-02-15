use anyhow::{anyhow, Context};
use rand::Rng;
use wasmtime::{
    component::{bindgen, Component, Linker, Val},
    Store,
};
bindgen!("runtime-binding");
struct MyState {}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
fn gcd_seq(seq: &[i64]) -> i64 {
    let mut t = 0;
    for s in seq {
        t = gcd(t, *s);
    }
    t
}
fn main() -> anyhow::Result<()> {
    let mut config = wasmtime::Config::new();
    config.wasm_component_model(true);
    let engine =
        wasmtime::Engine::new(&config).with_context(|| anyhow!("Failed to create engine"))?;

    let composed_component = Component::from_file(&engine, "../composed.wasm")
        .with_context(|| anyhow!("Failed to read component"))?;
    let mut store = Store::new(&engine, MyState {});
    let linker = Linker::new(&engine);


    let (binding, _) = Runtime::instantiate(&mut store, &composed_component, &linker)
        .with_context(|| anyhow!("Failed to instantiate runtime bindings"))?;
    let mut result = [Val::S64(0)];

    let mut rng = rand::thread_rng();
    let mut randnum = || rng.gen_range(1i64..=10);
    let a = randnum() * 2 * 3;
    let b = randnum() * 3 * 5;
    let c = randnum() * 2 * 3 * 5 * 7;
    let d = randnum() * 2 * 3 * 5 * 7 * 11;
    let array = [a, b, c, d];
    let expected_result = gcd_seq(&array);
    println!("Array: {:?}, Expected result: {}", array, expected_result);
    binding
        .gcd4
        .call(
            &mut store,
            &[Val::S64(a), Val::S64(b), Val::S64(c), Val::S64(d)],
            &mut result,
        )
        .with_context(|| anyhow!("Failed to call gcd4"))?;
    println!("Received result: {:?}", result[0]);

    return Ok(());
}
