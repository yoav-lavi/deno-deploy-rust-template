use {{crate_name}}::handler;
use wasm_bindgen_test::*;
use web_sys::{Request, RequestInit};

#[wasm_bindgen_test]
async fn deno_deploy_test() {
    let mut request_init = RequestInit::new();

    request_init.method("POST");

    let request = Request::new_with_str_and_init("http://localhost:8000", &request_init).unwrap();

    let response = handler(request).await.unwrap();

    let response_body = wasm_bindgen_futures::JsFuture::from(response.text().unwrap())
        .await
        .unwrap()
        .as_string();

    assert_eq!(response_body, Some("hello world!".to_owned()));
}
