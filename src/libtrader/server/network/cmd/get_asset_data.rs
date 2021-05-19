use std::io::Write;

use crate::common::message::message::Message;
use crate::common::message::message_builder::message_builder;
use crate::common::message::message_type::MessageType;
use crate::common::misc::assert_msg::assert_msg;

use crate::server::db::cmd::get_stock::get_stock_from_db_between_epochs;
use crate::server::network::tls_connection::TlsConnection;

pub fn get_asset_data(tls_connection: &mut TlsConnection, message: &Message) {
    /* assert recieved message */
    if assert_msg(message, MessageType::DataTransfer, 3, 0, 0, 0) {
        tls_connection.closing = true;
        warn!("GET_ASSET_DATA_MSG_ASSERT_FAILED");
        return;
    }

    /*
     * Parse arguments
     * */
    /* get json data */
    let stringified_data = std::str::from_utf8(&message.data).unwrap();
    let data = json::parse(&stringified_data).unwrap();
    /* get symbol, start_epoch, and end_epoch */
    let symbol = data["symbol"].as_str().unwrap();
    let start_epoch = data["start_epoch"]
        .as_str()
        .unwrap()
        .to_string()
        .parse::<i64>()
        .unwrap();
    let end_epoch = data["end_epoch"]
        .as_str()
        .unwrap()
        .to_string()
        .parse::<i64>()
        .unwrap();

    /* call get_stock_from_db_between_epochs() */
    match get_stock_from_db_between_epochs(symbol, start_epoch, end_epoch) {
        Ok(vals) => {
            /* send the data */
            let mut counter = 0;
            for val in &vals {
                let message = message_builder(
                    MessageType::DataTransfer,
                    1,
                    counter,
                    vals.len(),
                    0,
                    bincode::serialize(&val).unwrap(),
                );
                let _ = tls_connection.write(&bincode::serialize(&message).unwrap());
                counter = counter + 1;
            }
        }
        Err(err) => {
            /* handle error */
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
