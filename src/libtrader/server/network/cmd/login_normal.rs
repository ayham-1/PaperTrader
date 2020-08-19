use std::io::Write;

use crate::common::message::message::Message;
use crate::common::message::message_type::MessageType;
use crate::common::message::message_builder::message_builder;

use crate::server::network::tls_connection::TlsConnection;
use crate::server::account::authorization::acc_auth;

pub fn login_normal(tls_connection: &mut TlsConnection, message: &Message) {
    /* assert recieved message */
    if message.msgtype != MessageType::Command || message.argument_count != 3
        || message.data_message_number != 0 || message.data_message_max != 0
           || message.data.len()  == 0 {
               warn!("LOGIN_INVALID_MESSAGE");
               tls_connection.closing = true;
               return;
           }

    /* call acc_auth() server version */
    match acc_auth(tls_connection, message) {
        Ok(_) => {},
        Err(err) => {
            let message = message_builder(MessageType::ServerReturn, 0, 0, 0, 0, err.as_bytes().to_vec());
            let _ = tls_connection.write(&bincode::serialize(&message).unwrap());
        }
    }
}
