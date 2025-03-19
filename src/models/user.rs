use sea_orm::prelude::*;
use chrono::NaiveDateTime;
use sea_orm::{Set, ActiveModelBehavior};
use async_trait::async_trait;
use sea_orm::{DatabaseConnection, ConnectionTrait};

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model { 
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32, 

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: NaiveDateTime,

    #[sea_orm(unique)]
    pub wallet: String, 

    pub twitter_id: Option<String>,

    pub restricted_until: Option<NaiveDateTime>, 
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait] 
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(
        self,
        _db: &C,
        _insert: bool
    ) -> Result<Self, DbErr>
    where
        C: ConnectionTrait, // ✅ FIXED: Use ConnectionTrait
    {
        let mut model = self;
        if model.created_at.is_not_set() {
            model.created_at = Set(chrono::Utc::now().naive_utc());
        }
        Ok(model)
    }
}