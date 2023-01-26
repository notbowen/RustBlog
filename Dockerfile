FROM rust:slim

RUN apt update && apt install -y pkg-config libssl-dev clang patch

WORKDIR /rust-blog
COPY ./ .

RUN cargo build --release

CMD ["/rust-blog/target/release/blog"]