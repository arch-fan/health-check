FROM rust:latest as base
WORKDIR /build

FROM base as build
COPY . .

RUN cargo build --release

FROM alpine:latest as prod
WORKDIR /app
COPY --from=build /build/target/release/health-check .

CMD ["./health-check"]