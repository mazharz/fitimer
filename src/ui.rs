use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    symbols,
    widgets::{Axis, Block, Chart, Dataset, Gauge, GraphType},
    Frame,
};

use crate::{app::timerstate::TimerType, config::Config, App};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let colors = Config::read().color;
    let black = colors.black;
    let red = colors.red;
    let green = colors.green;

    let chunks = Layout::default()
        .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
        .split(f.size());

    let timer_state = &app.timer;
    let stats = &app.timer.stat_data;
    let work_data = stats.get_work_data();
    let rest_data = stats.get_rest_data();
    let datasets = vec![
        Dataset::default()
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(red))
            .graph_type(GraphType::Line)
            .data(&work_data),
        Dataset::default()
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(green))
            .graph_type(GraphType::Line)
            .data(&rest_data),
    ];
    let stat_chart = Chart::new(datasets)
        .block(Block::default())
        .x_axis(
            Axis::default()
                .style(Style::default())
                .bounds([0.0, (work_data.len() - 1) as f64]),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(black))
                .bounds([app.timer.stat_data.min, app.timer.stat_data.max]),
        );
    f.render_widget(stat_chart, chunks[0]);

    let full_secs = app.timer.expiry.duration.as_secs();
    let remaining_secs = app.timer.expiry.get_remaining();

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
    let guage_colors = get_guage_colors(&app.timer.timer_type, app.timer.enabled);
    let timer_progress = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(guage_colors.0)
                .bg(guage_colors.1)
                .add_modifier(Modifier::BOLD),
        )
        .label(progress_label)
        .ratio(progress);
    f.render_widget(timer_progress, chunks[1]);
}

fn get_guage_colors(timer_type: &TimerType, is_enabled: bool) -> (Color, Color) {
    let colors = Config::read().color;
    let gray = colors.gray;
    let red = colors.red;
    let green = colors.green;
    let black = colors.black;

    if !is_enabled {
        return (black, gray);
    }

    match timer_type {
        TimerType::Work => {
            return (red, black);
        }
        _ => return (green, black),
    }
}
