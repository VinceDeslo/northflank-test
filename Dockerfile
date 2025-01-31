# Build stage
FROM rust:1.84-slim AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /usr/local/bin
COPY --from=builder /usr/src/app/target/release/northflank-test .
EXPOSE 3000
CMD ["./northflank-test"]
