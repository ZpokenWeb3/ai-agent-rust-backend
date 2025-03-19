use sea_orm::prelude::*;
use chrono::NaiveDateTime;
use crate::models::base::{State};

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "chats")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32, 

    #[sea_orm(default_expr = "Expr::current_timestamp()")] 
    pub created_at: NaiveDateTime,

    #[sea_orm(unique)]
    pub uuid: String, 

    pub name: Option<String>,
    pub user_id: i32, 

    #[sea_orm(default_value = "active")]
    pub state: State, 
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
