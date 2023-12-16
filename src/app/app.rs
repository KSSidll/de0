use color_eyre::eyre::Result;

#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState {
    #[default] Running,
    Stopped,
}

use crate::{app::renderer::Renderer, ratatui::ui::Tui};
use crate::{app::event_listener::EventListener, ratatui::event::TuiEventListener};

use super::event_listener::Event;

pub struct App {
    running_status: RunningState,
    renderer: Tui,
    event_listener: TuiEventListener,
}

impl App {
    // pub fn new(renderer: Box<dyn Renderer>, event_handler: Box<dyn EventHandler>) -> Self {
    pub fn new(renderer: Tui, event_listener: TuiEventListener) -> Self {
	Self {
	    running_status: RunningState::default(),
	    renderer,
	    event_listener,
	}
    }

    pub fn start(&mut self) -> Result<()> {
	self.renderer.start()?;
	self.event_listener.begin()?;

	Ok(())
    }

    pub fn terminate(&mut self) -> Result<()> {
	self.event_listener.end()?;
	self.renderer.terminate()?;

	Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
	loop {
	    // application exit
	    if self.running_status == RunningState::Stopped {
		break;
	    }

	    self.handle_events()?;

	    self.renderer.draw()?
	}

	Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
	let event = self.event_listener.listen()?;

	if event == Event::QuitApp {
	    self.running_status = RunningState::Stopped;
	}

	Ok(())
    }
}
