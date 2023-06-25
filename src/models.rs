use crate::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = wasm)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Wasm {
    pub hash: String,
    pub binary: Vec<u8>,
    pub title: String,
    pub description: String,
}

#[derive(Insertable)]
#[diesel(table_name = wasm)]
pub struct NewWasm<'a> {
    pub hash: &'a str,
    pub binary: &'a [u8],
    pub title: &'a str,
    pub description: &'a str,
}
