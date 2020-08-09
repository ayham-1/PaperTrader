use crate::ds::message::message::Message;
use crate::ds::message::message_type::MessageType;

pub fn message_builder(msg_type: MessageType, inst: i64, arg_cnt: usize, data_msg_num: usize,
                       data_msg_max: usize, data: Vec<u8>) -> Result<Message, String> {
    let mut message: Message = Message::default();
    message.msgtype = msg_type;
    message.instruction = inst;
    message.argument_count = arg_cnt;
    message.data_message_number = data_msg_num;
    message.data_message_max = data_msg_max;
    message.data = data;
    Ok(message)
}
