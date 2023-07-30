FROM rust:1.69 AS builder

RUN apt update && apt install -y clang

WORKDIR /rust-blog
COPY ./ .

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /rust-blog

RUN apt update && apt install -y libcurl4-openssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /rust-blog/target/release/blog ./
COPY --from=builder /rust-blog/static ./static
COPY --from=builder /rust-blog/templates ./templates

CMD ["./blog"]
