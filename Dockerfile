FROM rust:1.86-slim

RUN apt update  && apt install pkg-config libssl-dev -y
WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["cargo", "run", "--release"]