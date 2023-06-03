use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Chart, Dataset, Gauge},
    Frame,
};

use crate::{app::timerstate::TimerType, config::Config, App};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let colors = Config::read().color;
    let black = colors.black;
    let white = colors.white;
    let gray = colors.gray;
    let red = colors.red;
    let green = colors.green;

    let chunks = Layout::default()
        .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
        .split(f.size());

    let timer_state = &app.timer;
    let stats = &app.timer.stat_data;
    let work_data = stats.get_work_data();
    let rest_data = stats.get_rest_data();
    // println!("{:?}", work_data);
    let datasets = vec![
        Dataset::default()
            .name("Work")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(red))
            .data(&work_data),
        Dataset::default()
            .name("Rest")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(green))
            .data(&rest_data),
    ];
    let stat_chart = Chart::new(datasets)
        .block(Block::default())
        .x_axis(
            Axis::default()
                .style(Style::default())
                .bounds([0.0, work_data.len() as f64]),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(black))
                .bounds([app.timer.stat_data.min, app.timer.stat_data.max])
                .labels(vec![
                    Span::styled(
                        (app.timer.stat_data.min / 3600.0).ceil().to_string(),
                        Style::default().fg(gray),
                    ),
                    // Span::raw("0"),
                    Span::styled(
                        (app.timer.stat_data.max / 3600.0).ceil().to_string(),
                        Style::default().fg(gray),
                    ),
                ]),
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
