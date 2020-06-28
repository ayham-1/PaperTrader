static INST_SWITCH_STATE: isize = 0;

#[derive(PartialEq, Debug)]
pub enum CommandInst {
    LoginMethod1 = 1,
    LoginMethod2 = 2,
    Register = 3,
    PurchaseAsset = 4,
    SellAsset = 5
}
static INST_COMMAND_MAX_ID: isize = CommandInst::SellAsset as isize;

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
static INST_DATA_MAX_ID: isize = DataTransferInst::GetUserTransactionHist as isize;
