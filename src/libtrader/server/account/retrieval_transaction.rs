use crate::common::account::transaction::Transaction;
use crate::common::message::message::Message;
use crate::common::message::message_builder::message_builder;
use crate::common::message::message_type::MessageType;
use crate::common::misc::return_flags::ReturnFlags;

use crate::server::network::jwt_wrapper::verify_jwt_token;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream;

pub async fn acc_retrieve_transaction(
    sql_conn: &tokio_postgres::Client,
    socket: &mut TlsStream<TcpStream>,
    message: &Message,
) -> Result<(), ReturnFlags> {
    /* verify JWT token */
    let token = match verify_jwt_token(bincode::deserialize(&message.data).unwrap()) {
        Ok(token) => token,
        Err(_) => {
            warn!("ACC_RETRIEVE_TRANSACTION_UNAUTH_TOKEN");
            socket.shutdown().await.unwrap();
            return Err(ReturnFlags::ServerAccUnauthorized);
        }
    };

    /* get userId's transactions */
    let mut transactions: Vec<Transaction> = Vec::new();
    for row in sql_conn
        .query(
            "SELECT * FROM accounts_schema.transactions WHERE user_id = $1",
            &[&token.user_id],
        )
        .await
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
    socket
        .write_all(&bincode::serialize(&message).unwrap())
        .await
        .map_err(|_| ReturnFlags::ServerRetrieveTransactionFailed)?;

    Ok(())
}
