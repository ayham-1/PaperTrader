use crate::common::message::message::Message;
use crate::common::message::message_type::MessageType;
use crate::common::message::message_builder::message_builder;

use crate::server::network::tls_connection::TlsConnection;
use crate::server::account::retrieval_portfolio::acc_retrieve_portfolio;

pub fn retrieve_portfolio(tls_connection: &mut TlsConnection, message: &Message) {
    /* assert recieved message */
    if message.msgtype != MessageType::Command || message.argument_count != 1
        || message.data_message_number != 0 || message.data_message_max != 0
            || message.data.len() == 0 {
                warn!("RETRIEVE_PORTFOLIO_INVALID_MESSAGE");
                tls_connection.closing = true;
                return;
    }

    /* call acc_retrieve_portfolio() server version */
    match acc_retrieve_portfolio(tls_connection, message) {
        Ok(_) => {},
        Err(err) => warn!("REGISTER_FAILED: {}", err)
    };
    tls_connection.closing = true;
}
