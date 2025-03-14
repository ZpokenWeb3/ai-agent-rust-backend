use actix_web::{get, web, HttpResponse, Responder};
use crate::contracts::solana::AgaveClient;

#[get("/agave/balance/{address}")]
async fn get_agave_balance(path: web::Path<String>) -> impl Responder {
    let address = path.into_inner();
    let agave_client = AgaveClient::new();

    match agave_client.get_balance(&address) {
        Ok(balance) => HttpResponse::Ok().json(serde_json::json!({ "balance": balance })),
        Err(e) => HttpResponse::InternalServerError().json(e),
    }
}
