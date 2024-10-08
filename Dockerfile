FROM rust AS builder

WORKDIR /bot
RUN apt-get update
RUN apt-get upgrade -y
RUN apt-get install -y cmake

COPY . .
RUN cargo build --release

FROM debian:stable-slim

RUN apt-get update
RUN apt-get upgrade -y
RUN apt-get install -y yt-dlp

COPY --from=builder /bot/target/release/lyrebird /
ENTRYPOINT ["/lyrebird"]