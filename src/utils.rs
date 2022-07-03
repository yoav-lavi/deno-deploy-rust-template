use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::Request;

pub async fn to_body(request: Request) -> Result<JsValue, JsValue> {
    Ok(JsFuture::from(request.json()?).await?)
}
