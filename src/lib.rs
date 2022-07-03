mod utils;

use js_sys::JSON;
use utils::to_body;
use wasm_bindgen::prelude::*;
use web_sys::{Request, Response};

#[wasm_bindgen]
pub async fn handler(request: Request) -> Result<Response, JsValue> {
    let request_body = to_body(request).await?;

    let body_string = &JSON::stringify(&request_body)?;

    Response::new_with_opt_str(body_string.as_string().as_deref())
}
