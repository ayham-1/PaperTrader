use std::io::Write;

use crate::common::message::message::Message;
use crate::common::message::message_type::MessageType;
use crate::common::message::message_builder::message_builder;
use crate::common::misc::assert_msg::assert_msg;

use crate::server::network::tls_connection::TlsConnection;
use crate::server::account::authorization::acc_auth;

pub fn login_normal(tls_connection: &mut TlsConnection, message: &Message) {
    /* assert recieved message */
    if assert_msg(message, MessageType::Command, 3, 0, 0, 0) {
        tls_connection.closing = true;
        warn!("LOGIN_INVALID_MESSAGE");
        return;
    }

    /* call acc_auth() server version */
    match acc_auth(tls_connection, message) {
        Ok(_) => {},
        Err(err) => {
            let message = message_builder(MessageType::ServerReturn, 0, 0, 0, 0, 
                                          bincode::serialize(&err).unwrap());
            let _ = tls_connection.write(&bincode::serialize(&message).unwrap());
        }
    }
}
