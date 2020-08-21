use crate::common::message::message::Message;
use crate::common::message::message_type::MessageType;

/// Asserts a recieved message meta information.
///
/// Takes in a message and meta information to check the message against.
///
/// Arguments:
/// message - The mesage to assert against.
/// msg_type - MessageType expected.
/// arg_cnt - The argument count expected.
/// msg_dnum - The number of the data message.
/// msg_dmax - The number of the max data message.
/// data_len - The length of the data payload.
///
/// Returns: a boolean.
pub fn assert_msg(message: &Message, msg_type: MessageType, arg_cnt: usize, msg_dnum: usize, msg_dmax: usize, 
                  data_len: usize) -> bool {
    if message.msgtype != msg_type || message.argument_count != arg_cnt
        || message.data_message_number != msg_dnum || message.data_message_max != msg_dmax
            || message.data.len() == data_len {
                warn!("ASSERT_MSG_FAILED");
                return false;
            }

    return true;
}
