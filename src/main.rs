use color_eyre::eyre::Result;
use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{CrosstermBackend, Terminal, Frame},
    widgets::Paragraph,
};

struct App {
    should_be_running: bool,
}

impl App {
    fn new() -> Self {
	Self {
	    should_be_running: true,
	}
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    start_app()?;
    let status = run();
    end_app()?;

    status?; // trow error after end_app in case there was any

    Ok(())
}

fn start_app() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Ok(())
}

fn end_app() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn run() -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
    let mut app = App::new();

    loop {
	handle_events(&mut app)?;

	terminal.draw(|f| {
	    render(&app, f);
	})?;

	// application exit
	if !app.should_be_running {
	    break;
	}
    }

    Ok(())
}

fn render(app: &App, f: &mut Frame) {
    f.render_widget(Paragraph::new("Press q to exit..."), f.size());
}

fn handle_events(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(10000))? {
	if let Key(key) = event::read()? {
	    if key.kind == event::KeyEventKind::Press {
		match key.code {
		    Char('q') => app.should_be_running = false,
		    _ => {},
		}
	    }
	}
    }
    Ok(())
}

