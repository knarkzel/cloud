use axum::{
    extract::{Multipart, State},
    response::Redirect,
    routing::{get, post},
    Json, Router,
};
use cloud::*;
use deadpool_diesel::sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::net::SocketAddr;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[tokio::main]
async fn main() -> Result<()> {
    // Logging
    tracing_subscriber::fmt::init();

    // Database
    let manager = sqlite::Manager::new("database.sqlite", deadpool_diesel::Runtime::Tokio1);
    let pool = sqlite::Pool::builder(manager).build()?;

    // Run migrations
    {
        let connection = pool.get().await?;
        connection
            .interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }

    // Create application
    let app = Router::new()
        .route("/api/wasm/read", get(wasm_read))
        .route("/api/wasm/create", post(wasm_create))
        .with_state(pool);

    // Run it
    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    info!("Starting server on http://{address}");
    Ok(axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?)
}

async fn wasm_create(State(pool): State<Pool>, mut multipart: Multipart) -> Result<Redirect> {
    // Read form
    match multipart.next_field().await? {
        Some(field) => {
            // Insert into database
            let db = pool.get().await?;
            let wasm_binary = field.bytes().await?;
            database::wasm_create(db, wasm_binary).await?;
            Ok(Redirect::to("/"))
        }
        _ => Err(error!("Missing binary from form"))?,
    }
}

async fn wasm_read(State(pool): State<Pool>) -> Result<Json<Vec<Wasm>>> {
    // Fetch from database
    let db = pool.get().await?;
    Ok(Json(database::wasm_read(db).await?))
}
