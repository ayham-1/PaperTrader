#[derive(PartialEq, Debug)]
pub enum MessageType {
    Command,
    DataTransfer,
    ServerReturn
}
impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
