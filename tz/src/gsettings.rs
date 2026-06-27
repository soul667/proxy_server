use std::process::Command;

use anyhow::{bail, Context, Result};

use crate::proxy::PROXY_HOST;

const SCHEMA: &str = "org.gnome.system.proxy";

pub fn get(key: &str) -> Result<String> {
    get_nested("", key)
}

pub fn get_nested(section: &str, key: &str) -> Result<String> {
    let schema = if section.is_empty() {
        SCHEMA.to_string()
    } else {
        format!("{SCHEMA}.{section}")
    };
    let output = Command::new("gsettings")
        .args(["get", &schema, key])
        .output()
        .context("执行 gsettings 失败，需要 GNOME 桌面环境")?;
    if !output.status.success() {
        bail!("gsettings get 失败");
    }
    let value = String::from_utf8(output.stdout)?.trim().to_string();
    Ok(trim_gsettings(value))
}

fn trim_gsettings(value: String) -> String {
    if value.starts_with('\'') && value.ends_with('\'') {
        value[1..value.len() - 1].to_string()
    } else {
        value
    }
}

pub fn set_proxy(port: u16) -> Result<()> {
    run(&["set", SCHEMA, "mode", "manual"])?;
    for section in ["http", "https", "socks"] {
        let schema = format!("{SCHEMA}.{section}");
        run(&["set", &schema, "host", PROXY_HOST])?;
        run(&["set", &schema, "port", &port.to_string()])?;
    }
    run(&[
        "set",
        SCHEMA,
        "ignore-hosts",
        "['localhost', '127.0.0.0/8', '::1']",
    ])?;
    Ok(())
}

pub fn turn_off() -> Result<()> {
    run(&["set", SCHEMA, "mode", "none"])?;
    Ok(())
}

fn run(args: &[&str]) -> Result<()> {
    let status = Command::new("gsettings").args(args).status()?;
    if status.success() {
        Ok(())
    } else {
        bail!("gsettings 命令失败: {}", args.join(" "))
    }
}
