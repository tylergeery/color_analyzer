FROM rustlang/rust:nightly

RUN rustup update

WORKDIR /usr/src/app
COPY ./server/ /usr/src/app

EXPOSE 8080

RUN cargo build --release
CMD cargo run --release
