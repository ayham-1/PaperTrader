use crate::common::message::message::Message;

use crate::server::network::tls_connection::TlsConnection;

pub fn purchase_asset(_tls_connection: &mut TlsConnection, _message: &Message) {
    /* assert recieved message */
    // TODO: INVALID MESSAGE CHECKING SYSTEM
    //if message.msgtype != MessageType::Command || message.argument_count != 4
    //    || message.data_message_number != 0 || message.data_message_max != 0
    //        || message.data.len() == 0 {
    //            warn!("PURCHASE_ASSET_INVALID_MESSAGE");
    //            tls_connection.closing = true;
    //            return;
    //        }

    //* parse request data */
    //let stringified_data = std::str::from_utf8(&message.data).unwrap();
    //let data = json::parse(&stringified_data).unwrap();
    //* get symbol, shares amount, price, buy or sell */
    //let symbol = data["symbol"].as_str().unwrap();
    //let shares_amount = data["shares_amount"].as_str().unwrap().to_string().parse::<i64>().unwrap();
    //let is_buy = data["is_buy"].as_bool().unwrap();

    //* connect to the data base */
    //let mut client = db_connect(DB_USER, DB_PASS).unwrap();

    //* check if the symbol exists */
    //let company = get_company_from_db(symbol);
    //if !company.is_ok() {
    //    let msg = message_builder(MessageType::ServerReturn, 0, 1, 0, 0, 
    //                              bincode::serialize("COULDNOT FIND COMPANY")).unwrap();
    //    return;
    //}

    //*  */
}
