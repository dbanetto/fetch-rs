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

RUN rm target/release/fetch-web && \
    cargo build --release --verbose && \
    strip target/release/fetch-web

FROM node:8 AS frontend

RUN mkdir -p /code/src/frontend

WORKDIR /code

COPY package.json /code/package.json
COPY package-lock.json /code/package-lock.json

RUN npm install

COPY webpack.config.js /code/webpack.config.js
COPY tsconfig.json /code/tsconfig.json

COPY src/frontend /code/src/frontend

RUN npm run build

FROM debian:stretch-slim

RUN apt update && \
    apt install -y --no-install-recommends libpq5

WORKDIR /opt

COPY --from=backend /code/target/release/fetch-web /opt/fetch
COPY --from=frontend /code/public /opt/public
COPY migrations /opt/migrations

ENTRYPOINT ["/opt/fetch"]
