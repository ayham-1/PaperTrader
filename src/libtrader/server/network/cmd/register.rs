use crate::common::message::inst::CommandInst;
use crate::common::message::message::Message;
use crate::common::message::message_builder::message_builder;
use crate::common::message::message_type::MessageType;
use crate::common::misc::assert_msg::assert_msg;

use crate::server::account::creation::acc_create;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream;

pub async fn register(
    sql_conn: &tokio_postgres::Client,
    tls_connection: &mut TlsStream<TcpStream>,
    message: &Message,
) -> std::io::Result<()> {
    println!("hello");
    /* assert recieved message */
    if !assert_msg(
        message,
        MessageType::Command,
        true,
        5,
        false,
        0,
        false,
        0,
        false,
        0,
    ) && message.instruction == CommandInst::Register as i64
        && message.data.len() != 0
    {
        warn!("REGISTER_INVALID_MESSAGE");
        return tls_connection.shutdown().await;
    }

    /* call acc_create() server version */
    match acc_create(sql_conn, message).await {
        Ok(_) => {
            let server_response =
                message_builder(MessageType::ServerReturn, 1, 0, 0, 0, Vec::new());
            tls_connection
                .write_all(&bincode::serialize(&server_response).unwrap())
                .await
        }
        Err(err) => {
            warn!("REGISTER_FAILED: {}", err);
            Ok(())
        }
    }
}
