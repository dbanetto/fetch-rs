FROM golang:1.10 AS build

ARG version

RUN mkdir -p /code
WORKDIR /code
COPY . .

RUN make docker

FROM alpine:3.7

COPY --from=build /code/bin/fetch /usr/local/bin/fetch

RUN apk --update upgrade && \
    apk add curl ca-certificates && \
    update-ca-certificates && \
    rm -rf /varcache/apk/*

HEALTHCHECK --interval=5m CMD wget http://localhost:8181/healthcheck -q -O /dev/null
ENTRYPOINT ["/usr/local/bin/fetch", "-config", "/etc/fetcherd.toml"]
