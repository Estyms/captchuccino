FROM rust:slim-bookworm AS base
RUN apt-get update
RUN apt-get install libssl-dev pkg-config -y
RUN cargo install cargo-chef
WORKDIR app

FROM base AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM base AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin captchuccino

FROM debian:bookworm-slim
RUN apt-get update && apt-get install ca-certificates libssl3 -y && apt-get clean autoclean && apt-get autoremove --yes && rm -rf /var/lib/{apt,dpkg,cache,log}/
WORKDIR /root/
COPY --from=builder /app/target/release/captchuccino .
COPY translations/ translations/
CMD ["./captchuccino"]