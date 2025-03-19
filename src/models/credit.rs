use sea_orm::prelude::*;
use crate::models::user;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "credits")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)] 
    pub id: i32, 
    
    pub user_id: i32, 
    
    #[sea_orm(default_value = "0")]
    pub twitter_post_id: Option<String>, 

    #[sea_orm(default_value = false)]
    pub is_used: bool, 
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "user::Entity", 
        from = "Column::UserId",
        to = "user::Column::Id"
    )]
    User,
}

impl ActiveModelBehavior for ActiveModel {}
