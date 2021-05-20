use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct ReturnFlags: u64 {
        const LIBTRADER_INIT_CLIENT_CONNECT = 1;
        const LIBTRADER_INIT_LOG_FAILED = 2;
        const LIBTRADER_INIT_FAILED = 3;
        const COMMON_GEN_LOG_DIR_CREATION_FAILED = 4;
        const COMMON_TLS_BAD_CONFIG = 5;

        const COMMON_GET_COMPANY_FAILED = 6;
        const COMMON_GET_STOCK_FAILED = 7;

        const SERVER_DB_CONNECT_FAILED = 8;
        const SERVER_DB_WRITE_FAILED = 9;

        const SERVER_DB_USER_HASH_NOT_FOUND = 10;
        const SERVER_DB_USER_SALT_NOT_FOUND = 11;

        const SERVER_DB_CREATE_TRANSACTION_FAILED = 12;
        const SERVER_DB_CREATE_POSITION_FAILED = 13;
        const SERVER_DB_CREATE_STOCK_FAILED = 14;
        const SERVER_DB_CREATE_COMPANY_FAILED = 15;

        const SERVER_DB_SEARCH_STOCK_NOT_FOUND = 16;
        const SERVER_DB_SEARCH_COMPANY_NOT_FOUND = 17;

        const SERVER_REGISTER_INV_MSG = 18;
        const SERVER_LOGIN_INV_MSG = 19;

        const SERVER_PURCHASE_ASSET_INV_MSG = 20;

        const SERVER_ACC_UNAUTHORIZED = 21;
        const SERVER_ACC_USER_EXISTS = 22;

        const SERVER_GET_ASSET_DATA_INV_MSG = 23;
        const SERVER_GET_ASSET_INFO_INV_MSG = 24;

        const SERVER_GET_USER_ID_NOT_FOUND = 25;

        const SERVER_RETRIEVE_TRANSACTION_FAILED = 26;
        const SERVER_RETRIEVE_TRANSACTION_INV_MSG = 27;
        const SERVER_RETRIEVE_PORTFOLIO_FAILED = 28;
        const SERVER_RETRIEVE_PORTFOLIO_INV_MSG = 29;

        const SERVER_CREATE_JWT_TOKEN_FAILED = 30;

        const SERVER_TLS_CONN_WRITE_FAILED = 31;
        const SERVER_TLS_CONN_PROCESS_FAILED = 32;
        const SERVER_TLS_CONN_READ_PLAIN_FAILED = 33;
        const SERVER_TLS_SERVER_ACCEPT_FAILED = 34;

        const SERVER_HANDLE_DATA_RCVD_INV_MSG = 35;

        const CLIENT_ACC_RETRIEVE_PORTFOLIO_ERROR = 36;
        const CLIENT_ACC_RETRIEVE_TRANSACTION_ERROR = 37;
        const CLIENT_ACC_CREATION_FAILED = 38;
        const CLIENT_ACC_INVALID_SESSION_ID = 39;
        const CLIENT_ACC_UNAUTHORIZED = 40;

        const CLIENT_REQ_SALT_FAILED = 41;
        const CLIENT_REQ_SALT_INV_MSG = 42;
        const CLIENT_REQ_SALT_INV_MSG_RET_SIZE = 43;
        const CLIENT_REQ_SALT_INV_MSG_INST = 44;
        const CLIENT_REQ_SALT_REJ = 45;
        const CLIENT_GEN_SALT_FAILED = 46;

        const CLIENT_TLS_READ_ERROR = 47;
        const CLIENT_WAIT_AND_READ_BRANCHED = 48;
    }
}
impl std::fmt::Display for ReturnFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
