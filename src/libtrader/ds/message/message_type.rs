#[derive(PartialEq, Debug)]
pub enum MessageType {
    Command,
    DataTransfer,
    ServerReturn
}
