# fetcherd-go

A Go implementation of the fetcherd ([python version](https://github.com/zyphrus/fetcherd-py)).

## Supports

 - [X] fetch
 - [ ] sorting
 - [x] Web interface
 - [x] CLI based runs
 - [x] Configuration

## Running

To start the web service

`go run cmd/fetcherd/main.go`

To force a manual fetch either run

`go run cmd/fetcherd/main.go -fetch`

or by sending a POST request to `/force/fetch`, e.g.

`curl -X POST localhost:8181/force/fetch`

## Config

The configuration is in the form of a JSON file,
see [config.json](./config.json) for an example file.

```JSON
{
  "api": "http://localhost:8080", // base location of fetch API
  "webui": {
    "enable": true, // if the web api should be run at all
    "host": "localhost:8181" // where it should be hosted
  }
}
```

## License

MIT, see [License.md](./LICENSE.md) for details
