FROM rust:1.61-slim-bullseye AS build-env

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

RUN curl -L https://github.com/jiro4989/ojosama/releases/download/v0.3.0/ojosama_0.3.0_linux_x86_64.tar.gz -o /tmp/ojosama_0.3.0_linux_x86_64.tar.gz && \
    tar -xzf /tmp/ojosama_0.3.0_linux_x86_64.tar.gz -C /usr/local/bin && \
    rm /tmp/ojosama_0.3.0_linux_x86_64.tar.gz

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

FROM debian:bullseye-20220418-slim

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

RUN curl -L https://yt-dl.org/downloads/latest/youtube-dl -o /usr/bin/youtube-dl && \
    chmod a+rx /usr/bin/youtube-dl

RUN curl -L https://github.com/jiro4989/ojosama/releases/download/v0.3.0/ojosama_0.3.0_linux_x86_64.tar.gz -o /tmp/ojosama_0.3.0_linux_x86_64.tar.gz && \
    tar -xzf /tmp/ojosama_0.3.0_linux_x86_64.tar.gz -C /usr/local/bin && \
    rm /tmp/ojosama_0.3.0_linux_x86_64.tar.gz

ENV LC_ALL=C.UTF-8

COPY --from=build-env /target/release/discord-speech-bot /bin/discord-speech-bot
COPY --from=build-env /sounds /sounds

CMD [ "/bin/sh",  "-c", "discord-speech-bot" ]