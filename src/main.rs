pub mod app;
pub mod color;
pub mod config;
pub mod env;
pub mod expiry;
pub mod formatter;
pub mod fs;
pub mod keys;
pub mod ui;

use app::App;
use config::Config;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use keys::Keys;
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use ui::draw;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // run app
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let tick_rate = Config::read().app.tick_rate;
    let tick_rate = Duration::from_millis(tick_rate);

    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| draw(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        app.timer.check();

        let keys = Keys::get();

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char(keys.get("quit").unwrap().key) {
                    return Ok(());
                } else if key.code == KeyCode::Char(keys.get("work").unwrap().key) {
                    app.timer.change_to_work(false);
                } else if key.code == KeyCode::Char(keys.get("rest").unwrap().key) {
                    app.timer.change_to_rest(false);
                } else if key.code == KeyCode::Char(keys.get("toggle_enabled").unwrap().key) {
                    app.timer.toggle_enabled();
                } else if key.code == KeyCode::Char(keys.get("toggle_help_popup").unwrap().key) {
                    app.help.toggle_is_open();
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}
