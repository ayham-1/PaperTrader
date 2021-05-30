use crate::common::message::inst::CommandInst;
use crate::common::message::message::Message;
use crate::common::message::message_builder::message_builder;
use crate::common::message::message_type::MessageType;
use crate::common::misc::assert_msg::assert_msg;

use crate::server::account::authorization::acc_auth;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream;

pub async fn login_normal(
    sql_conn: &tokio_postgres::Client,
    tls_connection: &mut TlsStream<TcpStream>,
    message: &Message,
) -> std::io::Result<()> {
    /* assert recieved message */
    if !assert_msg(
        message,
        MessageType::Command,
        true,
        3,
        false,
        0,
        false,
        0,
        false,
        0,
    ) && message.instruction == CommandInst::LoginMethod1 as i64
        && message.data.len() != 0
    {
        warn!("LOGIN_INVALID_MESSAGE");
        return tls_connection.shutdown().await;
    }

    /* call acc_auth() server version */
    match acc_auth(sql_conn, tls_connection, message).await {
        Ok(_) => Ok(()),
        Err(err) => {
            let server_response = message_builder(
                MessageType::ServerReturn,
                0,
                0,
                0,
                0,
                bincode::serialize(&err).unwrap(),
            );
            tls_connection
                .write_all(&bincode::serialize(&server_response).unwrap())
                .await
        }
    }
}
