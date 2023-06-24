use std::net::SocketAddr;

use axum::{extract::State, routing::get, Json, Router};
use cloud::*;
use deadpool_diesel::sqlite;
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

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
        .route("/wasm/list", get(wasm_list))
        .with_state(pool);

    // Run it
    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    info!("Starting server on http://{address}");
    Ok(axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?)
}

async fn wasm_list(State(pool): State<Pool>) -> Result<Json<Vec<Wasm>>> {
    // Fetch from database
    use schema::wasm::dsl::*;

    let db = pool.get().await?;
    let tables = db
        .interact(|db| wasm.select(Wasm::as_select()).load(db))
        .await
        .unwrap()
        .unwrap();

    Ok(Json(tables))
}
