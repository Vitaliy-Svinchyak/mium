use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders};

pub struct Theme {
    pub green: Color,
    pub yellow: Color,
    pub red: Color,
    pub contrast: Color,
    pub border: Color,
    pub white_text: Color,
}

pub static THEME: Theme = Theme {
    green: Color::Rgb(70, 166, 100),
    yellow: Color::Rgb(254, 188, 88),
    red: Color::Rgb(248, 76, 72),
    contrast: Color::Rgb(225, 69, 145),
    border: Color::Rgb(111, 111, 111),
    white_text: Color::Rgb(205, 205, 205),
};

pub fn theme_block(title: &str) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .title(Spans::from(Span::styled(
            title,
            Style::default().fg(Color::Rgb(188, 188, 188)),
        )))
        .border_style(Style::default().fg(THEME.border))
        .border_type(BorderType::Rounded)
}
