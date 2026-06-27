use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::proxy::PROXY_HOST;

pub fn env_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config/tz/env.sh")
}

pub fn set(port: u16) -> Result<()> {
    let path = env_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).context("创建配置目录失败")?;
    }

    let content = format!(
        r#"# 由 tz 自动生成，退出 tz 后会被 shell 自动 source
export http_proxy="http://{PROXY_HOST}:{port}"
export https_proxy="http://{PROXY_HOST}:{port}"
export HTTP_PROXY="http://{PROXY_HOST}:{port}"
export HTTPS_PROXY="http://{PROXY_HOST}:{port}"
export all_proxy="socks5://{PROXY_HOST}:{port}"
export ALL_PROXY="socks5://{PROXY_HOST}:{port}"
export no_proxy="127.0.0.1,localhost,::1"
export NO_PROXY="127.0.0.1,localhost,::1"
"#
    );

    fs::write(&path, content).context("写入终端代理配置失败")?;
    Ok(())
}

pub fn clear() -> Result<()> {
    let path = env_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).context("创建配置目录失败")?;
    }

    let content = r#"# 由 tz 自动生成 — 代理已关闭
unset http_proxy https_proxy HTTP_PROXY HTTPS_PROXY all_proxy ALL_PROXY
"#;

    fs::write(&path, content).context("写入终端代理配置失败")?;
    Ok(())
}

pub fn active_port() -> Option<u16> {
    let content = fs::read_to_string(env_path()).ok()?;
    for line in content.lines() {
        if line.starts_with("export http_proxy=") {
            let url = line.trim_start_matches("export http_proxy=").trim_matches('"');
            return url.rsplit(':').next()?.parse().ok();
        }
    }
    None
}
