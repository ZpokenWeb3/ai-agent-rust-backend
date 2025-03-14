use sea_orm::prelude::*;
use chrono::NaiveDateTime;
use sea_orm::sea_query::Expr;
use sea_orm::{Set, ActiveModelBehavior};
use async_trait::async_trait; // ✅ Import async_trait for async functions
use std::pin::Pin;
use std::future::Future;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model { 
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32, 

    #[sea_orm(default_expr = "Expr::current_timestamp()")] // ✅ Use Expr::current_timestamp()
    pub created_at: NaiveDateTime,

    #[sea_orm(unique)]
    pub wallet: String, 

    #[sea_orm(default_value = "null")]
    pub twitter_id: Option<String>,

    pub restricted_until: Option<NaiveDateTime>, // ✅ Fixed typo: "resricted_until" → "restricted_until"
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait] // ✅ Make it async
impl ActiveModelBehavior for ActiveModel {
    async fn before_save(
        self,
        _db: &impl sea_orm::DatabaseConnection,
        _insert: bool
    ) -> Result<Self, DbErr> {
        let mut model = self;
        if model.created_at.is_not_set() { // ✅ `is_not_set()` instead of `is_set() == false`
            model.created_at = Set(chrono::Utc::now().naive_utc());
        }
        Ok(model)
    }
}
