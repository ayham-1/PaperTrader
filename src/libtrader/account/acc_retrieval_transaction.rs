use std::io::Write;

use crate::ds::account::transaction::Transaction;
use crate::network::tls_client::TlsClient;

use crate::network::cmd::generic::wait_and_read_branched::wait_and_read_branched;

use crate::parser::message_builder::message_builder;
use crate::ds::message::message::Message;
use crate::ds::message::message_type::MessageType;
use crate::ds::message::inst::{DataTransferInst};

pub fn acc_retrieve_transaction(tls_client: &mut TlsClient, poll: &mut mio::Poll) -> Result<Vec<Transaction>, String> {
    assert_eq!(tls_client.auth_jwt.is_empty(), false);

    /* build message request */
    match message_builder(MessageType::DataTransfer, DataTransferInst::GetUserTransactionHist as i64, 1, 0, 0,
                          bincode::serialize(&tls_client.auth_jwt).unwrap()) {
        Ok(message) => {
            tls_client.write(bincode::serialize(&message).unwrap().as_slice()).unwrap();
        },
        Err(err) => return Err(err),
    };

    /* wait for response */
    wait_and_read_branched(tls_client, poll, Some(20), Some(500))?;

    /* decode response */
    let response: Message = bincode::deserialize(&tls_client.read_plaintext).unwrap();
    tls_client.read_plaintext.clear();

    if response.message_type == MessageType::ServerReturn && response.instruction == 1 
        && response.argument_count == 1 && response.data.len() != 0 {
            /* returned data*/
            let transactions: Vec<Transaction> = bincode::deserialize(&response.data).unwrap();
            return Ok(transactions);
        } else {
            /* could not get data */
            return Err("ACC_RETRIEVAL_TRANSACTION_UNAUTHORIZED".to_string());
        }
}
