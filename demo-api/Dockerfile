FROM rust:1.86 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin demo-api


FROM debian:bookworm-slim as runner

RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

COPY --from=builder /app/target/release/demo-api /app/demo-api

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_SECRET_KEY=
ENV ROCKET_PORT=8000

EXPOSE 8000

CMD ["./demo-api"]
