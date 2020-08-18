#[allow(dead_code)]
static INST_SWITCH_STATE: isize = 0;

#[derive(PartialEq, Debug)]
pub enum CommandInst {
    LoginMethod1 = 1,
    LoginMethod2 = 2,
    Register = 3,
    PurchaseAsset = 4,
    SellAsset = 5,
    GenHashSalt = 6,
    GetEmailSalt = 7,
    GetPasswordSalt = 8,
}
impl std::fmt::Display for CommandInst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[allow(dead_code)]
static INST_COMMAND_MAX_ID: isize = CommandInst::GetPasswordSalt as isize;

#[derive(PartialEq, Debug)]
pub enum DataTransferInst {
    GetAssetInfo = 6,
    GetAssetValue = 7,
    GetAssetValueCurrent = 8,
    GetUserInfo = 9,
    GetUserPortfolio = 10,
    GetUserTransactionHist = 11,
}
impl std::fmt::Display for DataTransferInst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[allow(dead_code)]
static INST_DATA_MAX_ID: isize = DataTransferInst::GetUserTransactionHist as isize;
