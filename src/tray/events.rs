#[derive(Debug, Clone, PartialEq)]
pub enum TrayEvent {
    Title,
    Running,
    Exit,
}

impl From<&str> for TrayEvent {
    fn from(s: &str) -> Self {
        match s {
            "Title" => TrayEvent::Title,
            "Running" => TrayEvent::Running,
            "Exit" => TrayEvent::Exit,
            _ => TrayEvent::Title,
        }
    }
}

impl TrayEvent {
    pub fn as_str(&self) -> &str {
        match self {
            TrayEvent::Title => "Title",
            TrayEvent::Running => "Running",
            TrayEvent::Exit => "Exit",
        }
    }
}
