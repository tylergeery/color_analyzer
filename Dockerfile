FROM rustlang/rust:nightly

ENV RUST_BACKTRACE 1
RUN rustup update && \
    cargo install cargo-watch

WORKDIR /usr/src/app

EXPOSE 8080
