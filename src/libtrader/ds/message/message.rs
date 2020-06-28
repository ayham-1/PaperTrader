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
