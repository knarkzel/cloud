use std::net::SocketAddr;

use axum::extract::Multipart;
use axum::response::Redirect;
use axum::routing::post;
use axum::{extract::State, routing::get, Json, Router};
use cloud::*;
use deadpool_diesel::sqlite;
use diesel::prelude::*;
use diesel::QueryDsl;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use wasmer_cache::Hash;

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
            // Read binary
            let wasm_binary = field.bytes().await?;
            let wasm_hash = Hash::generate(&wasm_binary).to_string();

            // Insert into database
            use schema::wasm::dsl::*;

            let db = pool.get().await?;
            db.interact(move |db| {
                let new_wasm = NewWasm {
                    hash: &wasm_hash,
                    binary: &wasm_binary,
                };
                diesel::insert_into(wasm).values(new_wasm).execute(db)
            })
            .await
            .unwrap()?;

            Ok(Redirect::to("/"))
        }
        _ => Err(error!("Missing binary from form"))?,
    }
}

async fn wasm_read(State(pool): State<Pool>) -> Result<Json<Vec<Wasm>>> {
    // Fetch from database
    use schema::wasm::dsl::*;

    let db = pool.get().await?;
    let tables = db
        .interact(|db| wasm.select(Wasm::as_select()).load(db))
        .await
        .unwrap()?;

    Ok(Json(tables))
}
