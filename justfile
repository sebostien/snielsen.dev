alias rr := run-release
alias b  := build

build:
  cargo build --release
  pnpm -C frontend build
  mkdir -p static/
  cp ./frontend/build/* ./static -r

run-release:
  just b
  RUST_LOG=info ./target/release/backend

