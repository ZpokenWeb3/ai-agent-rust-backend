use sea_orm::prelude::*;
use std::fmt;
use strum_macros::{EnumString, Display};

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name= "base_models")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32, 
}

impl fmt::Display for Model { 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write(f, "BaseModel [{}}", self.id)
    }
}

#[derive(Debug, Clone, EnumString, Display)]
pub enum State { 
    Active, 
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