use std::io::Write;

use crate::common::account::transaction::Transaction;
use crate::common::message::message::Message;
use crate::common::message::message_type::MessageType;
use crate::common::message::inst::DataTransferInst;
use crate::common::message::message_builder::message_builder;

use crate::server::network::tls_connection::TlsConnection;
use crate::server::network::jwt_wrapper::verify_jwt_token;
use crate::server::db::config::{DB_ACC_USER, DB_ACC_PASS};
use crate::server::db::initializer::db_connect;

pub fn acc_retrieve_transaction(tls_connection: &mut TlsConnection, message: &Message) -> Result<(), String> {
    /* verify JWT token */
    let token = match verify_jwt_token(bincode::deserialize(&message.data).unwrap()) {
        Ok(token) => token,
        Err(_) => {
            warn!("ACC_RETRIEVE_TRANSACTION_UNAUTH_TOKEN");
            tls_connection.closing = true;
            return Err("ACC_RETRIEVE_TRANSACTION_REJECTED".to_string());
        }
    };

    /* connect to database */
    let mut client = db_connect(DB_ACC_USER, DB_ACC_PASS)?;

    /* get userId's transactions */
    let mut transactions: Vec<Transaction> = Vec::new();
    for row in client.query("SELECT * FROM accounts_schema.transactions WHERE user_id = $1",
    &[&token.user_id]).unwrap() {
        let mut transaction = Transaction::default();
        transaction.stock_symbol = row.get(2);
        transaction.shares_size = row.get(3);
        transaction.shares_cost = row.get(4);
        transaction.is_buy = row.get(5);

        transactions.push(transaction);
    }

    /* build message to be send */
    match message_builder(MessageType::ServerReturn, 1, 1, 0, 0, bincode::serialize(&transactions).unwrap()) {
        Ok(message) => {
            match tls_connection.tls_session.write(&bincode::serialize(&message).unwrap()) {
                Ok(_) => tls_connection.do_tls_write_and_handle_error(),
                Err(err) => return Err(format!("ACC_RETRIEVE_TRANSACTION_FAILED_SENDING_MESSAGE: {}", err)),
            }
        },
        Err(_) => return Err("ACC_RETRIEVE_TRANSACTION_MESSAGE_BUILD_FAILED".to_string())
    }

    Ok(())
}
