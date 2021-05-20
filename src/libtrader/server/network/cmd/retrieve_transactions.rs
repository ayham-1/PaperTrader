use crate::common::message::message::Message;
use crate::common::message::message_type::MessageType;
use crate::common::misc::assert_msg::assert_msg;

use crate::server::account::retrieval_transaction::acc_retrieve_transaction;
use crate::server::network::tls_connection::TlsConnection;

pub fn retrieve_transactions(tls_connection: &mut TlsConnection, message: &Message) {
    /* assert recieved message */
    if !assert_msg(
        message,
        MessageType::DataTransfer,
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
        warn!("RETRIEVE_TRANSACTION_INVALID_MESSAGE");
        return;
    }

    /* call acc_retrieve_transaction() server version */
    match acc_retrieve_transaction(tls_connection, message) {
        Ok(_) => {}
        Err(err) => warn!("RETRIEVE_TRANSACTION_FAILED: {}", err),
    };
}
