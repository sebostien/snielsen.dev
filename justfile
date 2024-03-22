alias rr := run-release
alias b  := build
alias bd := build-docker
alias rd := run-docker

build:
  cargo build --release
  pnpm -C frontend build
  mkdir -p static/
  cp ./frontend/build/* ./static -r

run-release:
  just b
  RUST_LOG=info ./target/release/backend

build-docker:
  docker build -t website .

run-docker:
  just build-docker
  docker run --env-file .env -d -it -p 8000:8000 website
