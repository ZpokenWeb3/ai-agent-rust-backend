use sea_orm::prelude::*;
use chrono::NaiveDateTime;
use sea_orm::sea_query::Expr; 
use sea_orm::Set;
#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model { 
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32, 

    #[sea_orm(default_value = "CURRENT_TIMESTAMP")]
    pub created_at: NaiveDateTime,

    #[sea_orm(unique)]
    pub wallet: String, 

    #[sea_orm(default_value = "null")]
    pub twitter_id: Option<String>,

    pub resricted_until: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn before_save(model: &mut ActiveModel, _insert: bool) -> Result<(), DbErr> {
        if model.created_at.is_set() == false {
            model.created_at = Set(chrono::Utc::now().naive_utc());
        }
        Ok(())
    }
}
