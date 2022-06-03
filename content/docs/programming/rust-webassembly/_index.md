# Rust and WebAssembly support

- Rust is the emerging programming language and this experimentation is to use Rust code as WebAssembly package to be executed in the browser (e.g. Chrome)
- For example - JavaScript calling the Rust function directly into the browser or vice-versa.
- [Click here](https://www.rust-lang.org/learn) to start learning Rust. [Rust book](https://doc.rust-lang.org/book/) is also available for free.
- [Click here](https://webassembly.org/) to know more about WebAssembly (WASM) and [Rust example with WASM](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm).

## Installation and project generation

- All instructions are for MacOS only. Install Rust using brew and check Cargo (Rust's package manager):
- Rust is installed and managed by the rustup tool. [Click here](https://www.rust-lang.org/tools/install) to learn more.
```
brew install rustup
rustup-init
cargo --version
```
- Generate a Rust WebAssembly project
```
cargo new --lib rust-wasm
cargo build
```
- Build the WASM package (add ~/.cargo/bin to your PATH)
```
wasm-pack build --target web
```
- Run the Web Server in root folder:
```
python3 -m http.server
```
- It is running on: http://localhost:8000