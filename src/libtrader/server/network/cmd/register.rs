use crate::ds::message::message::Message;
use crate::ds::message::message_type::MessageType;
use crate::parser::message_builder::message_builder;
use crate::ds::generic::global_state::GlobalState;
use crate::network::tls_connection::TlsConnection;
use crate::account::acc_creation::acc_create;

#[cfg(feature="server")]
pub fn register(tls_connection: &mut TlsConnection, message: &Message) {
    /* assert recieved message */
    if message.msgtype != MessageType::Command || message.argument_count != 6
        || message.data_message_number != 0 || message.data_message_max != 0
        || message.data.len() != 0 {
        warn!("REGISTER_INVALID_MESSAGE");
        return;
    }

    /* call acc_create() server version */
    match acc_create(tls_connection, message) {
        Ok(_) => {
            match message_builder(MessageType::ServerReturn, 1, 0, 0, 0, Vec::new()) {
                Ok(msg) => {},
                Err(_) => {}
            }
        },
        Err(err) => warn!("REGISTER_FAILED: {}", err)
    }
}
