FROM rust:bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM rust:bookworm
USER 1000:1000
COPY --from=builder --chown=1000:1000 /app/target/release/takehome_hunt /takehome_hunt
ENTRYPOINT ["./takehome_hunt"]