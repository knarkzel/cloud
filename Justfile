watch:
    cd web/ && npm run dev &
    cargo watch -x run

init:
    cd web/ && npm i
    cargo build

build: init
    cd web/ && npm run build
    cargo build --release
    ls -lh target/release/cloud
