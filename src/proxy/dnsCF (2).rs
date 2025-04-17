use worker::*;

pub async fn doh(req_wireformat: &[u8]) -> Result<Vec<u8>> {
    let mut headers = Headers::new();
    headers.set("Content-Type", "application/dns-message")?;
    headers.set("Accept", "application/dns-message")?;

    let mut init = RequestInit::new();
    init.with_method(Method::Post);
    init.with_body(Some(req_wireformat.into()));
    init.with_headers(headers);

    let req = Request::new_with_init("https://1.1.1.1/dns-query", &init)?;
    let resp = Fetch::Request(req).send().await?;
    let bytes = resp.bytes().await?;

    Ok(bytes.to_vec())
}