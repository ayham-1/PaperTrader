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
}
impl std::fmt::Display for CommandInst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[allow(dead_code)]
static INST_COMMAND_MAX_ID: isize = CommandInst::GenHashSalt as isize;

#[derive(PartialEq, Debug)]
pub enum DataTransferInst {
    GetAssetInfo = 6,
    GetAssetValueCurrent = 7,
    GetAssetValueDay = 8,
    GetAssetValueWeek = 9,
    GetAssetValueMonth = 10,
    GetAssetValueYear = 11,
    GetAssetValueAllTime = 12,
    GetUserInfo = 13,
    GetUserPortfolio = 14,
    GetUserTransactionHist = 15,
}
impl std::fmt::Display for DataTransferInst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

#[allow(dead_code)]
static INST_DATA_MAX_ID: isize = DataTransferInst::GetUserTransactionHist as isize;
