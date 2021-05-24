use crate::common::account::transaction::Transaction;
use crate::common::message::message::Message;
use crate::common::message::message_builder::message_builder;
use crate::common::message::message_type::MessageType;
use crate::common::misc::return_flags::ReturnFlags;

use crate::server::db::config::{DB_ACC_PASS, DB_ACC_USER};
use crate::server::db::initializer::db_connect;
use crate::server::network::jwt_wrapper::verify_jwt_token;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream;

pub async fn acc_retrieve_transaction(
    tls_connection: &mut TlsStream<TcpStream>,
    message: &Message,
) -> Result<(), ReturnFlags> {
    /* verify JWT token */
    let token = match verify_jwt_token(bincode::deserialize(&message.data).unwrap()) {
        Ok(token) => token,
        Err(_) => {
            warn!("ACC_RETRIEVE_TRANSACTION_UNAUTH_TOKEN");
            tls_connection.shutdown().await.unwrap();
            return Err(ReturnFlags::ServerAccUnauthorized);
        }
    };

    /* connect to database */
    let mut client = db_connect(DB_ACC_USER, DB_ACC_PASS)?;

    /* get userId's transactions */
    let mut transactions: Vec<Transaction> = Vec::new();
    for row in client
        .query(
            "SELECT * FROM accounts_schema.transactions WHERE user_id = $1",
            &[&token.user_id],
        )
        .unwrap()
    {
        let mut transaction = Transaction::default();
        transaction.stock_symbol = row.get(2);
        transaction.shares_size = row.get(3);
        transaction.shares_cost = row.get(4);
        transaction.is_buy = row.get(5);

        transactions.push(transaction);
    }

    /* build message to be send */
    let message = message_builder(
        MessageType::ServerReturn,
        1,
        1,
        0,
        0,
        bincode::serialize(&transactions).unwrap(),
    );
    let _ = tls_connection.write(&bincode::serialize(&message).unwrap());

    Ok(())
}
