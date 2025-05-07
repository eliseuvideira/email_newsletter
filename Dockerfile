FROM rust:1.82 AS builder
WORKDIR /usr/src/app
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM debian:bookworm-slim AS runner
WORKDIR /usr/src/app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends ca-certificates openssl \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/email_newsletter ./email_newsletter
COPY --from=builder /usr/src/app/configuration ./configuration
ENV APP_ENVIRONMENT=production
ENTRYPOINT ["./email_newsletter"]
