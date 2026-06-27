use crate::{envfile, gsettings};

pub struct ApplyResult {
    pub terminal: bool,
    pub system: bool,
}

pub fn apply(port: u16) -> ApplyResult {
    let terminal = envfile::set(port).is_ok();
    let system = gsettings::set_proxy(port).is_ok();
    ApplyResult { terminal, system }
}

pub fn clear() -> ApplyResult {
    let terminal = envfile::clear().is_ok();
    let system = gsettings::turn_off().is_ok();
    ApplyResult { terminal, system }
}

pub fn format_apply_status(base: &str, result: &ApplyResult) -> String {
    let scope = match (result.terminal, result.system) {
        (true, true) => "系统+终端",
        (true, false) => "终端",
        (false, true) => "系统",
        (false, false) => "失败",
    };
    format!("{base} · {scope}")
}
