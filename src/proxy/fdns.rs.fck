
use worker::*;
use js_sys::Uint8Array;
use wasm_bindgen::JsValue;

pub async fn doh(req_wireformat: &[u8]) -> Result<Vec<u8>> {
    let mut headers = Headers::new();
    headers.append("content-type", "application/dns-message")?;
    headers.append("accept", "application/dns-message")?;

    let body = Some(Uint8Array::from(req_wireformat).into());

    let req = Request::new_with_init_and_str(
        "https://1.1.1.1/dns-query",
        RequestInit::new()
            .with_method("POST")
            .with_headers(headers)
            .with_body(body),
    )?;

    let res = Fetch::Request(req).send().await?;
    let buf = res.bytes().await?;
    Ok(buf.to_vec())
}
