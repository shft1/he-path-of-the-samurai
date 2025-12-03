# syntax=docker/dockerfile:1.6

FROM rust:slim-bookworm AS build
RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
RUN mkdir -p src && printf 'fn main() {}' > src/main.rs && cargo fetch

COPY src ./src
RUN cargo build --release && strip target/release/rust_iss

FROM gcr.io/distroless/cc-debian12:nonroot AS runtime
ENV RUST_LOG=info
WORKDIR /app
COPY --from=build /app/target/release/rust_iss /app/rust_iss
EXPOSE 3000
ENTRYPOINT ["/app/rust_iss"]
