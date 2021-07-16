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
/// Should be used in contexts that return ```io::Result```.
/// Should be used in Async context.
///
/// Arguments:
/// socket - TLS socket to use.
/// auth_jwt - JWT token to authenticate with.
///
/// Returns: ```io::Result``` wraps ```Portfolio```.
///
/// Example:
/// ```rust
///     let mut portfolio = acc_retrieve_portfolio(&mut tls_client, &mut poll)?;
/// ```
pub async fn acc_retrieve_portfolio(
    socket: &mut TlsStream<TcpStream>,
    auth_jwt: String,
) -> io::Result<Portfolio> {
    if auth_jwt.is_empty() == true {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "ACC_RETRIEVE_PORTFOLIO: JWT TOKEN EMPTY",
        ));
    }

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
