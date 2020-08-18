use std::io::Write;

use crate::common::message::message::Message;
use crate::common::message::message_type::MessageType;
use crate::common::message::message_builder::message_builder;

use crate::server::network::tls_connection::TlsConnection;
use crate::server::db::cmd::get_stock::get_stock_from_db_between_epochs;

pub fn get_asset_data(tls_connection: &mut TlsConnection, message: &Message) {
    /* assert recieved message */
    if message.msgtype != MessageType::DataTransfer || message.argument_count != 3
        || message.data_message_number != 0 || message.data_message_max != 0
            || message.data.len() == 0 {
                warn!("GET_ASSET_DATA_INVALID_MESSAGE");
                tls_connection.closing = true;
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
    let start_epoch = data["start_epoch"].as_str().unwrap().to_string().parse::<i64>().unwrap();
    let end_epoch = data["end_epoch"].as_str().unwrap().to_string().parse::<i64>().unwrap();

    /* call get_stock_from_db_between_epochs() */
    match get_stock_from_db_between_epochs(symbol, start_epoch, end_epoch) {
        Ok(vals) => { /* send the data */
            let mut counter = 0;
            for val in &vals {
                match message_builder(MessageType::DataTransfer, 1, counter, vals.len(), 0, 
                                      bincode::serialize(&val).unwrap()) {
                    Ok(msg) => {
                        tls_connection.tls_session.write(&bincode::serialize(&msg).unwrap()).unwrap();
                        tls_connection.do_tls_write_and_handle_error();
                    },
                    Err(_) => {
                        error!("GET_ASSET_DATA_FAILED_MESSAGE_BUILD");
                    },
                }
                counter = counter + 1;
            }
        },
        Err(err) => { /* handle error */
            match message_builder(MessageType::ServerReturn, 0, 0, 0, 0, bincode::serialize(&err).unwrap()) {
                Ok(msg) => {
                    tls_connection.tls_session.write(&bincode::serialize(&msg).unwrap()).unwrap();
                    tls_connection.do_tls_write_and_handle_error();
                },
                Err(_) => {
                    error!("GET_ASSET_DATA_FAILED_MESSAGE_BUILD");
                },
            }
        }
    }
}
