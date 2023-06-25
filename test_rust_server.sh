#!/bin/bash
# Download rust-web-server
wget https://github.com/Realsnack/rust-web-server/archive/refs/heads/main.zip
unzip main.zip
cd rust-web-server-main

# Install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Build rust-web-server
cargo build --release

# Run rust-web-server
./target/release/rust-web-server &

export PATH=$PATH:/root/k6

# Run k6
./k6 run /root/test.js -e URL=http://localhost:6920 -e MAX_VU=1000
