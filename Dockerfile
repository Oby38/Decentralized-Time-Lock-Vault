FROM rust:1.81-slim

ARG STELLAR_CLI_VERSION=26.0.0

RUN apt-get update && apt-get install -y --no-install-recommends \
        curl \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && rustup target add wasm32-unknown-unknown \
    && curl -sSL "https://github.com/stellar/stellar-cli/releases/download/v${STELLAR_CLI_VERSION}/stellar-cli-v${STELLAR_CLI_VERSION}-x86_64-unknown-linux-gnu.tar.gz" \
       | tar -xz -C /usr/local/bin stellar

WORKDIR /app
COPY . .

RUN make build
