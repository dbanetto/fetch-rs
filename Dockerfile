FROM rust:1.24 AS backend

# static setup
RUN apt update && \
    apt install -y musl-tools && \
    mkdir -p /code/src/backend && \
    echo 'fn main() {}' > /code/src/backend/main.rs && \
    rustup target add x86_64-unknown-linux-musl 

WORKDIR /code

# copy across dependencies
COPY Cargo.toml /code/Cargo.toml
COPY Cargo.lock /code/Cargo.lock

# build a cache of dependencies 
RUN cargo build --release --verbose --target x86_64-unknown-linux-musl

# copy over project code
COPY src/backend /code/src/backend

RUN rm target/x86_64-unknown-linux-musl/release/fetch-web && \
    cargo build --release --verbose --target x86_64-unknown-linux-musl 

FROM node:8 AS frontend

RUN mkdir -p /code/src/frontend

WORKDIR /code

COPY package.json /code/package.json
COPY package-lock.json /code/package-lock.json

RUN npm install

COPY webpack.config.js /code/webpack.config.js
COPY tsconfig.json /code/tsconfig.json
COPY .babelrc /code/.babelrc

COPY src/frontend /code/src/frontend

RUN npm run build

FROM scratch 

WORKDIR /opt

COPY --from=backend /code/target/x86_64-unknown-linux-musl/release/fetch-web /fetch
COPY --from=frontend /code/public /public
COPY config.toml /config.toml

ENTRYPOINT ["/fetch"]
