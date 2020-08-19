use std::io::Write;

use crate::common::message::message::Message;
use crate::common::message::message_type::MessageType;
use crate::common::message::message_builder::message_builder;

use crate::server::network::tls_connection::TlsConnection;
use crate::server::account::creation::acc_create;

pub fn register(tls_connection: &mut TlsConnection, message: &Message) {
    /* assert recieved message */
    if message.msgtype != MessageType::Command || message.argument_count != 5
        || message.data_message_number != 0 || message.data_message_max != 0
            || message.data.len() == 0 {
                warn!("REGISTER_INVALID_MESSAGE");
                tls_connection.closing = true;
                return;
    }

    /* call acc_create() server version */
    match acc_create(message) {
        Ok(_) => {
            match message_builder(MessageType::ServerReturn, 1, 0, 0, 0, Vec::new()) {
                Ok(msg) => {
                    let _ = tls_connection.write(bincode::serialize(&msg).unwrap().as_slice());
                },
                Err(_) => {}
            }
        },
        Err(err) => warn!("REGISTER_FAILED: {}", err)
    };
}
