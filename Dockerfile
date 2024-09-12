FROM rust:alpine as build
WORKDIR /build
COPY . .
RUN cargo build --release

FROM busybox:stable
WORKDIR /app
COPY --from=build /build/target/release/health-check .

CMD ["./health-check"]