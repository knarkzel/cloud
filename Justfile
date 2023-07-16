watch:
    cd web/ && npm run dev &
    cargo watch -x run

init:
    cd web/ && npm i

build: init
    cd web/ && npm run build
    cargo build --release
    zip cloud.zip target/release/cloud
    ls -lh target/release/cloud cloud.zip
