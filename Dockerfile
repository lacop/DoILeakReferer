FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && mv ./target/release/doileakreferer ./doileakreferer

FROM debian:bullseye-slim

RUN useradd -ms /bin/bash app

USER app
WORKDIR /app
COPY --from=builder /usr/src/app/doileakreferer /app/doileakreferer

CMD ./doileakreferer