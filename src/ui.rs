use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::Spans,
    widgets::{Axis, Block, Borders, Chart, Clear, Dataset, Gauge, GraphType, Paragraph},
    Frame,
};

use crate::{
    app::timer::TimerType,
    keys::{Key, Keys},
    App, CONFIG,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let black = CONFIG.get_colors().black;
    let red = CONFIG.get_colors().red;
    let green = CONFIG.get_colors().green;

    let chunks = Layout::default()
        .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
        .split(f.size());

    let timer_state = &app.timer;
    let stats = &app.stat;
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
                .bounds([app.stat.min, app.stat.max]),
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

    if app.help.is_open {
        draw_help_text(f);
    }
}

fn draw_help_text<B: Backend>(f: &mut Frame<B>) {
    let block = Block::default().title("Shortcuts").borders(Borders::ALL);
    let area = centered_rect(50, 50, f.size());
    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(block, area);

    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .vertical_margin(2)
        .horizontal_margin(4)
        .split(area);

    let keys = Keys::get();
    let mut keys: Vec<Key> = keys.values().cloned().collect();
    keys.sort_by_key(|k| k.order);
    let mut text: Vec<Spans> = vec![];
    for value in keys {
        text.push(Spans::from(
            String::from(value.key)
                + &String::from(" => ")
                + &String::from(value.description.clone()),
        ));
    }

    let paragraph = Paragraph::new(text);
    f.render_widget(paragraph, chunks[0])
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn get_guage_colors(timer_type: &TimerType, is_enabled: bool) -> (Color, Color) {
    let gray = CONFIG.get_colors().gray;
    let red = CONFIG.get_colors().red;
    let green = CONFIG.get_colors().green;
    let black = CONFIG.get_colors().black;

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
