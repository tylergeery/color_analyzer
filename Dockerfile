FROM rustlang/rust:nightly

RUN rustup update && \
    cargo install cargo-watch

WORKDIR /usr/src/app

EXPOSE 8080
