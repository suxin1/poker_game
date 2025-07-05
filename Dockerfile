FROM lukemathwalker/cargo-chef:latest-rust-1.88.0-alpine3.21 AS chef
WORKDIR /app

# 更换 Cargo 源为 USTC 镜像
RUN echo '[source.crates-io]' > /usr/local/cargo/config.toml \
    && echo 'replace-with = "ustc"' >> /usr/local/cargo/config.toml \
    && echo '' >> /usr/local/cargo/config.toml \
    && echo '[source.ustc]' >> /usr/local/cargo/config.toml \
    && echo 'registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"' >> /usr/local/cargo/config.toml

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
# RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --bin server --release

# We do not need the Rust toolchain to run the binary!
FROM alpine AS runtime

ENV RUST_LOG=error \
    MAX_CLIENT="60" \
    HTTP_SERVER_ADDR="[::]:8081" \
    NATIVE_SOCKET_ADDR="[::]:8082" \
    WT_SOCKET_ADDR="[::]:8082" \
    WEB_SOCKET_ADDR="[::]:8085"

WORKDIR /app
COPY --from=builder /app/target/release/server /usr/local/bin
EXPOSE 8081 8082 8083 8084
ENTRYPOINT ["/usr/local/bin/server"]