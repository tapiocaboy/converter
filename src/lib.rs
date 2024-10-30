use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen(start)]
fn run() {
    bare_bones();
    using_a_macro();
    using_web_sys();
}


/// Foreign function interface for JavaScript
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

fn bare_bones() {
    log("Hello world!");
    log_u32(42);
    log_many("Logging", "many values!");
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn using_a_macro() {
    console_log!("Hello {}!","world");
    console_log!("Logging {} values", 2);
    console_log!("1 + 4 = {}", 1 + 4);
}

fn using_web_sys() {
    use web_sys::console;
    console::log_1(&"Hello using web-sys".into());
    console::log_2(&"Logging using web-sys".into(), &2.into());
    console::log_3(&"Logging using web-sys".into(), &"multiple".into(), &"values".into());
    let js: JsValue = 4.into();
    console::log_2(&"Logging arbitrary values looks like".into(), &js);
}