use color_eyre::eyre::Result;

pub trait EventListener {
    /// Start handling events
    fn begin(&mut self) -> Result<()>;

    /// Stop handling events, should be called before application termination
    fn end(&mut self) -> Result<()>;

    /// Listen to events
    fn listen(&mut self) -> Result<Event>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    KeyPress(KeyCode),
    QuitApp,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyCode {

}
