use std::io::Write;

use crate::common::message::message::Message;
use crate::common::message::message_type::MessageType;
use crate::common::message::message_builder::message_builder;
use crate::common::message::inst::DataTransferInst;

use crate::server::network::tls_connection::TlsConnection;
use crate::server::db::cmd::get_company::get_company_from_db;

pub fn get_asset_info(tls_connection: &mut TlsConnection, message: &Message) {
    /* assert recieved message */
    if message.msgtype != MessageType::DataTransfer || message.argument_count != 1 
        || message.data_message_number != 0 || message.data_message_max != 0
            || message.data.len() == 0 {
                warn!("GET_ASSET_INFO_INVALID_MESSAGE");
                tls_connection.closing = true;
                return;
            }

    /* call get_company_from_db() */
    match get_company_from_db(bincode::deserialize(&message.data).unwrap()) {
        Ok(company) => {
            match message_builder(MessageType::ServerReturn, DataTransferInst::GetAssetInfo as i64, 0, 0, 1, 
                                  bincode::serialize(&company).unwrap()) {
                Ok(msg) => {
                    let _ = tls_connection.write(&bincode::serialize(&msg).unwrap());
                },
                Err(_) => {
                    error!("GET_ASSET_INFO_FAILED_MESSAGE_BUILD");
                },
            }
        },
        Err(err) => {
            match message_builder(MessageType::ServerReturn, 0, 0, 0, 0, bincode::serialize(&err).unwrap()) {
                Ok(msg) => {
                    let _ = tls_connection.write(&bincode::serialize(&msg).unwrap());
                },
                Err(_) => {
                    error!("GET_ASSET_INFO_FAILED_MESSAGE_BUILD");
                },
            }
        }
    }
}
