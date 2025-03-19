use sea_orm::prelude::*;
use sea_orm::entity::prelude::*;
use sea_orm::DeriveActiveEnum;  
use strum::{EnumString, Display};
use strum_macros::{EnumIter};

#[derive(Debug, Clone, EnumString, Display, EnumIter, DeriveActiveEnum, PartialEq)]
#[sea_orm(rs_type = "String", db_type = "String")]  // ✅ Fix: Proper db_type
pub enum TradeTypeEnum { 
    #[sea_orm(string_value = "open")]
    Open,
    #[sea_orm(string_value = "closed")]
    Closed,
}

#[derive(Debug, Clone, EnumString, Display, EnumIter, DeriveActiveEnum, PartialEq)]
#[sea_orm(rs_type = "String", db_type = "String")]  // ✅ Fixed db_type
pub enum State { 
    #[sea_orm(string_value = "active")]
    Active, 
    #[sea_orm(string_value = "deleted")]
    Deleted,
}

#[derive(Debug, Clone, EnumString, Display)]
pub enum ActionParameter { 
    TransferPrize, 
    Swap, 
    Shilling,
}

#[derive(Debug, Clone, EnumString, Display)]
pub enum ConversationStatus { 
    Approve, 
    ApproveFailed, 
    Decline, 
    Reject, 
    Discuss, 
    ReadyToShilling,
}
