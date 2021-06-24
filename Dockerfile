FROM rust:1.49-slim-buster

RUN apt-get update && \
    apt-get install -y \
    libopus-dev \
    build-essential \
    libssl-dev \
    autoconf \
    automake \
    libtool \
    m4 \
    ffmpeg \
    curl \
    python \
 && apt-get -y clean \
 && rm -rf /var/lib/apt/lists/*

RUN curl -L https://yt-dl.org/downloads/latest/youtube-dl -o /usr/bin/youtube-dl && \
    chmod a+rx /usr/bin/youtube-dl

ENV LC_ALL=C.UTF-8

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

COPY  . .

CMD [ "/bin/sh",  "-c", "cargo run" ]