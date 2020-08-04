#[derive(PartialEq, Debug)]
pub enum MessageType {
    Command = 0,
    DataTransfer = 1,
    ServerReturn = 2
}
impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Default for MessageType {
    fn default() -> Self {
        MessageType::Command
    }
}
