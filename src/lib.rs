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
/// setup_error_handler(&ws, reject);
/// ```
fn setup_error_handler(ws: &WebSocket, reject_args: Rc<RefCell<Option<js_sys::Function>>>) -> Result<(), JsValue> {
    let reject_clone = Rc::clone(&reject_args);
    let error_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        if let Some(reject) = reject_clone.borrow_mut().take() {
            let _ = reject.call1(&JsValue::NULL, &e.error());
        }
    }) as Box<dyn FnMut(ErrorEvent)>);
    
    ws.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
    error_callback.forget();
    Ok(())
}
/// Setup message sender
/// # Arguments
/// * `ws` - The WebSocket instance
/// * `message` - The message to send
/// # Example
/// ```rust
/// setup_message_sender(&ws, message);
/// ```
fn setup_message_sender(ws: &WebSocket, message: String) -> Result<(), JsValue> {
    let ws_clone = Rc::clone(&Rc::new(ws.clone()));
    let mut message = message.clone();
    message.push_str(" oh Yah baby! from WASM Rust");

    let onopen_callback = Closure::wrap(Box::new(move |_| {
        if let Err(e) = ws_clone.send_with_str(&message) {
            web_sys::console::error_1(&e);
        }
    }) as Box<dyn FnMut(JsValue)>);

    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();
    Ok(())
}

/// Setup message handler
/// # Arguments
/// * `ws` - The WebSocket instance
/// * `resolve_args` - The resolve arguments
/// # Example
/// ```rust
/// setup_message_handler(&ws, resolve);
/// ```
fn setup_message_handler(ws: &WebSocket, resolve_args: Rc<RefCell<Option<js_sys::Function>>>) -> Result<(), JsValue> {
    let resolve = Rc::clone(&resolve_args);
    let callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        if let Some(resolve) = resolve.borrow_mut().take() {
            let data = e.data().as_string().unwrap_or_default();
            if let Err(e) = resolve.call1(&JsValue::NULL, &JsValue::from_str(&data)) {
                web_sys::console::error_1(&e);
            }
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    
    ws.set_onmessage(Some(callback.as_ref().unchecked_ref()));
    callback.forget();
    Ok(())
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
        match (resolve, reject) {
            (Some(res), Some(rej)) => (promise, res, rej),
            _ => return Err(JsValue::from_str("Error in Server")),
        }
    };

    let resolve = Rc::new(RefCell::new(Some(resolve)));
    let reject = Rc::new(RefCell::new(Some(reject)));

    setup_message_handler(&ws, Rc::clone(&resolve))?;
    setup_error_handler(&ws, Rc::clone(&reject))?;
    setup_message_sender(&ws, message)?;

    let result = JsFuture::from(promise).await?;
    Ok(result.as_string().unwrap_or_default())
}
