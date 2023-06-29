use crate::{models::*, *};
use axum::body::Bytes;
use deadpool_diesel::sqlite::Connection;
use schema::wasm::dsl::*;
use wasmer_cache::Hash;

pub async fn wasm_create(
    db: Connection,
    wasm_title: String,
    wasm_description: String,
    wasm_types: String,
    wasm_binary: Bytes,
) -> Result<String> {
    let wasm_hash = Hash::generate(&wasm_binary);
    db.interact(move |db| {
        let new_wasm = NewWasm {
            hash: &wasm_hash.to_string(),
            binary: &wasm_binary,
            title: &wasm_title,
            description: &wasm_description,
            types: &wasm_types,
        };
        diesel::insert_into(wasm).values(new_wasm).execute(db)
    })
    .await
    .unwrap()?;

    Ok(wasm_hash.to_string())
}

pub async fn wasm_read(db: Connection, wasm_hash: String) -> Result<Wasm> {
    let table = db
        .interact(|db| wasm.find(wasm_hash).select(Wasm::as_select()).first(db))
        .await
        .unwrap()?;

    Ok(table)
}

pub async fn wasm_list(db: Connection) -> Result<Vec<Wasm>> {
    let tables = db
        .interact(|db| wasm.select(Wasm::as_select()).load(db))
        .await
        .unwrap()?;

    Ok(tables)
}

pub async fn wasm_fetch(db: Connection, wasm_hash: String) -> Result<Vec<u8>> {
    let table: Vec<u8> = db
        .interact(|db| wasm.find(wasm_hash).select(binary).first(db))
        .await
        .unwrap()?;

    Ok(table)
}
