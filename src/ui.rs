use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Gauge, Paragraph, Tabs},
    Frame,
};

use crate::{config::Config, App};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let colors = Config::read();
    let gray = colors.color.gray;
    let white = colors.color.white;
    let black = colors.color.black;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0)].as_ref())
        .split(f.size());

    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default())))
        .collect();

    let tabs = Tabs::new(titles)
        .divider("")
        .block(Block::default().borders(Borders::NONE))
        .select(app.tabs.active_index)
        .style(Style::default().fg(gray))
        .highlight_style(Style::default().bg(white).fg(black));
    f.render_widget(tabs, chunks[0]);

    match app.tabs.active_index {
        0 => draw_timer_tab(f, app, chunks[1]),
        1 => draw_stats_tab(f, app, chunks[1]),
        _ => {}
    };
}

fn draw_timer_tab<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let colors = Config::read();
    let white = colors.color.white;
    let gray = colors.color.gray;

    let chunks = Layout::default()
        .constraints(
            [
                // TODO: figure out this 4 later
                Constraint::Min(4),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(area);

    let timer_tab_content = Paragraph::new(Spans::from("This is the timer tab"));
    f.render_widget(timer_tab_content, chunks[0]);

    // TODO: change this to time remaining & percentage for ratio based on that
    // timer and the full time of the current timer cycle
    let progress = 0.41;
    let progress_label = format!("{}%", progress * 100.0);
    let timer_progress = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(white)
                .bg(gray)
                .add_modifier(Modifier::BOLD),
        )
        .label(progress_label)
        .ratio(progress);
    f.render_widget(timer_progress, chunks[1]);
}

fn draw_stats_tab<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let stats_tab_content = Paragraph::new(Spans::from("This is the stats tab"));
    f.render_widget(stats_tab_content, area);
}
