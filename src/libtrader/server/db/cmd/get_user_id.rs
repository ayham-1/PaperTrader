use crate::server::db::cmd::user_exists::user_exists;

use crate::common::misc::return_flags::ReturnFlags;

pub async fn get_user_id(sql_conn: &tokio_postgres::Client, username: &str) -> Result<i64, ReturnFlags> {
    /* check that user exists */
    if user_exists(sql_conn, username).await {
        for row in sql_conn
            .query(
                "SELECT id,username FROM accounts_schema.accounts WHERE username LIKE $1",
                &[&username],
            )
            .await
            .unwrap()
        {
            return Ok(row.get(0));
        }
    }
    Err(ReturnFlags::ServerGetUserIdNotFound)
}
