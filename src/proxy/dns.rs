use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use reqwest::Client;
use std::net::UdpSocket;
use std::time::Duration;

/// DNS over HTTPS with fallback to Google + UDP
pub async fn doh(req_wireformat: &[u8]) -> Result<Vec<u8>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(3))
        .build()?;

    let endpoints = [
        "https://1.1.1.1/dns-query",       // Cloudflare
        "https://dns.google/dns-query",   // Google
    ];

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/dns-message"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/dns-message"));

    for endpoint in endpoints.iter() {
        let resp = client
            .post(*endpoint)
            .headers(headers.clone())
            .body(req_wireformat.to_vec())
            .send()
            .await;

        if let Ok(res) = resp {
            if let Ok(body) = res.bytes().await {
                return Ok(body.to_vec());
            }
        }
    }

    // Fallback ke UDP DNS Google (8.8.8.8)
    resolve_via_udp(req_wireformat)
}

/// Fallback resolver via UDP to 8.8.8.8:53
fn resolve_via_udp(request: &[u8]) -> Result<Vec<u8>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_read_timeout(Some(Duration::from_secs(2)))?;
    socket.send_to(request, "8.8.8.8:53")?;

    let mut buf = [0u8; 512];
    let (n, _) = socket.recv_from(&mut buf)?;
    Ok(buf[..n].to_vec())
}
