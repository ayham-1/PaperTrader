use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum ReturnFlags {
    LibtraderInitClientConnect = 1,
    LibtraderInitLogFailed = 2,
    LibtraderInitFailed = 3,
    CommonGenLogDirCreationFailed = 4,
    CommonTlsBadConfig = 5,

    CommonGetCompanyFailed = 6,
    CommonGetStockFailed = 7,

    ServerDbConnectFailed = 8,
    ServerDbWriteFailed = 9,

    ServerDbUserHashNotFound = 10,
    ServerDbUserSaltNotFound = 11,

    ServerDbCreateTransactionFailed = 12,
    ServerDbCreatePositionFailed = 13,
    ServerDbCreateStockFailed = 14,
    ServerDbCreateCompanyFailed = 15,

    ServerDbSearchStockNotFound = 16,
    ServerDbSearchCompanyNotFound = 17,

    ServerRegisterInvMsg = 18,
    ServerLoginInvMsg = 19,

    ServerPurchaseAssetInvMsg = 20,

    ServerAccUnauthorized = 21,
    ServerAccUserExists = 22,

    ServerGetAssetDataInvMsg = 23,
    ServerGetAssetInfoInvMsg = 24,

    ServerGetUserIdNotFound = 25,

    ServerRetrieveTransactionFailed = 26,
    ServerRetrieveTransactionInvMsg = 27,
    ServerRetrievePortfolioFailed = 28,
    ServerRetrievePortfolioInvMsg = 29,

    ServerCreateJwtTokenFailed = 30,

    ServerTlsConnWriteFailed = 31,
    ServerTlsConnProcessFailed = 32,
    ServerTlsConnReadPlainFailed = 33,
    ServerTlsServerAcceptFailed = 34,

    ServerHandleDataRcvdInvMsg = 35,

    ClientAccRetrievePortfolioError = 36,
    ClientAccRetrieveTransactionError = 37,
    ClientAccCreationFailed = 38,
    ClientAccInvalidSessionId = 39,
    ClientAccUnauthorized = 40,

    ClientReqSaltFailed = 41,
    ClientReqSaltInvMsg = 42,
    ClientReqSaltInvMsgRetSize = 43,
    ClientReqSaltInvMsgInst = 44,
    ClientReqSaltRej = 45,
    ClientGenSaltFailed = 46,

    ClientTlsReadError = 47,
    ClientWaitAndReadBranched = 48,
}
impl std::fmt::Display for ReturnFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
