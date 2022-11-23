use wasmedge_sdk::{
    error::HostFuncError, host_function, params, Caller, ImportObjectBuilder, Vm, WasmValue,
};

// fd_write: FuncType {params{i32 , i32 , i32 , i32} returns{i32}}
#[host_function]
fn my_fd_write(_caller: Caller, _args: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("this is my fd_write implementation");

    Ok(vec![])
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let import = ImportObjectBuilder::new()
        .with_func::<(), ()>("fd_write", my_fd_write)?
        .build("wasi_snapshot_preview1")?;

    // create a vm
    let vm = Vm::new(None)?;

    // register the wasm lib as a named module into the vm
    let vm = vm.register_import_module(import)?;

    // get the module instance named "wasi_snapshot_preview1"
    let wasi_instance = vm.named_module("wasi_snapshot_preview1")?;

    // list the exported wasm function in the instance
    wasi_instance
        .func_names()
        .unwrap()
        .iter()
        .for_each(|name| println!("func: {}", name));

    // register wasm module
    let wasm_file = "/Users/sam/workspace/rust/quark-demo/target/wasm32-wasi/release/wasm_lib.wasm";
    let vm = vm.register_module_from_file("extern", wasm_file)?;

    vm.run_func(Some("extern"), "hello", params!())?;

    Ok(())
}
