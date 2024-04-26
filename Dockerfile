FROM rust:1.77-bullseye AS build

RUN #apt install -y pkg-config
WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
COPY src/ ./src/

RUN cargo build --release

FROM debian:bullseye

RUN apt update && apt install -y curl

WORKDIR /app
COPY --from=build /app/target/release/integral_ads /app/integral_ads
ENTRYPOINT ["/app/integral_ads"]