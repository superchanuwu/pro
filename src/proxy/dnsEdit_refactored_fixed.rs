
use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use reqwest::Client;
use std::net::UdpSocket;
use std::time::Duration;

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

    for endpoint in endpoints {
        match client
            .post(endpoint)
            .headers(headers.clone())
            .body(req_wireformat.to_vec())
            .send()
            .await
        {
            Ok(res) => match res.bytes().await {
                Ok(body) => return Ok(body.to_vec()),
                Err(e) => eprintln!("Failed to read response from {}: {}", endpoint, e),
            },
            Err(e) => eprintln!("Request to {} failed: {}", endpoint, e),
        }
    }

    eprintln!("All DoH endpoints failed, falling back to UDP...");
    resolve_via_udp(req_wireformat)
}

fn resolve_via_udp(request: &[u8]) -> Result<Vec<u8>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("8.8.8.8:53")?;
    socket.send(request)?;

    let mut buf = [0u8; 512];
    let len = socket.recv(&mut buf)?;
    Ok(buf[..len].to_vec())
}
