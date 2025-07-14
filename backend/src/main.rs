use dotenvy::dotenv;
use std::{env, net::SocketAddr};
use sqlx::{postgres::PgPoolOptions};

mod routes;

#[tokio::main]
async fn main() {
    // 1. Cargar la url db del env
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // 2. Crear pool de conexiones a la db
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create database pool.");
    
    println!("✅ Pool db creado.");

    // 3. Crear el router y definir las rutas.
    let app = routes::test::create_router(pool);

    // 4. Iniciar el servidor.
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Escuchando en http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}