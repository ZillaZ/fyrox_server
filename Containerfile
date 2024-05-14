FROM rust:1.75

WORKDIR /usr/src/server

RUN mkdir src; touch src/main.rs

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

COPY src/ ./src/
EXPOSE 8000

RUN cargo build --release

CMD cargo run
