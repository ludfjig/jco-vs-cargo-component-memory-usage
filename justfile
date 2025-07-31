build-js:
    cd component/js && npm run build

build-rs:
    cd component/rust && cargo component bindings && cargo component build --release

build-components: build-js build-rs

run component:
    cd runner && cargo run --release -- {{component}}