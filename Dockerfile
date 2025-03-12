# Build stage
FROM rust:1.75 AS builder
WORKDIR /usr/src/stock_tracker
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /usr/src/stock_tracker/target/release/stock_tracker .
COPY --from=builder /usr/src/stock_tracker/data ./data
VOLUME ["/app/data"]
ENV PORT=8080
EXPOSE 8080
CMD ["./stock_tracker"]