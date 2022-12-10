use wasmedge_sdk::{
    error::HostFuncError, host_function, params, Caller, ImportObjectBuilder, Vm, WasmValue,
};

// https://github.com/WasmEdge/WasmEdge/blob/master/lib/host/wasi/wasimodule.cpp#L12
// fd_write: FuncType {params{i32 , i32 , i32 , i32} returns{i32}}
#[host_function]
fn my_fd_write(_caller: Caller, _args: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    println!("this is my fd_write implementation");

    Ok(vec![])
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    println!("args: {:?}", args);

    let wasm_file = &args[1];

    let import = ImportObjectBuilder::new()
        .with_func::<(i32, i32, i32, i32), i32>("fd_write", my_fd_write)?
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
    let vm = vm.register_module_from_file("extern", wasm_file)?;

    vm.run_func(Some("extern"), "hello", params!())?;

    Ok(())
}
