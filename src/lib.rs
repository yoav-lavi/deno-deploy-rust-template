use wasm_bindgen::prelude::*;
use web_sys::{Request, Response};

#[wasm_bindgen]
pub async fn handler(_request: Request) -> Result<Response, JsValue> {
    console_error_panic_hook::set_once();
    Response::new_with_opt_str(Some("hello world!"))
}
