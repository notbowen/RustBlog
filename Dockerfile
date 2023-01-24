# Builder
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

ENV USER=rust-blog-user
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /rust-blog

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

# Final image
FROM alpine

# Import user info from builder
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /rust-blog

# Copy binary over
COPY --from=builder /rust-blog/target/x86_64-unknown-linux-musl/release/blog ./

# Copy other files over
COPY ./ .

# Use an unprivileged user.
USER rust-blog:rust-blog

CMD ["/rust-blog/blog"]
