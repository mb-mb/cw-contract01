cargo wasm
cargo build
cosmwasm-check .\target\wasm32-unknown-unknown\release\counting_contract.wasm
cargo test
