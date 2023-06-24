use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::wasm)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Wasm {
    pub hash: String,
    pub binary: Vec<u8>
}
