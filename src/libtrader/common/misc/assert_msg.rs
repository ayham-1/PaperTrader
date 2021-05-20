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
pub fn assert_msg(
    message: &Message,
    msg_type: MessageType,
    check_arg_cnt: bool,
    arg_cnt: usize,
    check_dnum: bool,
    msg_dnum: usize,
    check_dmax: bool,
    msg_dmax: usize,
    check_len: bool,
    data_len: usize,
) -> bool {
    if message.msgtype != msg_type {
        return false;
    } else if check_arg_cnt && (message.argument_count != arg_cnt) {
        return false;
    } else if check_dnum && (message.data_message_number != msg_dnum) {
        return false;
    } else if check_dmax && (message.data_message_max != msg_dmax) {
        return false;
    } else if check_len && (message.data.len() != data_len) {
        return false;
    }

    return true;
}
