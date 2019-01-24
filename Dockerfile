FROM rustlang/rust:nightly

RUN rustup update

WORKDIR /usr/src/app

EXPOSE 8080

ENTRYPOINT ["cargo", "run"]
