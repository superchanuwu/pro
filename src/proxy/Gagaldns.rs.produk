use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use reqwest::Client;
use tokio::time::{timeout, Duration};

/// DNS over HTTPS with fallback to Google + UDP
pub async fn doh(req_wireformat: &[u8]) -> Result<Vec<u8>> {
    let client = Client::new();

    let endpoints = [
        "https://1.1.1.1/dns-query",         // Cloudflare
        "https://dns.google/dns-query",     // Google
    ];

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/dns-message"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/dns-message"));

    for endpoint in endpoints.iter() {
        if let Ok(res) = timeout(
            Duration::from_secs(3),
            client
                .post(*endpoint)
                .headers(headers.clone())
                .body(req_wireformat.to_vec())
                .send()
        ).await {
            if let Ok(body) = res?.bytes().await {
                return Ok(body.to_vec());
            }
        }
    }

    // Fallback ke UDP DNS Google (8.8.8.8)
    resolve_via_udp(req_wireformat)
}

fn resolve_via_udp(request: &[u8]) -> Result<Vec<u8>> {
    use std::net::UdpSocket;
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_read_timeout(Some(Duration::from_secs(2)))?;
    socket.connect("8.8.8.8:53")?;
    socket.send(request)?;

    let mut buf = [0u8; 512];
    let n = socket.recv(&mut buf)?;
    Ok(buf[..n].to_vec())
}
