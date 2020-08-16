use crate::common::message::message::Message;
use crate::common::message::message_type::MessageType;

use crate::server::network::tls_connection::TlsConnection;
use crate::server::account::retrieval_transaction::acc_retrieve_transaction;

pub fn retrieve_transactions(tls_connection: &mut TlsConnection, message: &Message) {
    /* assert recieved message */
    if message.msgtype != MessageType::DataTransfer || message.argument_count != 1
        || message.data_message_number != 0 || message.data_message_max != 0
            || message.data.len() == 0 {
                warn!("RETRIEVE_TRANSACTION_INVALID_MESSAGE");
                tls_connection.closing = true;
                return;
    }

    /* call acc_retrieve_transaction() server version */
    match acc_retrieve_transaction(tls_connection, message) {
        Ok(_) => {},
        Err(err) => warn!("RETRIEVE_TRANSACTION_FAILED: {}", err)
    };
}
