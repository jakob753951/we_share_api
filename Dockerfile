FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR we_share_api
LABEL authors="jakob753951"

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /we_share_api/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin we_share_api

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR we_share_api
COPY --from=builder /we_share_api/target/release/we_share_api /usr/local/bin
ENTRYPOINT ["/usr/local/bin/we_share_api"]