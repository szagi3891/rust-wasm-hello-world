mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alertConsole(s: &str);
    fn callFromWasm(message: &str);
}

#[wasm_bindgen]
//pub fn greet(message: &'static str) {
pub fn greet(message: &str, mes: &str) -> String {
    callFromWasm("Zaczynam alertowanie");
    alertConsole(&format!("pppp3 - Hello, {} == {}", message, mes));
    callFromWasm("Kończę alertowanie");

    "aaaaa kkklops".into()
}