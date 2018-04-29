FROM rust:latest AS backend

# static setup
RUN mkdir -p /code/src/backend && \
    echo 'fn main() {}' > /code/src/backend/main.rs

WORKDIR /code

# copy across dependencies
COPY Cargo.toml /code/Cargo.toml
COPY Cargo.lock /code/Cargo.lock

# build a cache of dependencies 
RUN cargo build --release --verbose

# copy over project code
COPY src/backend /code/src/backend

# remove fake build and build release
RUN rm target/release/fetch-api && \
    cargo build --release --verbose && \
    strip target/release/fetch-api

# Create final slim image
FROM debian:stretch-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends libpq5

WORKDIR /opt

COPY --from=backend /code/target/release/fetch-api /bin/fetch

CMD ["/bin/fetch", "--config", "/etc/fetch.toml"]
