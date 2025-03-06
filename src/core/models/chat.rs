use sea_orm::prelude::*;
use chrono::NaiveDateTime;
use crate::models::base::{State, ActionParameter};

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "chats")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32, 
    #[sea_orm(default_expr = "NOW()")]
    pub created_at: NaiveDateTime,
    #[sea_orm(unique)]
    pub uuid: String, 
    pub name: Option<String>,
    pub user_id: i32, 
    #[sea_orm(default_value = "active")]
    pub state: State, 
}

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "chat_action_extensions")]
pub struct ChatActionExtension { 
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32, 
    #[sea_orm(default_expr = "NOW()")]
    pub created_at: NaiveDateTime, 
    #[sea_orm(unique)]
    pub uuid: String, 
    pub name: Option<String>,
    pub user_id: i32, 
    #[sea_orm(default_value = "active")]
    pub state: State, 
    pub action: ActionParameter, 
    #[sea_orm(default_value = 0)]
    pub user_message_count: i32,
}