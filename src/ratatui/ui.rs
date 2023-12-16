use color_eyre::eyre::Result;
use super::super::app::renderer::Renderer;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{Frame, Terminal};
use ratatui::widgets::Paragraph;
use ratatui::backend::CrosstermBackend as Backend;

pub struct Tui {
    pub terminal: ratatui::Terminal<Backend<std::io::Stderr>>
}

impl Tui {
    pub fn new() -> Result<Self> {
	Ok(Self {
	    terminal: Terminal::new(Backend::new(std::io::stderr()))?
	})
    }

    pub fn render(frame: &mut Frame) {
	frame.render_widget(Paragraph::new("Press q to exit..."), frame.size());
    }
}

impl Renderer for Tui {
    fn start(&mut self) -> Result<()> {
	enable_raw_mode()?;
	execute!(std::io::stderr(), EnterAlternateScreen)?;
	Ok(())
    }

    fn terminate(&mut self) -> Result<()> {
	execute!(std::io::stderr(), LeaveAlternateScreen)?;
	disable_raw_mode()?;
	Ok(())
    }

    fn draw(&mut self) -> Result<()> {
	self.terminal.draw(|frame| {
	    Self::render(frame)
	})?;

	Ok(())
    }

}
