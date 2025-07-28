FROM mcr.microsoft.com/devcontainers/rust:1-bookworm AS dev-env

RUN apt-get update && \
    apt-get install -y \
    cmake \
    libopus-dev \
    build-essential \
    libssl-dev \
    libssl3 \
    pkg-config \
    autoconf \
    automake \
    libtool \
    m4 \
    curl \
    python3 \
    git

# install yt-dlp
RUN curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -o /usr/bin/yt-dlp && \
    chmod a+rx /usr/bin/yt-dlp

# install flyctl
RUN curl -L https://fly.io/install.sh | sh

# setup flyctl environment
ENV FLYCTL_INSTALL="/home/vscode/.fly"
ENV PATH="$FLYCTL_INSTALL/bin:$PATH"

ENV LC_ALL=C.UTF-8

COPY Makefile Makefile
RUN make install

FROM dev-env AS build-env

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/discord*

COPY  . .

RUN cargo build --release --features "tts"

CMD [ "/bin/sh",  "-c", "cargo run" ]

FROM debian:bookworm-20250721-slim

RUN apt-get update && \
    apt-get install -y \
    libopus-dev \
    build-essential \
    libssl-dev \
    libssl3 \
    pkg-config \
    autoconf \
    automake \
    libtool \
    m4 \
    curl \
    python3 \
 && apt-get -y clean \
 && rm -rf /var/lib/apt/lists/*

ENV LC_ALL=C.UTF-8

COPY --from=build-env /target/release/discord-tts-bot /bin/discord-tts-bot
COPY --from=build-env /sounds /sounds
COPY --from=build-env /usr/bin/yt-dlp /bin/yt-dlp

CMD [ "/bin/sh",  "-c", "discord-tts-bot" ]