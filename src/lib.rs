use js_sys::Promise;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

// /// Foreign function interface for JavaScript
// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);

//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_u32(a: u32);

//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_many(a: &str, b: &str);
// }

#[wasm_bindgen]
pub async fn ws_ping(endpoint: String, message: String) -> Result<String, JsValue> {
    // Create WebSocket instance and wrap it in Rc
    let ws = Rc::new(WebSocket::new(&endpoint)?);

    // Create a promise and its resolvers
    // TODO: Handle unwraps
    let (promise, resolve, reject) = {
        let mut resolve = None;
        let mut reject = None;
        let promise = Promise::new(&mut |res, rej| {
            resolve = Some(res);
            reject = Some(rej);
        });
        (promise, resolve.unwrap(), reject.unwrap())
    };

    // RC to share between calls.
    let resolve = Rc::new(RefCell::new(Some(resolve)));
    let reject = Rc::new(RefCell::new(Some(reject)));

    // Set up message handler
    {
        let resolve = Rc::clone(&resolve);
        let callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Some(resolve) = resolve.borrow_mut().take() {
                let data = e.data().as_string().unwrap_or_default();
                resolve
                    .call1(&JsValue::NULL, &JsValue::from_str(&data))
                    .unwrap();
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        ws.set_onmessage(Some(callback.as_ref().unchecked_ref()));
        // Prevent memory leak
        // TODO: There should be a better way to handle this.
        callback.forget();
    }

    // Set up error handler
    {
        let reject = Rc::clone(&reject);
        let callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
            if let Some(reject) = reject.borrow_mut().take() {
                reject.call1(&JsValue::NULL, &e.error()).unwrap();
            }
        }) as Box<dyn FnMut(ErrorEvent)>);
        ws.set_onerror(Some(callback.as_ref().unchecked_ref()));
        // Prevent memory leak
        // TODO: There should be a better way to handle this.
        callback.forget();
    }

    // Send the message once the connection is open
    {
        let ws_clone = Rc::clone(&ws);
        let mut message = message.clone();
        message.push_str("oh Yah baby! from WASM Rust");

        let onopen_callback = Closure::wrap(Box::new(move |_| {
            let _ = ws_clone.send_with_str(&message);
        }) as Box<dyn FnMut(JsValue)>);

        let ws_for_set = Rc::clone(&ws);
        ws_for_set.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
    }

    // Wait for the response
    let result = JsFuture::from(promise).await?;
    Ok(result.as_string().unwrap_or_default())
}
