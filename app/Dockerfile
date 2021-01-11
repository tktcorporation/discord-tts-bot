FROM rust:alpine3.12

WORKDIR /app
RUN apk add --no-cache alpine-sdk git

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

COPY  . .

CMD [ "/bin/sh",  "-c", "cargo run" ]