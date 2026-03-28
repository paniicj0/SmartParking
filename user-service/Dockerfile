FROM rust:latest as builder

WORKDIR /app

COPY . .

ENV SQLX_OFFLINE=trues

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/user-service .

EXPOSE 8081

CMD ["./user-service"]