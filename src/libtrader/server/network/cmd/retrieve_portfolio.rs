use log::warn;
use std::io;

use crate::common::message::inst::DataTransferInst;
use crate::common::message::message::Message;
use crate::common::message::message_type::MessageType;
use crate::common::misc::assert_msg::assert_msg;
use crate::common::misc::return_flags::ReturnFlags;

use crate::server::account::retrieval_portfolio::acc_retrieve_portfolio;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream;

pub async fn retrieve_portfolio(
    tls_connection: &mut TlsStream<TcpStream>,
    message: &Message,
) -> std::io::Result<()> {
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
    ) && message.instruction == DataTransferInst::GetUserPortfolio as i64
        && message.data.len() != 0
    {
        warn!("RETRIEVE_PORTFOLIO_INVALID_MESSAGE");
        return tls_connection.shutdown().await;
    }

    /* call acc_retrieve_portfolio() server version */
    match acc_retrieve_portfolio(tls_connection, message).await {
        Ok(_) => Ok(()),
        Err(err) => {
            warn!("RETRIEVE_PORTFOLIO_FAILED: {}", err);
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("{}: {}", ReturnFlags::ServerRetrievePortfolioFailed, err),
            ))
        }
    }
}
