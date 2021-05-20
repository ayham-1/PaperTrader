use std::io::Write;

use crate::common::message::message::Message;
use crate::common::message::message_builder::message_builder;
use crate::common::message::message_type::MessageType;
use crate::common::misc::assert_msg::assert_msg;

use crate::server::account::creation::acc_create;
use crate::server::network::tls_connection::TlsConnection;

pub fn register(tls_connection: &mut TlsConnection, message: &Message) {
    /* assert recieved message */
    if !assert_msg(
        message,
        MessageType::Command,
        true,
        5,
        false,
        0,
        false,
        0,
        false,
        0,
    ) {
        warn!("REGISTER_INVALID_MESSAGE");
        tls_connection.closing = true;
        return;
    }

    /* call acc_create() server version */
    match acc_create(message) {
        Ok(_) => {
            let message = message_builder(MessageType::ServerReturn, 1, 0, 0, 0, Vec::new());
            let _ = tls_connection.write(&bincode::serialize(&message).unwrap());
        },
        Err(_) => warn!("REGISTER_FAILED: " /*{}", err*/) // TODO: OUTPUT ERRORS
    };
}
