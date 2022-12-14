
use wasmedge_sdk::{
    error::HostFuncError, host_function,  Caller, ImportObjectBuilder, Vm, WasmValue, types::ExternRef
};

#[host_function]
fn hello(_caller: Caller, args: Vec<WasmValue>) -> Result<Vec<WasmValue>, HostFuncError> {
    // parse the first argument of WasmValue type
    let result = args[0].extern_ref::<MyString>();
    assert!(result.is_some());
    let my_string = result.unwrap();
    println!("args[0]: {}", my_string.s);

    // parse the second argument of WasmValue type
    let result = args[1].extern_ref::<MyStr>();
    assert!(result.is_some());
    let my_str = result.unwrap();
    println!("args[1]: {}", my_str.s);
   
    Ok(vec![])
}

// MyString is similar to Rust built-in String
#[derive(Debug)]
pub struct MyString {
    pub s: String
}

// MyStr is similar to Rust built-in string slice, namely str
#[derive(Debug)]
pub struct MyStr<'a> {
    pub s: &'a str
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let import = ImportObjectBuilder::new()
        .with_func::<(ExternRef, ExternRef), ()>("say_hello", hello)?
        .build("extern")?;

    // create a vm
    let vm = Vm::new(None)?;

    // register the wasm lib as a named module into the vm
    let vm = vm.register_import_module(import)?;

    let s = "Earth";
    let mut my_string = MyString { s: String::from(s) };

    let s = "Moon";
    let mut my_str = MyStr { s };
   
    vm.run_func(Some("extern"), "say_hello", vec![WasmValue::from_extern_ref(&mut my_string), WasmValue::from_extern_ref(&mut my_str)])?;
   
    Ok(())
}
