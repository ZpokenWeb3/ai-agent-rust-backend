use sea_orm::prelude::*;
use crate::models::user::Model as User;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "credits")]
pub struct Model {
     #[sea_orm(primary_key, auto_increment = true)] 
     pub id: i32, 
     pub user_id: i32, 
     #[sea_orm(default_value = "0")]
     pub twitter_post_id: String, 
     #[sea_orm(default_value = false)]
     pub is_used: bool, 
}