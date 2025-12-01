# Multi-stage build für optimale Image-Größe
FROM rust:1.75-alpine AS builder

# Build-Dependencies
RUN apk add --no-cache musl-dev openssl-dev

WORKDIR /build

# Workspace structure kopieren
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Release build
RUN cargo build --release --bin honeytrap-server

# Runtime image
FROM alpine:latest

# Runtime dependencies
RUN apk add --no-cache ca-certificates libgcc

# Non-root user erstellen
RUN addgroup -g 1000 honeytrap && \
    adduser -D -u 1000 -G honeytrap honeytrap

WORKDIR /app

# Binary kopieren
COPY --from=builder /build/target/release/honeytrap-server /app/honeytrap-server

# Ownership
RUN chown -R honeytrap:honeytrap /app

USER honeytrap

# Config volume
VOLUME ["/app/config"]

# Ports (anpassbar je nach Config)
EXPOSE 8080 8443

# Environment defaults
ENV RUST_LOG=info,honeytrap=debug
ENV HONEYTRAP_CONFIG=/app/config/honeytrap.toml
ENV HONEYTRAP_JSON_LOGS=true

ENTRYPOINT ["/app/honeytrap-server"]
