use std::time::Instant;

use anyhow::Context;
use reqwest::Proxy;

use crate::proxy::{ProxyDef, PROXY_HOST};

pub struct SpeedResult {
    pub proxy: ProxyDef,
    pub ip: String,
    pub latency_ms: Option<u128>,
    pub status: String,
}

const TEST_URL: &str = "http://ifconfig.me/ip";

pub async fn test_proxy(proxy: &'static ProxyDef) -> SpeedResult {
    let proxy_url = format!("http://{PROXY_HOST}:{}", proxy.port);
    let client = match Proxy::http(&proxy_url)
        .context("构建代理")
        .and_then(|p| {
            reqwest::Client::builder()
                .proxy(p)
                .timeout(std::time::Duration::from_secs(12))
                .build()
                .context("构建 HTTP 客户端")
        }) {
        Ok(c) => c,
        Err(e) => {
            return SpeedResult {
                proxy: *proxy,
                ip: "-".into(),
                latency_ms: None,
                status: truncate_err(&e.to_string()),
            };
        }
    };

    let start = Instant::now();
    match client.get(TEST_URL).send().await {
        Ok(resp) => {
            let latency = start.elapsed().as_millis();
            match resp.text().await {
                Ok(body) => SpeedResult {
                    proxy: *proxy,
                    ip: body.trim().to_string(),
                    latency_ms: Some(latency),
                    status: "OK".into(),
                },
                Err(e) => SpeedResult {
                    proxy: *proxy,
                    ip: "-".into(),
                    latency_ms: Some(latency),
                    status: truncate_err(&e.to_string()),
                },
            }
        }
        Err(e) => SpeedResult {
            proxy: *proxy,
            ip: "-".into(),
            latency_ms: None,
            status: truncate_err(&e.to_string()),
        },
    }
}

fn truncate_err(msg: &str) -> String {
    if msg.chars().count() > 28 {
        format!("{}…", msg.chars().take(27).collect::<String>())
    } else {
        msg.to_string()
    }
}
