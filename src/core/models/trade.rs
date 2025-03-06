use sea_orm::prelude::*;
use chronp::NaiveDateTime;
use crate::models::base::State;

#[derive(Debug, Clone, EnumString, Display)]
pub enum TradeTypeEnum { 
    Open,
    Clone,
}

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "trades")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    #[sea_orm(default_expr = "NOW()")]
    pub created_at: NaiveDateTime, 
    pub chat_uuid: String, 
    pub base_token_quantity: f64, 
    pub quote_token_quantity: f64, 
    #[sea_orm(default_value = "open")]
    pub trade_type: TradeTypeEnum, 
    pub tx_id: String, 
    pub profit_loss: Option<bool>,
    pub token_id: i32,
    pub payment_id: i32, 
    pub trade_position_id: i32, 
    pub fee_rate: Option<f64>,
}