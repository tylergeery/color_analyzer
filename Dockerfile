FROM rustlang/rust:nightly as prod

RUN apt-get update && apt-get -y install google-perftools libgoogle-perftools-dev

RUN rustup update

WORKDIR /usr/src/app
COPY ./server/ /usr/src/app

RUN cargo update && \
    cargo build --release

CMD cargo run --release

EXPOSE 8080


FROM prod as dev

RUN rustup update && \
    rustup self update && \
    rustup component add clippy --toolchain=nightly || cargo install --git https://github.com/rust-lang/rust-clippy/ --force clippy

RUN cargo install cargo-watch

ENV RUST_BACKTRACE 1

ENTRYPOINT ["cargo", "watch", "-x", "run"]
