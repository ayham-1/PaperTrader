use crate::common::message::message::Message;
use crate::common::message::message_type::MessageType;

/// Asserts a recieved message meta information.
///
/// Takes in a message and meta information to check the message against.
/// Can be used to check some attributes.
/// For example, you can check for message to have an X amount of arguments, and
/// not check how many arguments are passed.
///
/// Arguments:
/// message - The mesage to assert against.
/// msg_type - MessageType expected.
/// check_arg_cnt - Whether to check for the argument account.
/// arg_cnt - The argument count expected.
/// check_dnum - Whether to check for the number data message.
/// msg_dnum - The number of the data message.
/// check_dmax - Whether to check for the max data message.
/// msg_dmax - The number of the max data message.
/// check_len - Whether to check for the data payload length.
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
