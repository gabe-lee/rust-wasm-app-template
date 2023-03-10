use wasm_bindgen::prelude::*;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let text = document.create_element("p")?;
    text.set_inner_html("This text element was dynamically generated from within Rust");

    body.append_child(&text)?;

    Ok(())
}

#[wasm_bindgen]
pub fn add_in_rust(a: u32, b: u32) -> u32 {
    a + b
}
