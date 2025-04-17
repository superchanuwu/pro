use worker::*;
use js_sys::Uint8Array;
use wasm_bindgen::JsValue;
use serde::Deserialize;

// DNS over HTTPS: menerima wire-format DNS dan kirim ke 1.1.1.1/dns-query
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
    let mut res = Fetch::Request(req).send().await?;
    let abuf = res.array_buffer().await?;
    let u8buf = Uint8Array::new(&abuf);

    Ok(u8buf.to_vec())
}

#[derive(Deserialize)]
struct DoHAnswer {
    Answer: Option<Vec<DNSAnswer>>,
}

#[derive(Deserialize)]
struct DNSAnswer {
    data: String,
}

// Resolve domain string ke IP (menggunakan JSON API dari dns.google)
pub async fn resolve(domain: &str) -> Result<String> {
    let url = format!("https://dns.google/resolve?name={}&type=A", domain);
    let req = Request::new(&url, Method::Get)?;
    let mut res = Fetch::Request(req).send().await?;
    let text = res.text().await?;

    let parsed: DoHAnswer = serde_json::from_str(&text)?;
    if let Some(answers) = parsed.Answer {
        for ans in answers {
            if ans.data.parse::<std::net::IpAddr>().is_ok() {
                return Ok(ans.data);
            }
        }
    }

    // Fallback ke AAAA jika A gagal
    let url_aaaa = format!("https://dns.google/resolve?name={}&type=AAAA", domain);
    let req_aaaa = Request::new(&url_aaaa, Method::Get)?;
    let mut res_aaaa = Fetch::Request(req_aaaa).send().await?;
    let text_aaaa = res_aaaa.text().await?;
    let parsed_aaaa: DoHAnswer = serde_json::from_str(&text_aaaa)?;
    if let Some(answers) = parsed_aaaa.Answer {
        for ans in answers {
            if ans.data.parse::<std::net::IpAddr>().is_ok() {
                return Ok(ans.data);
            }
        }
    }

    Err("resolve: no valid A or AAAA record found".into())
}
