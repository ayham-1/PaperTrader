use data_encoding::HEXUPPER;

use crate::common::message::inst::{CommandInst, DataTransferInst};
use crate::common::message::message::Message;
use crate::common::message::message_builder::message_builder;
use crate::common::message::message_type::MessageType;

use crate::server::network::cmd::login_normal::login_normal;
use crate::server::network::cmd::register::register;
use crate::server::network::cmd::retrieve_portfolio::retrieve_portfolio;
use crate::server::network::cmd::retrieve_transactions::retrieve_transactions;

//use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream;

pub async fn handle_data(socket: &mut TlsStream<TcpStream>, buf: &[u8]) -> std::io::Result<()> {
    /* decode incoming message */
    let client_msg: Message = bincode::deserialize(&buf)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidInput,
                                            format!("HANDLE_DATA_RCVD_INVALID_MSG: {}", err)))?;
    println!("This is a message: {}", client_msg);

    /* handle individual client instructions */
    match client_msg.instruction {
        _ if client_msg.instruction == CommandInst::GenHashSalt as i64 => {
            use ring::rand::SecureRandom;
            use ring::{digest, rand};
            let rng = rand::SystemRandom::new();
            let mut salt = [0u8; digest::SHA512_OUTPUT_LEN / 2];
            rng.fill(&mut salt).unwrap();

            let server_response: Message = message_builder(
                MessageType::DataTransfer,
                CommandInst::GenHashSalt as i64,
                1,
                0,
                1,
                salt.to_vec(),
            );
            socket
                .write_all(bincode::serialize(&server_response).unwrap().as_slice())
                .await
        }
        _ if client_msg.instruction == CommandInst::GetEmailSalt as i64 => {
            use crate::server::db::cmd::get_user_salt::get_user_salt;
            match get_user_salt(
                String::from_utf8(client_msg.data).unwrap().as_str(),
                true,
                false,
            ) {
                Ok(salt) => {
                    let server_response: Message = message_builder(
                        MessageType::DataTransfer,
                        CommandInst::GetEmailSalt as i64,
                        1,
                        0,
                        1,
                        HEXUPPER.decode(salt.as_bytes()).unwrap(),
                    );
                    socket
                        .write_all(bincode::serialize(&server_response).unwrap().as_slice())
                        .await
                }
                Err(_) => {
                    let server_response =
                        message_builder(MessageType::ServerReturn, 0, 0, 0, 0, Vec::new());
                    socket
                        .write_all(bincode::serialize(&server_response).unwrap().as_slice())
                        .await
                }
            }
        }
        _ if client_msg.instruction == CommandInst::GetPasswordSalt as i64 => {
            use crate::server::db::cmd::get_user_salt::get_user_salt;
            match get_user_salt(
                String::from_utf8(client_msg.data).unwrap().as_str(),
                false,
                false,
            ) {
                Ok(salt) => {
                    let server_response: Message = message_builder(
                        MessageType::DataTransfer,
                        CommandInst::GetPasswordSalt as i64,
                        1,
                        0,
                        1,
                        HEXUPPER.decode(salt.as_bytes()).unwrap(),
                    );
                    socket
                        .write_all(bincode::serialize(&server_response).unwrap().as_slice())
                        .await
                }
                Err(_) => {
                    let server_response =
                        message_builder(MessageType::ServerReturn, 0, 0, 0, 0, Vec::new());

                    socket
                        .write_all(bincode::serialize(&server_response).unwrap().as_slice())
                        .await
                }
            }
        }
        _ if client_msg.instruction == CommandInst::Register as i64 => {
            register(socket, &client_msg).await
        }
        _ if client_msg.instruction == CommandInst::LoginMethod1 as i64 => {
            login_normal(socket, &client_msg).await
        }
        _ if client_msg.instruction == DataTransferInst::GetUserPortfolio as i64 => {
            retrieve_portfolio(socket, &client_msg).await
        }
        _ if client_msg.instruction == DataTransferInst::GetUserTransactionHist as i64 => {
            retrieve_transactions(socket, &client_msg).await
        }
        _ => Ok(()),
    }
}
