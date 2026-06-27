use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::Block;

/// Frost Glass — 终端模拟磨砂半透明（真透明需终端支持，此处用配色+纹理模拟）
pub struct Theme;

impl Theme {
    pub const BG: Color = Color::Rgb(8, 12, 22);
    pub const GLASS: Color = Color::Rgb(38, 48, 68);
    pub const GLASS_LIGHT: Color = Color::Rgb(58, 68, 88);
    pub const GLASS_BORDER: Color = Color::Rgb(160, 175, 200);
    pub const GLASS_GLOW: Color = Color::Rgb(120, 200, 255);
    pub const OVERLAY: Color = Color::Rgb(6, 10, 18);
    pub const FROST_TEX: Color = Color::Rgb(35, 45, 62);
    pub const TEXT: Color = Color::Rgb(235, 240, 248);
    pub const MUTED: Color = Color::Rgb(130, 145, 165);
    pub const ACCENT: Color = Color::Rgb(100, 210, 255);
    pub const ACCENT_FG: Color = Color::Rgb(8, 12, 22);
    pub const HIGHLIGHT_BG: Color = Color::Rgb(50, 90, 130);
    pub const SUCCESS: Color = Color::Rgb(80, 220, 170);
    pub const WARN: Color = Color::Rgb(255, 200, 80);
    pub const ERROR: Color = Color::Rgb(255, 120, 120);
    pub const HOME: Color = Color::Rgb(190, 160, 255);

    pub fn block(title: &str) -> Block<'_> {
        Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(Self::GLASS_BORDER))
            .title(format!(" {title} "))
            .title_style(
                Style::default()
                    .fg(Self::ACCENT)
                    .add_modifier(Modifier::BOLD),
            )
            .style(Style::default().bg(Self::GLASS).fg(Self::TEXT))
    }

    pub fn block_active(title: &str) -> Block<'_> {
        Self::block(title).border_style(
            Style::default()
                .fg(Self::GLASS_GLOW)
                .add_modifier(Modifier::BOLD),
        )
    }

    pub fn badge() -> Style {
        Style::default()
            .fg(Self::ACCENT_FG)
            .bg(Self::ACCENT)
            .add_modifier(Modifier::BOLD)
    }

    pub fn status_active() -> Style {
        Style::default().fg(Self::SUCCESS).add_modifier(Modifier::BOLD)
    }

    pub fn status_off() -> Style {
        Style::default().fg(Self::MUTED)
    }

    pub fn label() -> Style {
        Style::default()
            .fg(Self::MUTED)
            .add_modifier(Modifier::BOLD)
    }

    pub fn value() -> Style {
        Style::default().fg(Self::TEXT)
    }

    pub fn menu_sep() -> Style {
        Style::default().fg(Self::FROST_TEX)
    }

    pub fn menu_highlight() -> Style {
        Style::default()
            .fg(Color::White)
            .bg(Self::HIGHLIGHT_BG)
            .add_modifier(Modifier::BOLD)
    }

    pub fn footer() -> Style {
        Style::default().fg(Self::MUTED)
    }

    pub fn table_header() -> Style {
        Style::default()
            .fg(Self::ACCENT)
            .add_modifier(Modifier::BOLD)
    }

    pub fn latency(ms: u128) -> Style {
        let color = if ms < 300 {
            Self::SUCCESS
        } else if ms < 800 {
            Self::WARN
        } else {
            Self::ERROR
        };
        Style::default().fg(color)
    }

    pub fn status_ok() -> Style {
        Style::default().fg(Self::SUCCESS)
    }

    pub fn status_err() -> Style {
        Style::default().fg(Self::ERROR)
    }

    pub fn proxy_port_style(port: u16) -> Style {
        let color = match port {
            1081 => Color::Rgb(255, 130, 130),
            1082 => Color::Rgb(255, 170, 90),
            1083 => Color::Rgb(120, 180, 255),
            1084 => Color::Rgb(100, 230, 150),
            1085 => Color::Rgb(150, 160, 255),
            1086 => Self::HOME,
            _ => Self::TEXT,
        };
        Style::default().fg(color)
    }

    pub fn accent_span() -> Style {
        Style::default().fg(Self::ACCENT)
    }
}
