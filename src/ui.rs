use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    widgets::{Gauge, Paragraph},
    Frame,
};

use crate::{app::timerstate::TimerType, config::Config, App};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let colors = Config::read().color;
    let white = colors.white;
    let gray = colors.gray;

    let chunks = Layout::default()
        .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
        .split(f.size());

    let timer_state = &app.timer;
    let empty_space_above = Paragraph::new(vec![]);
    f.render_widget(empty_space_above, chunks[0]);

    let full_secs = app.timer.expiry.duration.as_secs();
    let remaining_secs = app.timer.expiry.get();

    let progress = (remaining_secs as f64) * 100.0 / (full_secs as f64);
    let progress = if progress / 100.0 > 0.0 {
        progress / 100.0
    } else {
        0.0
    };
    let progress = if app.timer.enabled { progress } else { 1.0 };

    let current_type = match app.timer.timer_type {
        TimerType::Work => "Work",
        TimerType::Rest => "Rest",
    };

    let progress_label = if app.timer.enabled {
        format!("{} {}", current_type, timer_state.expiry.format())
    } else {
        String::from("Disabled")
    };
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
