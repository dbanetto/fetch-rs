FROM golang:1.10 AS build

RUN mkdir -p /go/src/github.com/zyphrus/fetcherd-go
WORKDIR /go/src/github.com/zyphrus/fetcherd-go
COPY . .

RUN go get -v -d ./ && \
    CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -v -o /usr/local/bin/fetcherd cmd/fetcherd/main.go

FROM alpine

COPY --from=build /usr/local/bin/fetcherd /usr/local/bin/fetcherd

CMD ["/usr/local/bin/fetcherd", "-config", "/etc/fetcherd.json"]
