# syntax=docker/dockerfile:1.6

FROM rust:slim-bookworm AS build
RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
RUN mkdir -p src && printf 'fn main() {}' > src/main.rs
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo fetch

COPY src ./src
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release && cp target/release/rust_iss /tmp/rust_iss && strip /tmp/rust_iss

FROM gcr.io/distroless/cc-debian12:nonroot AS runtime
ENV RUST_LOG=info
WORKDIR /app
COPY --from=build /tmp/rust_iss /app/rust_iss
EXPOSE 3000
ENTRYPOINT ["/app/rust_iss"]
