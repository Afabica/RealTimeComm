FROM rust:1.75 as builder
WORKDIR /app 
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo build --release

FROM debian:buster-slim 
COPY --from=builder /app/target/release/my-backend /usr/local/bin/my-backend
CMD ["my-backend"]
