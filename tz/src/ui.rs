use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{List, ListItem, Paragraph, Row, Table};
use ratatui::Frame;

use crate::proxy::PROXIES;
use crate::theme::Theme;
use crate::App;

pub fn status_line(status: &str) -> Line<'static> {
    if status.contains("未开启") || status.contains("关闭") {
        Line::from(vec![
            Span::styled("● ", Theme::status_off()),
            Span::styled(status.to_string(), Theme::status_off()),
        ])
    } else if status.contains("失败") {
        Line::from(vec![
            Span::styled("● ", Theme::status_err()),
            Span::styled(status.to_string(), Theme::status_err()),
        ])
    } else {
        Line::from(vec![
            Span::styled("● ", Theme::status_active()),
            Span::styled(status.to_string(), Theme::value()),
        ])
    }
}

pub fn draw_main(frame: &mut Frame, app: &mut App) {
    frame.render_widget(
        Paragraph::new("").style(Style::default().bg(Theme::BG)),
        frame.area(),
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(8), Constraint::Length(3)])
        .split(frame.area());

    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(" tz ", Theme::badge()),
            Span::raw("  "),
            Span::styled(
                "Proxy Switcher",
                Style::default()
                    .fg(Theme::ACCENT)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        status_line(&app.status),
    ])
    .block(Theme::block_active("路由控制台"));
    frame.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = app
        .menu_items
        .iter()
        .map(|entry| {
            if entry.label.starts_with('─') {
                ListItem::new(entry.label.clone()).style(Theme::menu_sep())
            } else {
                let port = entry
                    .label
                    .split(':')
                    .next_back()
                    .and_then(|p| p.trim().parse().ok())
                    .unwrap_or(0);
                ListItem::new(entry.label.clone()).style(Theme::proxy_port_style(port))
            }
        })
        .collect();

    let list = List::new(items)
        .block(Theme::block("↑↓ 选择  Enter/Space 确认  q 退出"))
        .highlight_style(Theme::menu_highlight())
        .highlight_symbol("▸ ");

    frame.render_stateful_widget(list, chunks[1], &mut app.list_state);

    let footer = Paragraph::new("系统代理 + 终端 http_proxy/all_proxy  ·  退出 tz 后当前 shell 自动生效")
        .style(Theme::footer())
        .block(Theme::block("提示"));
    frame.render_widget(footer, chunks[2]);
}

pub fn draw_speed(frame: &mut Frame, app: &App) {
    frame.render_widget(
        Paragraph::new("").style(Style::default().bg(Theme::BG)),
        frame.area(),
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(6), Constraint::Length(3)])
        .split(frame.area());

    let title = if app.speed_testing {
        format!("测速中... ({}/{})", app.speed_done, PROXIES.len())
    } else {
        "延迟越低越好".into()
    };
    frame.render_widget(
        Paragraph::new(title)
            .style(Theme::value())
            .block(Theme::block_active("代理测速")),
        chunks[0],
    );

    let header = Row::new(vec!["节点", "端口", "出口 IP", "延迟", "状态"]).style(Theme::table_header());
    let rows: Vec<Row> = app
        .speed_results
        .iter()
        .map(|r| {
            let latency_cell = r
                .latency_ms
                .map(|ms| Span::styled(format!("{ms} ms"), Theme::latency(ms)))
                .unwrap_or_else(|| Span::styled("-", Theme::menu_sep()));
            let status_style = if r.status == "OK" {
                Theme::status_ok()
            } else {
                Theme::status_err()
            };
            Row::new(vec![
                Span::styled(
                    format!("{} {}", r.proxy.flag, r.proxy.name),
                    Theme::proxy_port_style(r.proxy.port),
                ),
                Span::styled(r.proxy.port.to_string(), Theme::value()),
                Span::styled(r.ip.clone(), Theme::value()),
                latency_cell,
                Span::styled(r.status.clone(), status_style),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(14),
            Constraint::Length(6),
            Constraint::Min(16),
            Constraint::Length(10),
            Constraint::Length(10),
        ],
    )
    .header(header)
    .block(Theme::block("测速结果"));
    frame.render_widget(table, chunks[1]);

    let hint = if app.speed_testing {
        Span::styled("请稍候，正在逐个测试节点...", Theme::footer())
    } else {
        Span::styled("Enter / Esc  返回主菜单", Theme::accent_span())
    };
    frame.render_widget(
        Paragraph::new(Line::from(hint)).block(Theme::block("操作")),
        chunks[2],
    );
}

pub fn draw(frame: &mut Frame, app: &mut App) {
    match app.screen {
        crate::Screen::Main => draw_main(frame, app),
        crate::Screen::SpeedTest => draw_speed(frame, app),
    }
}
