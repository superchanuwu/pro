use worker::*;
use js_sys::Uint8Array;
use wasm_bindgen::JsValue;

pub async fn doh(req_wireformat: &[u8]) -> Result<Vec<u8>> {
    let mut headers = Headers::new();
    headers.append("content-type", "application/dns-message")?;
    headers.append("accept", "application/dns-message")?;

    let body: JsValue = Uint8Array::from(req_wireformat).into();

    let mut init = RequestInit::new();
    init.with_method(Method::Post);
    init.with_headers(headers);
    init.with_body(Some(body));

    let req = Request::new_with_init("https://1.1.1.1/dns-query", &init)?;
    let res = Fetch::Request(req).send().await?;
    let abuf = res.array_buffer().await?;
    let u8buf = Uint8Array::new(&abuf);

    Ok(u8buf.to_vec())
}
