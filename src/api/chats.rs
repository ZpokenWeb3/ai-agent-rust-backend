use actix_web::{get, post, delete, patch, web, HttpResponse, Responder, HttpRequest};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use crate::core::db::get_db_pool;
use crate::utils::redis::RedisClient;
use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::Result;


#[derive(Serialize, Deserialize)]
pub struct Chat {
    pub id: i32,
    pub uuid: String,
    pub user_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub state: String, 
}


#[derive(Serialize, Deserialize)]
pub struct ChatCreate {
    pub user_id: i32,
}


#[get("/chats")]
pub async fn get_all_chats(
    pool: web::Data<PgPool>,
) -> impl Responder {
    let chats = sqlx::query_as!(
        Chat,
        "SELECT id, uuid, user_id, created_at, state FROM chats WHERE state != 'deleted' ORDER BY created_at DESC"
    )
    .fetch_all(pool.get_ref())
    .await;

    match chats {
        Ok(chats) => HttpResponse::Ok().json(chats),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch chats"),
    }
}


#[get("/chats/{user_id}")]
pub async fn get_user_chats(
    user_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let chats = sqlx::query_as!(
        Chat,
        "SELECT id, uuid, user_id, created_at, state FROM chats WHERE state != 'deleted' AND user_id = $1 ORDER BY created_at DESC",
        *user_id
    )
    .fetch_all(pool.get_ref())
    .await;

    match chats {
        Ok(chats) => HttpResponse::Ok().json(chats),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch user chats"),
    }
}


#[post("/chats")]
pub async fn create_chat(
    pool: web::Data<PgPool>,
    chat: web::Json<ChatCreate>,
) -> impl Responder {
    let chat_uuid = Uuid::new_v4().to_string();
    let created_at = Utc::now().naive_utc();

    let result = sqlx::query!(
        "INSERT INTO chats (uuid, user_id, created_at, state) VALUES ($1, $2, $3, 'active') RETURNING id, uuid, user_id, created_at, state",
        chat_uuid,
        chat.user_id,
        created_at
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(chat) => HttpResponse::Created().json(chat),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create chat"),
    }
}


#[get("/chats/uuid/{chat_uuid}")]
pub async fn get_chat_by_uuid(
    chat_uuid: web::Path<String>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let chat = sqlx::query_as!(
        Chat,
        "SELECT id, uuid, user_id, created_at, state FROM chats WHERE uuid = $1 AND state != 'deleted'",
        *chat_uuid
    )
    .fetch_optional(pool.get_ref())
    .await;

    match chat {
        Ok(Some(chat)) => HttpResponse::Ok().json(chat),
        Ok(None) => HttpResponse::NotFound().body("Chat not found"),
        Err(_) => HttpResponse::InternalServerError().body("Error retrieving chat"),
    }
}


#[delete("/chats/{chat_uuid}")]
pub async fn delete_chat(
    chat_uuid: web::Path<String>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let result = sqlx::query!(
        "UPDATE chats SET state = 'deleted' WHERE uuid = $1",
        *chat_uuid
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete chat"),
    }
}


#[patch("/chats/{chat_uuid}")]
pub async fn update_chat(
    chat_uuid: web::Path<String>,
    pool: web::Data<PgPool>,
    update_data: web::Json<serde_json::Value>,
) -> impl Responder {
    let update_query = format!(
        "UPDATE chats SET {} WHERE uuid = $1",
        update_data
            .iter()
            .map(|(k, _)| format!("{} = ${}", k, k))
            .collect::<Vec<_>>()
            .join(", ")
    );

    let result = sqlx::query(&update_query)
        .bind(&chat_uuid.into_inner())
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Chat updated"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update chat"),
    }
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_chats)
        .service(get_user_chats)
        .service(create_chat)
        .service(get_chat_by_uuid)
        .service(delete_chat)
        .service(update_chat);
}
