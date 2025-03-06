use sear_orm::prelude::*;
use chrono::NaiveDateTime;
use crate::models::base::Model as BaseModel;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sear_orm(table_name = "agent_balance_changes")]
pub struct Model {
    #[sear_orm(primary_key, auto_increment = true)]
    pub id: i32, 
    #[sear_orm(default_expr = "NOW()")]
    pub created_at: NaiveDateTime, 
    pub amount: f64, 
    pub sol_amount: f64,
}