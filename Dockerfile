FROM rust:1.86-slim

RUN apt update && apt install pkg-config -y
WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["./target/release/env_test"]