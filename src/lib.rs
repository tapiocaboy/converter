use js_sys::Promise;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

/// Setup error handler
/// # Arguments
/// * `ws` - The WebSocket instance
/// * `reject_args` - The reject arguments
/// # Example
/// ```rust
/// let reject_args = Rc::new(RefCell::new(None));
/// setup_error_handler(&ws, reject_args);
/// ```
fn setup_error_handler(ws: &WebSocket, reject_args: Rc<RefCell<Option<js_sys::Function>>>) {
    let reject_clone = Rc::clone(&reject_args);
    let error_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        if let Some(reject) = reject_clone.borrow_mut().take() {
            reject.call1(&JsValue::NULL, &e.error()).unwrap();
        }
    }) as Box<dyn FnMut(ErrorEvent)>);
    
    ws.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
    error_callback.forget();
}

/// Setup message sender
/// # Arguments
/// * `ws` - The WebSocket instance
/// # Example
/// ```rust
/// setup_message_sender(&ws, message);
/// ```
fn setup_message_sender(ws: &WebSocket, message: String) {
    let ws_clone = Rc::clone(&Rc::new(ws.clone()));
    let mut message = message.clone();
    message.push_str(" oh Yah baby! from WASM Rust");

    let onopen_callback = Closure::wrap(Box::new(move |_| {
        let _ = ws_clone.send_with_str(&message);
    }) as Box<dyn FnMut(JsValue)>);

    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();
}

/// Setup message handler
/// # Arguments
/// * `ws` - The WebSocket instance
/// * `resolve_args` - The resolve arguments
/// # Example
/// ```rust
/// setup_message_handler(&ws, resolve);
/// ```
fn setup_message_handler(ws: &WebSocket, resolve_args: Rc<RefCell<Option<js_sys::Function>>>) {
    let resolve = Rc::clone(&resolve_args);
    let callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        if let Some(resolve) = resolve.borrow_mut().take() {
            let data = e.data().as_string().unwrap_or_default();
            resolve
                .call1(&JsValue::NULL, &JsValue::from_str(&data))
                .unwrap();
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    
    ws.set_onmessage(Some(callback.as_ref().unchecked_ref()));
    callback.forget();
}

#[wasm_bindgen]
pub async fn ws_ping(endpoint: String, message: String) -> Result<String, JsValue> {
    let ws = Rc::new(WebSocket::new(&endpoint)?);

    let (promise, resolve, reject) = {
        let mut resolve = None;
        let mut reject = None;
        let promise = Promise::new(&mut |res, rej| {
            resolve = Some(res);
            reject = Some(rej);
        });
        (promise, resolve.unwrap(), reject.unwrap())
    };

    let resolve = Rc::new(RefCell::new(Some(resolve)));
    let reject = Rc::new(RefCell::new(Some(reject)));

    setup_message_handler(&ws, Rc::clone(&resolve));
    setup_error_handler(&ws, Rc::clone(&reject));
    setup_message_sender(&ws, message);

    let result = JsFuture::from(promise).await?;
    Ok(result.as_string().unwrap_or_default())
}
