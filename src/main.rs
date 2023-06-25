use axum::{
    body::{boxed, Full},
    extract::{Multipart, State},
    http::{header, HeaderValue, Uri},
    response::Redirect,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use cloud::*;
use deadpool_diesel::sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rust_embed::{EmbeddedFile, RustEmbed};
use std::{net::SocketAddr, path::PathBuf};
use tower_http::cors::CorsLayer;

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
        .route("/", get(index_handler))
        .route("/*path", get(static_handler))
        .fallback(index_handler)
        .with_state(pool)
        .layer(CorsLayer::new().allow_origin([
            HeaderValue::from_static("http://localhost:3000"),
            HeaderValue::from_static("http://localhost:5173"),
        ]));

    // Run it
    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    info!("Starting server on http://{address}");
    Ok(axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?)
}

async fn index_handler() -> impl IntoResponse {
    static_handler(Uri::from_static("/index.html")).await
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches("/");

    // Add .html to routes
    match PathBuf::from(path).extension() {
        Some(_) => StaticFile(path.to_string()),
        None => StaticFile(format!("{path}.html")),
    }
}

#[derive(RustEmbed)]
#[folder = "web/build"]
struct Assets;

pub struct StaticFile<T>(pub T);

fn render_file(path: &str, content: EmbeddedFile) -> Response {
    let body = boxed(Full::from(content.data));
    let mime = mime_guess::from_path(path).first_or_octet_stream();
    Response::builder()
        .header(header::CONTENT_TYPE, mime.as_ref())
        .body(body)
        .unwrap()
}

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Assets::get(&path) {
            Some(content) => render_file(&path, content),
            None => render_file("404.html", Assets::get("404.html").unwrap()),
        }
    }
}

async fn wasm_create(State(pool): State<Pool>, mut multipart: Multipart) -> Result<Redirect> {
    // Read form
    let title = multipart
        .next_field()
        .await?
        .ok_or(error!("Missing title"))?
        .text()
        .await?;

    let description = multipart
        .next_field()
        .await?
        .ok_or(error!("Missing description"))?
        .text()
        .await?;

    match multipart.next_field().await? {
        Some(field) => {
            // Insert into database
            let wasm_binary = field.bytes().await?;
            let db = pool.get().await?;
            database::wasm_create(db, title, description, wasm_binary).await?;
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
