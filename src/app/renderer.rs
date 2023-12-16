use color_eyre::eyre::Result;

pub trait Renderer {
    /// Called to setup the renderer environment
    fn start(&mut self) -> Result<()>;

    /// Called to dispose of any renderer environment setup side-effects and handle renderer termination
    fn terminate(&mut self) -> Result<()>;

    fn draw(&mut self) -> Result<()>;
}
