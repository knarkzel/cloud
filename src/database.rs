use crate::{models::*, *};
use axum::body::Bytes;
use deadpool_diesel::sqlite::Connection;
use schema::wasm::dsl::*;
use wasmer_cache::Hash;

pub async fn wasm_create(db: Connection, wasm_title: String, wasm_description: String, wasm_binary: Bytes) -> Result<()> {
    // Insert into database
    db.interact(move |db| {
        let wasm_hash = Hash::generate(&wasm_binary).to_string();
        let new_wasm = NewWasm {
            hash: &wasm_hash,
            binary: &wasm_binary,
            title: &wasm_title,
            description: &wasm_description,
        };
        diesel::insert_into(wasm).values(new_wasm).execute(db)
    })
    .await
    .unwrap()?;

    Ok(())
}

pub async fn wasm_read(db: Connection) -> Result<Vec<Wasm>> {
    // Get items from database
    use schema::wasm::dsl::*;

    let tables = db
        .interact(|db| wasm.select(Wasm::as_select()).load(db))
        .await
        .unwrap()?;

    Ok(tables)
}

pub async fn wasm_fetch(db: Connection, wasm_hash: String) -> Result<Vec<u8>> {
    // Fetch specific from database
    use schema::wasm::dsl::*;

    let table: Vec<u8> = db
        .interact(|db| wasm.find(wasm_hash).select(binary).first(db))
        .await
        .unwrap()?;

    Ok(table)
}
