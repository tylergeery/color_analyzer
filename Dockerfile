FROM rustlang/rust:nightly

RUN apt-get update && apt-get -y install google-perftools libgoogle-perftools-dev

RUN rustup update

WORKDIR /usr/src/app
COPY ./server/ /usr/src/app

RUN cargo update && \
    cargo build --release

CMD cargo run --release

EXPOSE 8080
