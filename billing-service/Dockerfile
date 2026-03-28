FROM rust:latest as builder

WORKDIR /app

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --release

FROM debian:bookworm-slim

# VAŽNO zbog TLS (email, reqwest)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/billing-service .

EXPOSE 8084

CMD ["./billing-service"]