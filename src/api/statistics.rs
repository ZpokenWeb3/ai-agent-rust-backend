use actix_web::{get, web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use chrono::{DateTime, Utc};
use crate::db::get_db_pool;
use anyhow::Result;


pub const SOL_IMAGE_URL: &str = "https://img-v1.raydium.io/icon/So11111111111111111111111111111111111111112.png";


#[derive(Serialize, Deserialize)]
pub struct MessageCountResponse {
    pub total_messages: i32,
}


#[derive(Serialize, Deserialize)]
pub struct UsersCountResponse {
    pub total_users: i32,
}


#[derive(Serialize, Deserialize)]
pub struct ShillingStatisticsResponse {
    pub result: serde_json::Value,
}


#[derive(Serialize, Deserialize)]
pub enum ShillingStatisticsAction {
    DashboardInfo,
    TotalPaid,
    MaxMinTradePnL,
    Transactions,
    Rejections,
    CurrentBalance,
    Trades,
    AgentBalanceByMinutes,
    AgentBalanceByDay,
    AgentBalanceByWeek,
    AgentBalanceByMonth,
    AgentBalanceByYear,
    TokensApproved,
    Assets,
}


pub async fn get_unique_users_count(pool: &PgPool) -> Result<i32> {
    let count = sqlx::query!("SELECT COUNT(id) as count FROM users")
        .fetch_one(pool)
        .await?
        .count
        .unwrap_or(0);
    Ok(count)
}


pub async fn get_message_count_by_user_action(pool: &PgPool, user_id: i32, action: &str) -> Result<i32> {
    let count = sqlx::query!(
        "SELECT SUM(user_message_count) as count FROM chatactionextensions WHERE action = $1 AND user_id = $2",
        action,
        user_id
    )
    .fetch_one(pool)
    .await?
    .count
    .unwrap_or(0);
    Ok(count)
}


pub async fn get_count_of_trades(pool: &PgPool) -> Result<i32> {
    let count = sqlx::query!("SELECT COUNT(id) as count FROM trades")
        .fetch_one(pool)
        .await?
        .count
        .unwrap_or(0);
    Ok(count)
}


pub async fn get_total_pnl(pool: &PgPool) -> Result<f64> {
    let result = sqlx::query!(
        "SELECT SUM(closed.baseTokenQuantity - open.baseTokenQuantity) as total_pnl
         FROM trades open
         JOIN trades closed ON open.trade_position_id = closed.trade_position_id
         WHERE open.trade_type = 'open' AND closed.trade_type = 'closed'"
    )
    .fetch_one(pool)
    .await?
    .total_pnl
    .unwrap_or(0.0);
    Ok(result)
}


pub async fn get_max_min_pnl(pool: &PgPool) -> Result<(f64, f64, Option<String>, Option<String>)> {
    let row = sqlx::query!(
        "WITH pnl_calc AS (
            SELECT (closed.baseTokenQuantity - open.baseTokenQuantity) AS pnl, closed.tx_id, t.symbol
            FROM trades open
            JOIN trades closed ON open.trade_position_id = closed.trade_position_id
            JOIN tokens t ON closed.token_id = t.id
            WHERE open.trade_type = 'open' AND closed.trade_type = 'closed'
        )
        SELECT 
            (SELECT pnl FROM pnl_calc ORDER BY pnl DESC LIMIT 1) AS max_pnl,
            (SELECT tx_id FROM pnl_calc ORDER BY pnl DESC LIMIT 1) AS max_tx_id,
            (SELECT pnl FROM pnl_calc ORDER BY pnl ASC LIMIT 1) AS min_pnl,
            (SELECT tx_id FROM pnl_calc ORDER BY pnl ASC LIMIT 1) AS min_tx_id
        "
    )
    .fetch_one(pool)
    .await?;

    Ok((row.max_pnl.unwrap_or(0.0), row.min_pnl.unwrap_or(0.0), row.max_tx_id, row.min_tx_id))
}


#[get("/users")]
pub async fn get_all_users_count(pool: web::Data<PgPool>) -> impl Responder {
    match get_unique_users_count(pool.get_ref()).await {
        Ok(count) => HttpResponse::Ok().json(UsersCountResponse { total_users: count }),
        Err(_) => HttpResponse::InternalServerError().body("Error retrieving user count"),
    }
}


#[get("/messages")]
pub async fn get_messages_count_by_user_action(pool: web::Data<PgPool>, user_id: web::Query<i32>, action: web::Query<String>) -> impl Responder {
    match get_message_count_by_user_action(pool.get_ref(), *user_id, &action.into_inner()).await {
        Ok(count) => HttpResponse::Ok().json(MessageCountResponse { total_messages: count }),
        Err(_) => HttpResponse::InternalServerError().body("Error retrieving message count"),
    }
}


#[get("/total_pnl")]
pub async fn get_total_pnl_route(pool: web::Data<PgPool>) -> impl Responder {
    match get_total_pnl(pool.get_ref()).await {
        Ok(total_pnl) => HttpResponse::Ok().json(serde_json::json!({ "total_pnl": total_pnl })),
        Err(_) => HttpResponse::InternalServerError().body("Error retrieving total PnL"),
    }
}


#[get("/max_min_pnl")]
pub async fn get_max_min_pnl_route(pool: web::Data<PgPool>) -> impl Responder {
    match get_max_min_pnl(pool.get_ref()).await {
        Ok((max_pnl, min_pnl, max_tx_id, min_tx_id)) => HttpResponse::Ok().json(serde_json::json!({
            "max": { "pnl": max_pnl, "tx_id": max_tx_id },
            "min": { "pnl": min_pnl, "tx_id": min_tx_id }
        })),
        Err(_) => HttpResponse::InternalServerError().body("Error retrieving max/min PnL"),
    }
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_users_count)
        .service(get_messages_count_by_user_action)
        .service(get_total_pnl_route)
        .service(get_max_min_pnl_route);
}
