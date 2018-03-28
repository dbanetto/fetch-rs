FROM golang:1.10 AS build

RUN mkdir -p /go/src/gitlab.com/zyphrus/fetcherd-go
WORKDIR /go/src/gitlab.com/zyphrus/fetcherd-go
COPY . .

RUN go get -v -d ./ && \
    CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -v -o /usr/local/bin/fetcherd cmd/fetcherd/main.go

FROM alpine:3.7

COPY --from=build /usr/local/bin/fetcherd /usr/local/bin/fetcherd

RUN apk --update upgrade && \
    apk add curl ca-certificates && \
    update-ca-certificates && \
    rm -rf /varcache/apk/*

HEALTHCHECK CMD wget http://localhost:8181/api/v1/healthcheck -q -O /dev/null
CMD ["/usr/local/bin/fetcherd", "-config", "/etc/fetcherd.json"]
