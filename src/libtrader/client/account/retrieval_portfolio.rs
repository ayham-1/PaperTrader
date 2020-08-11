use std::io::Write;

use crate::common::account::portfolio::Portfolio;
use crate::common::message::message::Message;
use crate::common::message::message_type::MessageType;
use crate::common::message::inst::DataTransferInst;
use crate::common::message::message_builder::message_builder;

use crate::client::network::tls_client::TlsClient;
use crate::client::network::cmd::wait_and_read_branched::wait_and_read_branched;

/// Retrieves from the connected TLS server an authorized portfolio.
///
/// Sends a request for portfolio with the JWT token of the client connection. Handles any response
/// and returns.
///
/// Arguments:
/// tls_client - TLS client to use containing the JWT token to authorize.
/// poll - For event handling.
///
/// Returns: portfolio on success, string on error containing reason of failure.
///
/// Example:
/// ```rust
///     match acc_retrieve_portfolio(&mut tls_client, &mut poll) {
///         Ok(portfolio) => {/* interesting stuff with portfolio */},
///         Err(err) => panic!("can not retrieve portfolio! error: {}", err)
///     };
/// ```
pub fn acc_retrieve_portfolio(tls_client: &mut TlsClient, poll: &mut mio::Poll) -> Result<Portfolio, String> {
    assert_eq!(tls_client.auth_jwt.is_empty(), false);

    /* build message request */
    match message_builder(MessageType::DataTransfer, DataTransferInst::GetUserPortfolio as i64, 1, 0, 0, 
                          bincode::serialize(&tls_client.auth_jwt).unwrap()) {
        Ok(message) => {
            tls_client.write(bincode::serialize(&message).unwrap().as_slice()).unwrap();
        },
        Err(_) => return Err("ACC_RETRIEVE_PORTFOLIO_FAILED_MESSAGE_BUILD".to_string()),
    };

    /* wait for response */
    wait_and_read_branched(tls_client, poll, Some(20), Some(500))?;

    /* decode response */
    let response: Message = bincode::deserialize(&tls_client.read_plaintext).unwrap();
    tls_client.read_plaintext.clear();

    if response.msgtype == MessageType::ServerReturn && response.instruction == 1 
        && response.argument_count == 1 && response.data.len() != 0 {
            /* returned data */
            let portfolio: Portfolio = bincode::deserialize(&response.data).unwrap();
            return Ok(portfolio);
        } else {
            /* could not get data */
            return Err("ACC_RETRIEVE_PORTFOLIO_UNAUTHORIZED".to_string());
        }
}
