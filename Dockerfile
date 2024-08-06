FROM rust:alpine3.19 as builder
WORKDIR /app
RUN apk add musl-dev
COPY . .
RUN cargo build --release

FROM scratch
USER 1000:1000
COPY --from=builder --chown=1000:1000 /app/target/release/takehome_hunt /takehome_hunt
ENTRYPOINT ["/takehome_hunt"]