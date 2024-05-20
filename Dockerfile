FROM rust:latest as base
WORKDIR /build

FROM base as build
COPY . .

RUN cargo build --release

FROM debian:stable-slim as prod
WORKDIR /app
COPY --from=builder /build/target/release/health-check .

CMD ["./health-check"]