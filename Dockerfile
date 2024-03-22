FROM rust:latest AS rust
WORKDIR /usr/src/backend
COPY backend/ ./

RUN cargo install --path .



FROM node:20-slim AS node
WORKDIR /usr/src/frontend
COPY frontend/ ./

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"

RUN corepack enable
RUN pnpm run build



FROM debian:trixie-slim

COPY --from=rust /usr/local/cargo/bin/backend /usr/local/bin/
COPY --from=node /usr/src/frontend/build/ /static/

EXPOSE 8000
ENTRYPOINT [ "backend" ]
