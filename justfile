set shell := ["bash", "-uc"]
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

GIT_COMMIT := `git rev-parse --verify --short HEAD`

# help
help:
  @just --list

# build wasm
build-wasm:
  @cargo build --release --target wasm32-unknown-unknown
  @wasm-bindgen --out-name bevy_breakout --out-dir wasm --target web target/wasm32-unknown-unknown/release/bevy-breakout.wasm
  @cp -r assets wasm

# install tools
install-tools: install-tools-build install-tools-dev

# install tools build
install-tools-build:
  @rustup target add wasm32-unknown-unknown
  @cargo install wasm-bindgen-cli

# install tools dev
install-tools-dev:
  @cargo install basic-http-server

# lint
lint:
  @cargo clippy

# run
run:
  @cargo run

# run dev
run-dev:
  @cargo run --features dev

# run wasm
run-wasm:
  @basic-http-server wasm

# test
test:
  @cargo test
