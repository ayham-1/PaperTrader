pub async fn user_exists(sql_conn: &tokio_postgres::Client, username: &str) -> bool {
    for _ in &sql_conn.query(
        "SELECT username FROM accounts_schema.accounts WHERE username LIKE $1",
        &[&username],
    ).await {
        return true;
    }
    false
}
