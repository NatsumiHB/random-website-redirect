# fun-stuff
It's basically useless web but in cooler

## Usage
You can either use Rust by just running `cargo run` in the root of the repository.

When using the `docker-compose` file you will need to create a network called `proxy_net`.

| Endpoint   | Result                             |
| :--------: | :--------------------------------: |
| /          | Redirect to a random fun website   |
| /<url>     | Redirect to a specific run website |
| /all_urls  | JSON of all possible fun websites  |