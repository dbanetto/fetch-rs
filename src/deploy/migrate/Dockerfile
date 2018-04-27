FROM rust:latest AS build

RUN cargo install diesel_cli --no-default-features --features postgres

FROM debian:stretch-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends libpq5

WORKDIR /opt

COPY --from=build /usr/local/cargo/bin/diesel /bin/diesel
COPY migrations /opt/migrations

CMD ["/bin/diesel", "migration", "run", "--migration-dir", "/opt/migrations"]
