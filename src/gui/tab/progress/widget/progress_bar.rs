use tui::backend::Backend;
use tui::layout::{Constraint, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Gauge, Sparkline};
use tui::{symbols, Frame};

use crate::gui::app::App;
use crate::gui::theme::{THEME, theme_block};

pub fn draw<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Length(3),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(area);
    f.render_widget(theme_block("Progress"), area);

    let progress = app.total_progress();
    let label = format!("{:.2}%", progress * 100.0);
    let progress_color = if progress <= 0.5 {
        THEME.red
    } else if progress < 1.0 {
        THEME.yellow
    } else {
        THEME.green
    };

    let gauge = Gauge::default()
        .block(Block::default().title("Overall:"))
        .gauge_style(
            Style::default()
                .fg(progress_color)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC | Modifier::BOLD),
        )
        .label(label)
        .ratio(progress);
    f.render_widget(gauge, chunks[0]);

    let progress_history = app.progress_history();
    let sparkline = Sparkline::default()
        .block(Block::default().title("Actions:"))
        .style(Style::default().fg(THEME.contrast))
        .data(&progress_history)
        .bar_set(symbols::bar::NINE_LEVELS);
    f.render_widget(sparkline, chunks[1]);
}
