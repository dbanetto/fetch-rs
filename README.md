# fetch-rs

Fetch web interface similar to [fetch-django](https://github.com/zyphrus/fetch-django)
using [diesel](https://diesel.rs)

## What you need

 * rust nightly, recommended to get it via [rustup](https://rustup.rs/)
 * postgres database
 * `DATABASE_URL` environment variable or via `.env` file

## Getting started

```bash
# assumes postgres is setup locally
echo 'DATABASE_URL=postgres://localhost/fetch_dev' > .env
cargo install diesel-cli
diesel migration run
cargo run
```

## Deploying

See the `docker-compose.yml` to see an example of how to deploy this app.

> Note: the provided `docker-compose.yml` is designed for development where 
> changes are live reloaded.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
