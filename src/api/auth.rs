use actix_web::{web, get, post, HttpResponse, Responder, HttpRequest, HttpMessage};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use actix_web::cookie::{Cookie, CookieJar};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::db::get_db_pool;
use crate::redis::RedisClient;
use anyhow::Result;
use std::collections::HashMap;

/// User schema
#[derive(Serialize, Deserialize)]
pub struct User {
    pub wallet: String,
}

/// Add a new user (Equivalent to `crud.py::add_user`)
pub async fn add_user(pool: &PgPool, wallet: String) -> Result<User> {
    sqlx::query!(
        "INSERT INTO users (wallet) VALUES ($1) RETURNING wallet",
        wallet
    )
    .fetch_one(pool)
    .await
    .map(|record| User { wallet: record.wallet })
    .map_err(anyhow::Error::from)
}

/// Select user by wallet (Equivalent to `crud.py::select_by_wallet`)
pub async fn select_by_wallet(pool: &PgPool, wallet: &str) -> Result<Option<User>> {
    let user = sqlx::query!(
        "SELECT wallet FROM users WHERE wallet = $1",
        wallet
    )
    .fetch_optional(pool)
    .await?
    .map(|record| User { wallet: record.wallet });

    Ok(user)
}

/// Extract user from session token (Equivalent to `dependencies.py::extract_user_from_access_token`)
pub async fn extract_user_from_access_token(
    req: HttpRequest,
    redis_client: web::Data<Arc<Mutex<RedisClient>>>,
    pool: web::Data<PgPool>,
) -> Result<User> {
    let cookies = req.cookies().unwrap_or(vec![]);
    let session_cookie = cookies
        .iter()
        .find(|c| c.name() == "session_id")
        .map(|c| c.value().to_string());

    let session_id = session_cookie.ok_or_else(|| anyhow::anyhow!("No session ID found"))?;
    let redis = redis_client.lock().await;
    let session_data: HashMap<String, String> = redis.get_json(&session_id).await?;
    
    let address = session_data.get("siwe_address").ok_or_else(|| anyhow::anyhow!("Invalid session"))?;
    let user = select_by_wallet(pool.get_ref(), address).await?;

    if let Some(user) = user {
        Ok(user)
    } else {
        add_user(pool.get_ref(), address.clone()).await
    }
}

/// Generate nonce for authentication (Equivalent to `views.py::get_nonce`)
#[get("/nonce")]
pub async fn get_nonce(
    redis_client: web::Data<Arc<Mutex<RedisClient>>>,
    req: HttpRequest,
) -> impl Responder {
    let session_id = uuid::Uuid::new_v4().to_string();
    let nonce = "some_nonce"; // Generate nonce logic

    let mut redis = redis_client.lock().await;
    redis.set_json(&session_id, &serde_json::json!({ "nonce": nonce })).await.unwrap();

    let mut response = HttpResponse::Ok().body(nonce);
    response.add_cookie(&Cookie::build("session_id", session_id).finish()).unwrap();

    response
}

/// Verify SIWE message (Equivalent to `views.py::verify`)
#[post("/verify")]
pub async fn verify(
    body: web::Json<HashMap<String, String>>,
    redis_client: web::Data<Arc<Mutex<RedisClient>>>,
    req: HttpRequest,
) -> impl Responder {
    let session_cookie = req.cookie("session_id").map(|c| c.value().to_string());
    let session_id = match session_cookie {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().body("No session found"),
    };

    let mut redis = redis_client.lock().await;
    let session_data: HashMap<String, String> = redis.get_json(&session_id).await.unwrap();

    if session_data.get("nonce") != body.get("nonce") {
        return HttpResponse::Unauthorized().body("Invalid nonce");
    }

    redis.set_json(&session_id, &serde_json::json!({ "siwe_address": body.get("address") })).await.unwrap();

    HttpResponse::Ok().body("Verified")
}

/// Logout user (Equivalent to `views.py::logout`)
#[post("/logout")]
pub async fn logout(redis_client: web::Data<Arc<Mutex<RedisClient>>>, req: HttpRequest) -> impl Responder {
    if let Some(session_id) = req.cookie("session_id").map(|c| c.value().to_string()) {
        let mut redis = redis_client.lock().await;
        redis.delete(&session_id).await.unwrap();
        HttpResponse::Ok().body("Logged out")
    } else {
        HttpResponse::Unauthorized().body("No session found")
    }
}

/// Get user details (Equivalent to `views.py::me`)
#[get("/me")]
pub async fn me(user: web::Data<User>) -> impl Responder {
    HttpResponse::Ok().json(user.wallet.clone())
}

/// Register routes
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_nonce)
        .service(verify)
        .service(logout)
        .service(me);
}
