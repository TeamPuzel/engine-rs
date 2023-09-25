cd ./client
cargo build --release
cd ..
mv ./client/target/wasm32-unknown-unknown/release/dungeons.wasm ./content/client.wasm