FROM rust:1.67.0-slim-bullseye AS build-env

RUN apt-get update && \
    apt-get install -y \
    libopus-dev \
    build-essential \
    libssl-dev \
    pkg-config \
    autoconf \
    automake \
    libtool \
    m4 \
    ffmpeg \
    curl \
    python \
    git

RUN curl https://raw.githubusercontent.com/nektos/act/master/install.sh | bash

RUN curl -L https://yt-dl.org/downloads/latest/youtube-dl -o /usr/bin/youtube-dl && \
    chmod a+rx /usr/bin/youtube-dl

ARG ojosama_version=0.11.0

RUN curl -L https://github.com/jiro4989/ojosama/releases/download/v${ojosama_version}/ojosama_${ojosama_version}_linux_x86_64.tar.gz -o /tmp/ojosama_${ojosama_version}_linux_x86_64.tar.gz && \
    tar -xzf /tmp/ojosama_${ojosama_version}_linux_x86_64.tar.gz -C /usr/bin && \
    rm /tmp/ojosama_${ojosama_version}_linux_x86_64.tar.gz

ENV LC_ALL=C.UTF-8

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/discord*

COPY  . .

RUN cargo build --release --features "tts"

CMD [ "/bin/sh",  "-c", "cargo run" ]

FROM debian:bullseye-20230208-slim

RUN apt-get update && \
    apt-get install -y \
    libopus-dev \
    build-essential \
    libssl-dev \
    pkg-config \
    autoconf \
    automake \
    libtool \
    m4 \
    ffmpeg \
    curl \
    python \
 && apt-get -y clean \
 && rm -rf /var/lib/apt/lists/*

ENV LC_ALL=C.UTF-8

COPY --from=build-env /target/release/discord-tts-bot /bin/discord-tts-bot
COPY --from=build-env /sounds /sounds
COPY --from=build-env /usr/bin/youtube-dl /bin/youtube-dl
COPY --from=build-env /usr/bin/ojosama /bin/ojosama

CMD [ "/bin/sh",  "-c", "discord-tts-bot" ]