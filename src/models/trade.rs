use sea_orm::prelude::*;
use chrono::NaiveDateTime;
use strum::{Display, EnumString, EnumIter}; // ✅ Ensuring required traits are imported
use sea_orm::sea_query::Expr;

#[derive(Debug, Clone, EnumString, Display, EnumIter, DeriveActiveEnum, PartialEq)] 
#[sea_orm(rs_type = "String", db_type = "String")]
pub enum TradeTypeEnum { 
    #[sea_orm(string_value = "Open")]
    Open,
    #[sea_orm(string_value = "Clone")]
    Clone,
}

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "trades")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,

    #[sea_orm(default_expr = "Expr::current_timestamp()")] 
    pub created_at: NaiveDateTime, 

    pub chat_uuid: String, 
    pub base_token_quantity: f64, 
    pub quote_token_quantity: f64, 

    #[sea_orm(default_value = "Open")] 
    pub trade_type: TradeTypeEnum, 

    pub tx_id: String, 
    pub profit_loss: Option<bool>,
    pub token_id: i32,
    pub payment_id: i32, 
    pub trade_position_id: i32, 
    pub fee_rate: Option<f64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::models::credit::Entity", 
        from = "Column::PaymentId",
        to = "crate::models::credit::Column::Id"
    )]
    Payment,

    #[sea_orm(
        belongs_to = "crate::models::user::Entity",
        from = "Column::ChatUuid",
        to = "crate::models::user::Column::Id"
    )]
    User,
}

impl ActiveModelBehavior for ActiveModel {}
