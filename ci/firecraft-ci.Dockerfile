FROM --platform=linux/amd64 rust:1.100.0-bookworm@sha256:30a9c647e188457aac6e0bd3f9d73ae725dc3f36e57a2808ba3dde653c7c6db8

ARG CARGO_LOCK_SHA256

LABEL org.opencontainers.image.title="FireCraft GitForge CI" \
      org.opencontainers.image.version="4" \
      org.opencontainers.image.source="https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment" \
      org.firecraft.cargo-lock-sha256="${CARGO_LOCK_SHA256}"

RUN apt-get update -qq \
    && apt-get install -y --no-install-recommends \
        build-essential \
        libssl-dev \
        libxcb-render0-dev \
        libxcb-shape0-dev \
        libxcb-xfixes0-dev \
        libxkbcommon-dev \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN rustup component add rustfmt clippy \
    && rustup target add wasm32-unknown-unknown

WORKDIR /workspace
COPY app/Cargo.toml app/Cargo.lock ./app/
COPY app/src/main.rs ./app/src/main.rs
RUN cargo fetch --locked --manifest-path app/Cargo.toml

ENV CARGO_NET_OFFLINE=true \
    CARGO_TERM_COLOR=always
