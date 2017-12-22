FROM golang:1.9

RUN mkdir -p /go/src/github.com/zyphrus/fetcherd-go
WORKDIR /go/src/github.com/zyphrus/fetcherd-go
COPY . .

RUN go get -v -d ./ && go build -v -o fetcherd cmd/fetcherd/main.go

CMD ["./fetcherd", "-config", "/etc/fetcherd.json"]
