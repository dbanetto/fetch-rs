FROM rust:1.28 AS backend

# static setup
RUN mkdir -p /code/src && \
    echo 'fn main() {}' > /code/src/main.rs

WORKDIR /code

# copy across dependencies
COPY Cargo.toml /code/Cargo.toml
COPY Cargo.lock /code/Cargo.lock

# build a cache of dependencies 
RUN cargo build --release --verbose

# copy over project code
COPY src /code/src

# remove fake build and build release
RUN rm target/release/fetch-api && \
    cargo build --release --verbose && \
    strip target/release/fetch-api

# Create final slim image
FROM debian:stretch-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends libpq5 wget

WORKDIR /opt

COPY --from=backend /code/target/release/fetch-api /bin/fetch-api

HEALTHCHECK --interval=10m --timeout=3s --retries=3 --start-period=5s \
    CMD wget http://localhost:3000/api/healthcheck -q -O /dev/null

CMD ["/bin/fetch-api", "--config", "/etc/fetch.toml"]
