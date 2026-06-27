mod apply;
mod envfile;
mod gsettings;
mod proxy;
mod speedtest;
mod theme;
mod ui;

use std::time::Duration;

use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;

use apply::{apply, clear, format_apply_status};
use proxy::{ProxyDef, PROXIES};
use speedtest::SpeedResult;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Main,
    SpeedTest,
}

#[derive(Clone)]
enum MenuAction {
    SetProxy(&'static ProxyDef),
    ShowCurrent,
    RunSpeedTest,
    TurnOff,
    Quit,
}

struct MenuEntry {
    label: String,
    action: MenuAction,
}

pub struct App {
    pub screen: Screen,
    pub menu_items: Vec<MenuEntry>,
    pub list_state: ratatui::widgets::ListState,
    pub status: String,
    pub speed_results: Vec<SpeedResult>,
    pub speed_testing: bool,
    pub speed_done: usize,
}

impl App {
    fn new() -> Self {
        let mut menu_items = PROXIES
            .iter()
            .map(|p| MenuEntry {
                label: format!("  {}  {}  :{}", p.flag, p.name, p.port),
                action: MenuAction::SetProxy(p),
            })
            .collect::<Vec<_>>();

        menu_items.push(MenuEntry {
            label: "────────────────".into(),
            action: MenuAction::ShowCurrent,
        });
        menu_items.push(MenuEntry {
            label: "  查看当前代理".into(),
            action: MenuAction::ShowCurrent,
        });
        menu_items.push(MenuEntry {
            label: "  测速对比".into(),
            action: MenuAction::RunSpeedTest,
        });
        menu_items.push(MenuEntry {
            label: "  关闭代理".into(),
            action: MenuAction::TurnOff,
        });
        menu_items.push(MenuEntry {
            label: "  退出".into(),
            action: MenuAction::Quit,
        });

        let mut list_state = ratatui::widgets::ListState::default();
        list_state.select(Some(0));

        Self {
            screen: Screen::Main,
            menu_items,
            list_state,
            status: current_proxy_summary().unwrap_or_else(|e| format!("读取失败: {e}")),
            speed_results: Vec::new(),
            speed_testing: false,
            speed_done: 0,
        }
    }

    fn selectable_indices(&self) -> Vec<usize> {
        self.menu_items
            .iter()
            .enumerate()
            .filter(|(_, item)| !item.label.starts_with('─'))
            .map(|(i, _)| i)
            .collect()
    }

    fn move_selection(&mut self, delta: i32) {
        let indices = self.selectable_indices();
        if indices.is_empty() {
            return;
        }
        let current = self.list_state.selected().unwrap_or(indices[0]);
        let pos = indices.iter().position(|&i| i == current).unwrap_or(0);
        let next = (pos as i32 + delta).rem_euclid(indices.len() as i32) as usize;
        self.list_state.select(Some(indices[next]));
    }

    fn selected_action(&self) -> Option<&MenuAction> {
        let idx = self.list_state.selected()?;
        let entry = self.menu_items.get(idx)?;
        if entry.label.starts_with('─') {
            return None;
        }
        Some(&entry.action)
    }
}

fn current_proxy_summary() -> Result<String> {
    let mode = gsettings::get("mode").unwrap_or_else(|_| "none".into());
    let env_port = envfile::active_port();

    if mode == "none" && env_port.is_none() {
        return Ok("当前: 未开启".into());
    }

    let port = env_port
        .or_else(|| gsettings::get_nested("http", "port").ok()?.parse().ok())
        .unwrap_or(0);
    let host = gsettings::get_nested("http", "host").unwrap_or_else(|_| "127.0.0.1".into());
    let label = PROXIES
        .iter()
        .find(|p| p.port == port)
        .map(|p| format!("{} {}", p.flag, p.name))
        .unwrap_or_else(|| "自定义".into());

    let terminal = env_port.is_some();
    let system = mode != "none";
    let scope = match (terminal, system) {
        (true, true) => "系统+终端",
        (true, false) => "终端",
        (false, true) => "系统",
        _ => "",
    };

    Ok(format!("当前: {label} ({host}:{port}) · {scope}"))
}

async fn run_speed_test(app: &mut App, terminal: &mut DefaultTerminal) -> Result<()> {
    app.speed_results.clear();
    app.speed_testing = true;
    app.speed_done = 0;

    for proxy in PROXIES {
        app.speed_done += 1;
        terminal.draw(|frame| ui::draw(frame, app))?;
        let result = speedtest::test_proxy(proxy).await;
        app.speed_results.push(result);
        terminal.draw(|frame| ui::draw(frame, app))?;
    }

    app.speed_testing = false;
    Ok(())
}

fn should_handle_key(key: &KeyEvent) -> bool {
    match key.kind {
        KeyEventKind::Press | KeyEventKind::Repeat => true,
        KeyEventKind::Release => is_submit(key),
        _ => false,
    }
}

fn is_submit(key: &KeyEvent) -> bool {
    matches!(
        key.code,
        KeyCode::Enter | KeyCode::Char('\r') | KeyCode::Char('\n') | KeyCode::Char(' ')
    )
}

fn is_quit(key: &KeyEvent) -> bool {
    matches!(key.code, KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q'))
}

async fn handle_main_action(
    app: &mut App,
    action: MenuAction,
    terminal: &mut DefaultTerminal,
) -> Result<bool> {
    match action {
        MenuAction::SetProxy(p) => {
            let result = apply(p.port);
            if result.terminal || result.system {
                app.status = current_proxy_summary().unwrap_or_else(|_| {
                    format_apply_status(
                        &format!("当前: {} {} (:{}", p.flag, p.name, p.port),
                        &result,
                    )
                });
            } else {
                app.status = "设置失败: 系统与终端均未生效".into();
            }
        }
        MenuAction::ShowCurrent => {
            app.status = current_proxy_summary().unwrap_or_else(|e| format!("读取失败: {e}"));
        }
        MenuAction::RunSpeedTest => {
            app.screen = Screen::SpeedTest;
            run_speed_test(app, terminal).await?;
        }
        MenuAction::TurnOff => {
            let result = clear();
            app.status = if result.terminal || result.system {
                current_proxy_summary().unwrap_or_else(|_| "当前: 未开启".into())
            } else {
                "关闭失败".into()
            };
        }
        MenuAction::Quit => return Ok(true),
    }
    Ok(false)
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut terminal = setup_terminal().context("初始化终端失败")?;
    let mut app = App::new();
    let mut should_quit = false;

    loop {
        terminal.draw(|frame| ui::draw(frame, &mut app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if !should_handle_key(&key) {
                    continue;
                }

                match app.screen {
                    Screen::Main => {
                        if is_quit(&key) {
                            should_quit = true;
                        } else if matches!(key.code, KeyCode::Up | KeyCode::Char('k')) {
                            app.move_selection(-1);
                        } else if matches!(key.code, KeyCode::Down | KeyCode::Char('j')) {
                            app.move_selection(1);
                        } else if is_submit(&key) {
                            if let Some(action) = app.selected_action().cloned() {
                                if handle_main_action(&mut app, action, &mut terminal).await? {
                                    should_quit = true;
                                }
                            }
                        }
                    }
                    Screen::SpeedTest => {
                        if !app.speed_testing && (is_submit(&key) || is_quit(&key)) {
                            app.screen = Screen::Main;
                        }
                    }
                }
            }
        }

        if should_quit {
            break;
        }
    }

    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<DefaultTerminal> {
    Ok(ratatui::init())
}

fn restore_terminal(_terminal: &mut DefaultTerminal) -> Result<()> {
    ratatui::restore();
    Ok(())
}
