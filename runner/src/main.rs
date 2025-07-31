use std::fs;
use wasmtime::{
    Store,
    component::{Component, Linker, ResourceTable},
};
use wasmtime_wasi::p2::{IoView, WasiCtx, WasiCtxBuilder, WasiView, add_to_linker_sync};

mod bindings {
    wasmtime::component::bindgen!(in "../component/component.wit");
}

struct MyState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl IoView for MyState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl WasiView for MyState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

fn get_memory_usage() -> (u64, u64) {
    let status = fs::read_to_string("/proc/self/status").expect("Failed to read /proc/self/status");
    let mut vm_size = None;
    let mut vm_rss = None;

    for line in status.lines() {
        if line.starts_with("VmSize:") {
            vm_size = Some(
                line.split_whitespace()
                    .nth(1)
                    .expect("Missing VmSize value")
                    .parse::<u64>()
                    .expect("Failed to parse VmSize as u64"),
            );
        } else if line.starts_with("VmRSS:") {
            vm_rss = Some(
                line.split_whitespace()
                    .nth(1)
                    .expect("Missing VmRSS value")
                    .parse::<u64>()
                    .expect("Failed to parse VmRSS as u64"),
            );
        }
    }

    (
        vm_size.expect("VmSize not found in /proc/self/status"),
        vm_rss.expect("VmRSS not found in /proc/self/status"),
    )
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <rs|js>", args[0]);
        std::process::exit(1);
    }

    let component_type = &args[1];
    let manifest = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set. Please run using cargo");

    let component_path = match component_type.as_str() {
        "js" => format!("{}/../component/js/handler.wasm", manifest),
        "rs" => format!(
            "{}/../component/rust/target/wasm32-wasip1/release/handler_rs.wasm",
            manifest
        ),
        _ => {
            eprintln!(
                "Invalid component type: {}. Use 'rs' or 'js'",
                component_type
            );
            std::process::exit(1);
        }
    };

    println!(
        "Loading {} component from: {}",
        component_type, component_path
    );

    // Setup Wasmtime engine and component
    let engine = wasmtime::Engine::default();
    let component =
        Component::from_file(&engine, component_path).expect("Failed to load component");
    println!("Finished compiling component");

    // Setup WASI linker
    let mut linker = Linker::new(&engine);
    add_to_linker_sync(&mut linker).expect("Failed to add WASI to linker");

    // Create WASI context and store
    let wasi_ctx = WasiCtxBuilder::new().build();
    let table = ResourceTable::new();
    let state = MyState {
        ctx: wasi_ctx,
        table,
    };
    let mut store = Store::new(&engine, state);

    // Instantiate the component
    let instance = bindings::HandlerWorld::instantiate(&mut store, &component, &linker)
        .expect("Failed to instantiate component");

    let request = bindings::exports::test::test::handler_interface::Request {
        uri: "Test URI".to_string(),
    };

    let handler = instance.test_test_handler_interface();

    // Memory leak detection loop
    for i in 0..1000000000 {
        if i % 100000 == 0 {
            let (vm_size, vm_rss) = get_memory_usage();
            println!(
                "Iteration: {}, Virtual: {} KB, RES: {} KB",
                i, vm_size, vm_rss
            );
        }
        handler
            .call_handleevent(&mut store, &request)
            .expect("Failed to call handleevent");
    }

    println!("got here 2");
}
