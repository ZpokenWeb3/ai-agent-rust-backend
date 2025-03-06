use sea_orm::prelude::*;
use chrono::NaiveDateTime;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model { 
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32, 
    #[sea_orm(default_expr = "NOW()")]
    pub created_at: NaiveDateTime,
    #[sea_orm(unique)]
    pub wallet: String, 
    #[sea_orm(default_value = "null")]
    pub twitter_id: Option<String>,
    pub resricted_until: Option<NaiveDateTime>,
}