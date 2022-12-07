cargo build --release --target=wasm32-unknown-unknown

wasm-tools component new \
  target/wasm32-unknown-unknown/release/dashbook_parser.wasm -o target/dashbook_parser.wasm
  
wit-bindgen host js target/dashbook_parser.wasm --out-dir pkg