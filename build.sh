cargo build --release --target=wasm32-wasip2
 
jco transpile target/wasm32-wasip2/release/dashbook_parser.wasm --out-dir pkg
