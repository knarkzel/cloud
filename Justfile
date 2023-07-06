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

deploy: build
    mkdir -p target/release/output
    cp target/release/cloud target/release/output/cloud
