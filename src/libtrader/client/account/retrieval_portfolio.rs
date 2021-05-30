use std::io;

use crate::common::account::portfolio::Portfolio;
use crate::common::message::inst::DataTransferInst;
use crate::common::message::message::Message;
use crate::common::message::message_builder::message_builder;
use crate::common::message::message_type::MessageType;
use crate::common::misc::assert_msg::assert_msg;
use crate::common::misc::return_flags::ReturnFlags;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_rustls::client::TlsStream;

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
pub async fn acc_retrieve_portfolio(
    socket: &mut TlsStream<TcpStream>,
    auth_jwt: String,
) -> io::Result<Portfolio> {
    // TODO: yea absolutely, let's crash the thread
    assert_eq!(auth_jwt.is_empty(), false);

    /* build message request */
    let message = message_builder(
        MessageType::Command,
        DataTransferInst::GetUserPortfolio as i64,
        1,
        0,
        0,
        bincode::serialize(&auth_jwt).unwrap(),
    );
    socket
        .write_all(&bincode::serialize(&message).unwrap())
        .await?;

    /* decode response */
    let mut buf = Vec::with_capacity(4096);
    socket.read_buf(&mut buf).await?;

    let response: Message = bincode::deserialize(&buf).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{}", ReturnFlags::ClientAccRetrievePortfolioError),
        )
    })?;

    if assert_msg(
        &response,
        MessageType::DataTransfer,
        true,
        1,
        false,
        0,
        false,
        0,
        false,
        0,
    ) && response.instruction == 1
        && response.data.len() != 0
    {
        /* returned data */
        let portfolio: Portfolio = bincode::deserialize(&response.data).unwrap();
        return Ok(portfolio);
    } else {
        /* could not get data */
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{}", ReturnFlags::ClientAccRetrievePortfolioError),
        ));
    }
}
