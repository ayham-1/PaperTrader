use std::io::Write;

use crate::common::account::transaction::Transaction;

use crate::common::message::inst::DataTransferInst;
use crate::common::message::message::Message;
use crate::common::message::message_builder::message_builder;
use crate::common::message::message_type::MessageType;
use crate::common::misc::return_flags::ReturnFlags;

use crate::client::network::cmd::wait_and_read_branched::wait_and_read_branched;
use crate::client::network::tls_client::TlsClient;

/// Retrieves ffrom the connected TLS server an authorized transaction history.
///
/// Sends a request for a transaction history with the JWT token of the client connection. Handles
/// any response and returns.
///
/// Arguments:
/// tls_client - TLS client to use containing the JWT token to authorize.
/// poll - For event handling.
///
/// Returns: transaction vector on success, string on error containing reason of failure.
///
/// Example:
/// ```rust
///     match acc_retrieve_transaction(&mut tls_client, &mut poll) {
///         Ok(transaction) => {/* interesting stuff with portfolio */},
///         Err(err) => panic!("can not retrieve transaction history! error: {}", err)
///     };
/// ```
pub fn acc_retrieve_transaction(
    tls_client: &mut TlsClient,
    poll: &mut mio::Poll,
) -> Result<Vec<Transaction>, ReturnFlags> {
    assert_eq!(tls_client.auth_jwt.is_empty(), false);

    /* build message request */
    let message = message_builder(
        MessageType::DataTransfer,
        DataTransferInst::GetUserTransactionHist as i64,
        1,
        0,
        0,
        bincode::serialize(&tls_client.auth_jwt).unwrap(),
    );
    tls_client
        .write(&bincode::serialize(&message).unwrap())
        .unwrap();

    /* wait for response */
    wait_and_read_branched(tls_client, poll, Some(20), Some(500))?;

    /* decode response */
    let response: Message = bincode::deserialize(&tls_client.read_plaintext).unwrap();
    tls_client.read_plaintext.clear();

    // TODO: fix this garbage message checking
    if response.msgtype == MessageType::ServerReturn
        && response.instruction == 1
        && response.argument_count == 1
        && response.data.len() != 0
    {
        /* returned data*/
        let transactions: Vec<Transaction> = bincode::deserialize(&response.data).unwrap();
        return Ok(transactions);
    } else {
        /* could not get data */
        return Err(ReturnFlags::CLIENT_ACC_RETRIEVE_TRANSACTION_ERROR); // TODO: return server returned error code too
    }
}
