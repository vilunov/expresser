FROM rust:1.28-slim-jessie
WORKDIR /build
COPY . .
RUN cargo build --release
RUN strip target/release/expresser

FROM debian:jessie-slim
COPY --from=0 /build/target/release/expresser /bin/
WORKDIR /app
VOLUME /app

CMD expresser
