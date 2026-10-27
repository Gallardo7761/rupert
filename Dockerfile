# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.98
ARG DEBIAN=trixie

FROM rust:${RUST_VERSION}-slim-${DEBIAN} AS builder
# libopus_sys builds Opus from source and links it statically.
RUN apt-get update \
    && apt-get install -y --no-install-recommends cmake make g++ \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target,sharing=locked \
    cargo test --workspace --release --locked \
    && cargo build --release --locked -p vito-discord \
    && cp target/release/vito-discord /usr/local/bin/vito-discord

FROM debian:${DEBIAN}-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
ADD --chmod=755 https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_linux /usr/local/bin/yt-dlp
COPY --from=denoland/deno:bin /deno /usr/local/bin/deno
RUN groupadd --system --gid 10001 vito \
    && useradd --system --uid 10001 --gid 10001 --no-create-home vito
COPY --from=builder /usr/local/bin/vito-discord /usr/local/bin/vito-discord
WORKDIR /app
USER vito
CMD ["vito-discord"]
