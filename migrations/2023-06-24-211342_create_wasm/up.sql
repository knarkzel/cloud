CREATE TABLE wasm (
  hash TEXT PRIMARY KEY NOT NULL,
  binary BLOB NOT NULL,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  types TEXT NOT NULL
);
