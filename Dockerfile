FROM rust:1.86-slim

RUN apt update  && apt install pkg-config libssl-dev -y
WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["./app/target/release/env_test"]