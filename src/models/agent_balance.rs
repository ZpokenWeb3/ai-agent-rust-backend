use sea_orm::prelude::*;
use chrono::NaiveDateTime;
use crate::models::base::Model as BaseModel;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "agent_balance_changes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32, 
    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: NaiveDateTime, 
    pub amount: f64, 
    pub sol_amount: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
