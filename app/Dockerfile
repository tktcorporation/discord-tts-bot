FROM rust:1.49-slim-buster

RUN apt-get update && \
    apt-get install -y \
    libopus-dev \
    build-essential \
    autoconf \
    automake \
    libtool \
    m4 \
    ffmpeg \
    youtube-dl

WORKDIR /app

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

COPY  . .

CMD [ "/bin/sh",  "-c", "cargo run" ]