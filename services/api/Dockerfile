FROM rust:1.31 AS backend

RUN mkdir -p /code/src

WORKDIR /code

# copy across dependencies
COPY Cargo.toml /code/Cargo.toml
COPY Cargo.lock /code/Cargo.lock
COPY src /code/src

# remove fake build and build release
RUN cargo build --release --verbose

# Create final slim image
FROM debian:stretch-slim

ENV RUST_LOG info
ENV RUST_BACKTRACE 1

RUN apt-get update && \
    apt-get install -y --no-install-recommends libpq5 wget

WORKDIR /opt

COPY --from=backend /code/target/release/fetch-api /bin/fetch-api

HEALTHCHECK --interval=10m --timeout=3s --retries=3 --start-period=5s \
    CMD wget http://localhost:3000/healthcheck -q -O /dev/null

CMD ["/bin/fetch-api", "--config", "/etc/fetch.toml"]
