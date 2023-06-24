// Modules
pub mod database;
pub mod models;
pub mod schema;
pub mod wasm;

// Imports
pub use axum_error::Result;
pub use deadpool_diesel::sqlite::Pool;
pub use diesel::prelude::*;
pub use eyre::eyre as error;
pub use models::*;
pub use tracing::info;
