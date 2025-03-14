use actix_web::{post, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::core::db::get_db_pool;
use crate::core::redis::RedisClient;
use anyhow::Result;
use chrono::Utc;


#[derive(Serialize, Deserialize)]
pub struct ConnectTwitter {
    pub wallet_address: String,
    pub twitter_id: String,
}


#[derive(Serialize, Deserialize)]
pub struct Credit {
    pub user_id: i32,
    pub twitter_post_id: String,
    pub is_used: bool,
}


pub async fn create_users_retwitt(
    pool: &PgPool,
    user_id: i32,
    twitter_post_id: String,
) -> Result<Credit> {
    let credit = sqlx::query_as!(
        Credit,
        "INSERT INTO credits (user_id, twitter_post_id, is_used) VALUES ($1, $2, false) RETURNING user_id, twitter_post_id, is_used",
        user_id,
        twitter_post_id
    )
    .fetch_one(pool)
    .await?;
    Ok(credit)
}


pub async fn has_user_available_credits(pool: &PgPool, user_id: i32) -> Result<bool> {
    let credit = sqlx::query!(
        "SELECT id FROM credits WHERE user_id = $1 AND is_used = false LIMIT 1",
        user_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(credit.is_some())
}


pub async fn get_all_twitter_users(pool: &PgPool) -> Result<Vec<String>> {
    let users = sqlx::query!("SELECT twitter_id FROM users WHERE twitter_id IS NOT NULL")
        .fetch_all(pool)
        .await?;
    Ok(users.into_iter().map(|u| u.twitter_id.unwrap()).collect())
}


pub async fn check_existing_retwitted_post(pool: &PgPool, user_id: i32, post_id: String) -> Result<bool> {
    let retweet = sqlx::query!(
        "SELECT id FROM credits WHERE user_id = $1 AND twitter_post_id = $2 LIMIT 1",
        user_id,
        post_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(retweet.is_some())
}


pub async fn update_user(pool: &PgPool, wallet: String, twitter_id: String) -> Result<()> {
    sqlx::query!(
        "UPDATE users SET twitter_id = $1 WHERE wallet = $2",
        twitter_id,
        wallet
    )
    .execute(pool)
    .await?;
    Ok(())
}


#[post("/check_retwitts")]
pub async fn check_retwitts(pool: web::Data<PgPool>) -> impl Responder {
    let users = get_all_twitter_users(pool.get_ref()).await;
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().body("Error retrieving users"),
    }
}


#[post("/sell_tokens")]
pub async fn sell_tokens(pool: web::Data<PgPool>) -> impl Responder {
    HttpResponse::Ok().body("Sell tokens process started")
}


#[post("/log_agent_balance")]
pub async fn log_agent_balance(pool: web::Data<PgPool>) -> impl Responder {
    HttpResponse::Ok().body("Agent balance logged")
}


#[post("/connect_twitter")]
pub async fn connect_twitter(
    pool: web::Data<PgPool>,
    data: web::Json<ConnectTwitter>,
) -> impl Responder {
    match update_user(pool.get_ref(), data.wallet_address.clone(), data.twitter_id.clone()).await {
        Ok(_) => HttpResponse::Ok().body("User updated"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update user"),
    }
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(check_retwitts)
        .service(sell_tokens)
        .service(log_agent_balance)
        .service(connect_twitter);
}
