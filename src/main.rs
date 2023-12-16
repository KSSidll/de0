mod app;
mod ratatui;

use color_eyre::eyre::Result;
use app::app::App;
use ratatui::{ui::Tui, event::TuiEventListener};

fn main() -> Result<()> {
    color_eyre::install()?;

    let renderer = Tui::new()?;
    let event_listener = TuiEventListener::new();

    let mut app = App::new(renderer, event_listener);

    app.start()?;
    let status = app.run();
    app.terminate()?;

    status?; // throw error after end_app in case there was any

    Ok(())
}
