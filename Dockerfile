# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.98
ARG DEBIAN=trixie

FROM rust:${RUST_VERSION}-slim-${DEBIAN} AS builder
WORKDIR /app
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target,sharing=locked \
    cargo test --workspace --release --locked \
    && cargo build --release --locked -p rupert-discord \
    && cp target/release/rupert-discord /usr/local/bin/rupert-discord

FROM debian:${DEBIAN}-slim
RUN groupadd --system --gid 10001 rupert \
    && useradd --system --uid 10001 --gid 10001 --no-create-home rupert
COPY --from=builder /usr/local/bin/rupert-discord /usr/local/bin/rupert-discord
WORKDIR /app
USER rupert
CMD ["rupert-discord"]
