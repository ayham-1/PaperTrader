use crate::common::message::message::Message;
use crate::common::message::message_type::MessageType;
use crate::common::misc::assert_msg::assert_msg;

use crate::server::network::tls_connection::TlsConnection;
use crate::server::account::retrieval_portfolio::acc_retrieve_portfolio;

pub fn retrieve_portfolio(tls_connection: &mut TlsConnection, message: &Message) {
    /* assert recieved message */
    if assert_msg(message, MessageType::Command, 1, 0, 0, 0) {
        tls_connection.closing = true;
        warn!("RETRIEVE_PORTFOLIO_INVALID_MESSAGE");
        return;
    }

    /* call acc_retrieve_portfolio() server version */
    match acc_retrieve_portfolio(tls_connection, message) {
        Ok(_) => {},
        Err(err) => warn!("RETRIEVE_PORTFOLIO_FAILED: {}", err)
    };
}
