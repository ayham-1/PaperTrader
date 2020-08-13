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
            match message_builder(MessageType::ServerReturn, 0, 0, 0, 0, err.as_bytes().to_vec()) {
                Ok(msg) => {
                    match tls_connection.tls_session.write(bincode::serialize(&msg).unwrap().as_slice()) {
                        Ok(_) => {tls_connection.do_tls_write_and_handle_error();return},
                        Err(err) => warn!("LOGIN_NORMAL_FAILED_SENDING_RESPONSE: {}", err)
                    }
                },
                Err(_) => {}
            }
        }
    };
}
