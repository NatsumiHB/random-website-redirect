# random-website-redirect
It's basically useless web but in cooler

## Usage
- Add urls to the `urls.json` file, they are in the format KEY:VALUE (if you don't do this, the server will crash upon a request)

Set the `RUST_LOG` environment variable in `.env` to specify your
[logging level](https://docs.rs/env_logger/0.7.1/env_logger/#enabling-logging).

You can either use Rust by just running `cargo run` in the root of the repository.

## Usage with Docker (`natsuwumi/random-website-redirect`)
Set the same environment variable, you can also use a `.env` file (or use the premade one for the defaults) and specify that in your docker command 
and also create a `urls.json` file.

The `urls.json` file must be bound to `/srv/random-website-redirect/urls.json` by using a volume in Docker.

The server runs on port 5002, you will need to publish that port.

| Endpoint   | Result                         |
| :--------: | :----------------------------: |
| /          | Redirect to a random website   |
| /\<url>    | Redirect to a specific website |
| /all_urls  | JSON of all possible websites  |
