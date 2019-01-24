FROM rustlang/rust:nightly

RUN rustup update

WORKDIR /usr/src/app
COPY ./server/ /usr/src/app

EXPOSE 8080

ENTRYPOINT ["cargo", "run"]
