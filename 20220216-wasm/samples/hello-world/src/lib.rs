mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet_with_alert() {
    alert("Hello, hello-world!");
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub fn greet() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No Window object found")?;
    let document = window.document().ok_or("No Document object found")?;
    let body = document.body().ok_or("document.body does not exist")?;
    let message = document.create_element("div")?;
    message.set_text_content(Some("Hello, world!"));
    body.append_child(&message)?;
    Ok(())
}
