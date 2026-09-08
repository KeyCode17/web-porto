use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

pub async fn sleep_ms(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        let scheduled = web_sys::window().and_then(|window| {
            window
                .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms)
                .ok()
        });
        if scheduled.is_none() {
            let _ = resolve.call1(&JsValue::NULL, &JsValue::NULL);
        }
    });
    let _ = JsFuture::from(promise).await;
}
