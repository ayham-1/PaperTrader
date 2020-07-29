use crate::ds::message::message_type::{MessageType};

#[derive(PartialEq, Debug)]
pub struct Message {
    pub message_type: MessageType,
    pub instruction: i64,
    pub data_size: usize,
    pub argument_count: usize,
    pub data_message_number: usize,
    pub data_message_max: usize,
    pub data: Box<[char]>,
}
impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {}, {}, {}, {:#?})", self.message_type, self.instruction, self.data_size, self.argument_count, self.data_message_number, self.data_message_max, self.data)
    }
}
