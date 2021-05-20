use crate::common::message::message::Message;
use crate::common::message::message_type::MessageType;
use crate::common::misc::assert_msg::assert_msg;

use crate::server::account::retrieval_portfolio::acc_retrieve_portfolio;
use crate::server::network::tls_connection::TlsConnection;

pub fn retrieve_portfolio(tls_connection: &mut TlsConnection, message: &Message) {
    /* assert recieved message */
    if !assert_msg(
        message,
        MessageType::Command,
        true,
        1,
        false,
        0,
        false,
        0,
        false,
        0,
    ) {
        tls_connection.closing = true;
        warn!("RETRIEVE_PORTFOLIO_INVALID_MESSAGE");
        return;
    }

    /* call acc_retrieve_portfolio() server version */
    match acc_retrieve_portfolio(tls_connection, message) {
        Ok(_) => {},
        Err(_) => warn!("RETRIEVE_PORTFOLIO_FAILED: " /*{}", err*/) // TODO: OUTPUT ERRORS
    };
}
