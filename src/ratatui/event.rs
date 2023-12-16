use color_eyre::eyre::Result;
use crate::app::event_listener::{EventListener, Event};
use crossterm::event::{self, Event::Key, KeyCode::Char};

pub struct TuiEventListener {

}

impl TuiEventListener {
    pub fn new() -> Self {
	Self {

	}
    }
}

impl EventListener for TuiEventListener {
    fn begin(&mut self) -> Result<()> {
	Ok(())
    }

    fn end(&mut self) -> Result<()> {
	Ok(())
    }

    fn listen(&mut self) -> Result<Event> {
	if event::poll(std::time::Duration::from_millis(10000))? {
	    if let Key(key) = event::read()? {
		if key.kind == event::KeyEventKind::Press {
		    match key.code {
			Char('q') => {
			    return Ok(Event::QuitApp)
			},
			_ => {},
		    }
		}
	    }
	}
	Ok(Event::None)
    }
}
