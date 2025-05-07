FROM alpine:3.21

RUN apk add --no-cache \
    git \
    curl \
    nodejs \
    npm \
    pnpm \
    wasm-pack \
    && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
