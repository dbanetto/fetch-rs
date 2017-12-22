# fetcherd-go

A Go implementation of the fetcherd ([python version](https://github.com/zyphrus/fetcherd-py)).

## Supports

 - [X] fetch
 - [ ] sorting
 - [x] Logging
 - [x] Web interface
   - [x] force fetch `POST /force/fetch`
   - [ ] force sort `POST /force/fetch`
   - [ ] read log `GET /log`
 - [x] CLI based runs
 - [x] Configuration

## Running

To get started run the following command in the `fetcherd-go` directory

`go get ./`

To start the web service

`go run cmd/fetcherd/main.go`

To force a manual fetch either run

`go run cmd/fetcherd/main.go -fetch`

or by sending a POST request to `/force/fetch`, e.g.

`curl -X POST localhost:8181/force/fetch`

## Config

The configuration is in the form of a JSON file,
see [config.json](./config.json) for an example file.

```javascript
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
