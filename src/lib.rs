use wasm_bindgen::prelude::*;
use web_sys::{Request, Response};

#[wasm_bindgen]
pub async fn handler(_request: Request) -> Result<Response, JsValue> {
    Response::new_with_opt_str(Some("hello world!"))
}
