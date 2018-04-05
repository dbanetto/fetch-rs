FROM golang:1.10 AS build

RUN mkdir -p /go/src/gitlab.com/zyphrus/sitesync
WORKDIR /go/src/gitlab.com/zyphrus/sitesync
COPY . .

RUN go get -v -d ./ && \
    CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -v -o /usr/local/bin/sitesync cmd/sitesync/main.go

FROM alpine:3.7

COPY --from=build /usr/local/bin/sitesync /usr/local/bin/sitesync

RUN apk --update upgrade && \
    apk add curl ca-certificates && \
    update-ca-certificates && \
    rm -rf /varcache/apk/*

HEALTHCHECK --interval=30m CMD wget http://localhost:8181/healthcheck -q -O /dev/null

ENTRYPOINT ["/usr/local/bin/sitesync", "-config", "/etc/fetcherd.toml"]
