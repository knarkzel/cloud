use axum::{
    body::{boxed, Full},
    extract::{Multipart, Path, State},
    http::{header, HeaderValue, Uri},
    response::Redirect,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router, Form,
};
use cloud::{*, wasm::Engine};
use deadpool_diesel::sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rust_embed::{EmbeddedFile, RustEmbed};
use serde_json::Value;
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
        .route("/api/wasm/list", get(wasm_list))
        .route("/api/wasm/read/:hash", get(wasm_read))
        .route("/api/wasm/create", post(wasm_create))
        .route("/api/wasm/run/:hash", get(wasm_run))
        .route("/api/wasm/:hash", get(wasm_fetch))
        .route("/", get(index_handler))
        .route("/*path", get(static_handler))
        .fallback(error_handler)
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

async fn error_handler() -> impl IntoResponse {
    static_handler(Uri::from_static("/404.html")).await
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

    let types = multipart
        .next_field()
        .await?
        .ok_or(error!("Missing types"))?
        .text()
        .await?;

    let wasm_binary = multipart
        .next_field()
        .await?
        .ok_or(error!("Missing binary"))?
        .bytes()
        .await?;

    // Create in database
    let db = pool.get().await?;
    let hash = database::wasm_create(db, title, description, types, wasm_binary).await?;
    Ok(Redirect::to(&format!("/run?hash={hash}")))
}

async fn wasm_read(State(pool): State<Pool>, Path(hash): Path<String>) -> Result<Json<Wasm>> {
    // Fetch single from database
    let db = pool.get().await?;
    Ok(Json(database::wasm_read(db, hash).await?))
}

async fn wasm_list(State(pool): State<Pool>) -> Result<Json<Vec<Wasm>>> {
    // Fetch from database
    let db = pool.get().await?;
    Ok(Json(database::wasm_list(db).await?))
}

async fn wasm_fetch(State(pool): State<Pool>, Path(hash): Path<String>) -> Result<Json<Vec<u8>>> {
    // Fetch from database
    let db = pool.get().await?;
    let bytes = database::wasm_fetch(db, hash).await?;
    Ok(Json(bytes))
}

async fn wasm_run(
    State(pool): State<Pool>,
    Path(hash): Path<String>,
    Form(input): Form<Value>,
) -> Result<Json<Value>> {
    // Fetch from database
    let db = pool.get().await?;
    let bytes = database::wasm_fetch(db, hash).await?;
    
    // Spawn task and run wasm
    tokio::task::spawn_blocking(move || {
        let mut engine = Engine::new()?;
        let output = engine.run::<Value, Value>(&bytes, &input)?;
        Ok(Json(output))
    }).await?
}
