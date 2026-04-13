use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum MessageType {
    Info,
    Warning,
    Error,
}
impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_str = match self {
            MessageType::Info => "INFO",
            MessageType::Warning => "WARNING",
            MessageType::Error => "ERROR",
        };
        write!(f, "{type_str}")
    }
}

#[derive(Clone, Debug)]
pub struct DialogMessage {
    pub msg: String,
    pub msg_type: MessageType,
    pub action: Action,
}

#[derive(Clone, Debug)]
pub enum Action {
    RemoveLock,
    RemoveOrphans,
    SetDnsServer,
    InstallGaming,
    InstallWinboat,
    InstallVramManagement,
}

pub trait UI {
    fn show_message(&self, message_type: MessageType, message: &str, title: String);
}

pub type RunCmdCallback = fn(command: &str, escalate: bool) -> bool;
