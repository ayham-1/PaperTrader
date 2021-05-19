use std::io::Write;

use crate::common::message::inst::DataTransferInst;
use crate::common::message::message::Message;
use crate::common::message::message_builder::message_builder;
use crate::common::message::message_type::MessageType;
use crate::common::misc::assert_msg::assert_msg;

use crate::server::db::cmd::get_company::get_company_from_db;
use crate::server::network::tls_connection::TlsConnection;

pub fn get_asset_info(tls_connection: &mut TlsConnection, message: &Message) {
    /* assert recieved message */
    if assert_msg(message, MessageType::DataTransfer, 1, 0, 0, 0) {
        tls_connection.closing = true;
        warn!("GET_ASSET_INFO_INVALID_MESSAGE");
        return;
    }

    /* call get_company_from_db() */
    match get_company_from_db(bincode::deserialize(&message.data).unwrap()) {
        Ok(company) => {
            let message = message_builder(
                MessageType::ServerReturn,
                DataTransferInst::GetAssetInfo as i64,
                0,
                0,
                1,
                bincode::serialize(&company).unwrap(),
            );
            let _ = tls_connection.write(&bincode::serialize(&message).unwrap());
        }
        Err(err) => {
            let message = message_builder(
                MessageType::ServerReturn,
                0,
                0,
                0,
                0,
                bincode::serialize(&err).unwrap(),
            );
            let _ = tls_connection.write(&bincode::serialize(&message).unwrap());
        }
    }
}
