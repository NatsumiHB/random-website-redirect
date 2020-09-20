# random-website-redirect
It's basically useless web but in cooler

## Usage
- Add urls to the `urls.json` file, they are in the format KEY:VALUE (if you don't do this, the server will crash upon a request)

Set the `RUST_LOG` environment variable in `.env` to specify your
[logging level](https://docs.rs/env_logger/0.7.1/env_logger/#enabling-logging).

You can either use Rust by just running `cargo run` in the root of the repository.

When using the `docker-compose` file you will need to create a network called `proxy_net`.

The server runs on port 5001, you will need to public that port in the `docker-compose.yml` file.

| Endpoint   | Result                         |
| :--------: | :----------------------------: |
| /          | Redirect to a random website   |
| /\<url>    | Redirect to a specific website |
| /all_urls  | JSON of all possible websites  |
