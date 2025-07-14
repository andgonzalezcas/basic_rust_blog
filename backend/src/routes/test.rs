use axum::{
    extract::State,
    routing::get,
    Json, Router,
};
use sqlx::{Pool, Postgres};
use serde_json;

type DbPool = Pool<Postgres>;

pub fn create_router(pool: DbPool) -> Router {
    Router::new()
        .route("/test", get(base))
        .route("/test/db", get(db_test))
        .with_state(pool)
}

async fn base() -> &'static str {
    "¡Hola! El servidor con Axum está funcionando. ✅"
}

async fn db_test(
    State(pool): State<DbPool>,
) -> Json<serde_json::Value> {
    let result = sqlx::query!("SELECT 1 + 1 AS sum")
        .fetch_one(&pool)
        .await;

    match result {
        Ok(rec) => Json(serde_json::json!({ "status": "success", "result": rec.sum })),
        Err(e) => Json(serde_json::json!({ "status": "error", "message": e.to_string() })),
    }
}