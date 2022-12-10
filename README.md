# quark-demo

The steps of building and runing this example.

- Install `rustup` and `Rust`

  Go to the [official Rust webpage](https://www.rust-lang.org/tools/install) and follow the instructions to install `rustup` and `Rust`.

  > It is recommended to use Rust 1.63 or above in the stable channel.

  Then, add `wasm32-wasi` target to the Rustup toolchain:

  ```bash
  rustup target add wasm32-wasi
  ```

- Install `libwasmedge`

  Refer to the [Quick Install](https://wasmedge.org/book/en/quick_start/install.html#quick-install) section of WasmEdge Runtime Book to install `libwasmedge`. Or, use the following command directly

  ```bash
  // The command will create a directory `$HOME/.wasmedge`
  ./install_libwasmedge.sh

  source $HOME/.wasmedge/env
  ```

  > For users in China mainland (中国大陆地区), try the following command to install `libwasmedge` if failed to run the command above
  >
  > ```bash
  > ./install_libwasmedge_zh.sh
  > source $HOME/.wasmedge/env
  > ```

  > To install a specific version of `libwasmedge`, use `-v` option. For example, the following command installs `libwasmedge 0.11.2` to `$HOME/.wasmedge`
  >
  > ```bash
  > ./install_libwasmedge.sh -v 0.11.2
  > source $HOME/.wasmedge/env
  > ```

- Download example

  ```bash
  git clone git@github.com:apepkuss/quark-demo.git
  cd quark-demo
  ```

- Build `wasm-lib`

  ```bash
  // in quark-demo
  cargo build -p wasm-lib --target wasm32-wasi --release
  ```

  If the command runs successfully, `wasm-lib.wasm` can be found in the directory of `./target/wasm32-wasi/release/`.

- Build and run `call-wasm-lib`

  ```bash
  // in quark-demo
  cargo run -p run-wasm-lib -- ./target/wasm32-wasi/release/wasm_lib.wasm
  ```

  If the command runs successfully, then the following message is printed out on the screen:

  ```bash
  args: ["target/debug/run-wasm-lib", "./target/wasm32-wasi/release/wasm_lib.wasm"]
  func: fd_write
  [2022-12-10 12:26:51.504] [error] instantiation failed: unknown import, Code: 0x62
  [2022-12-10 12:26:51.504] [error]     When linking module: "wasi_snapshot_preview1" , function name: "environ_get"
  [2022-12-10 12:26:51.504] [error]     At AST node: import description
  [2022-12-10 12:26:51.504] [error]     At AST node: import section
  [2022-12-10 12:26:51.504] [error]     At AST node: module
  Error: Core(Instantiation(UnknownImport))
  ```

  As `run-wasm-lib` program only defines a host function for wasi `fd_write` for `wasi_snapshot_preview1` wasi module, you still have to provide the implementations for other wasi functions, for example, as shown in the error messages above, the `environ_get` function is not implemented, you need to define a host function for it.
