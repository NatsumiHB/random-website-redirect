# fun-stuff
It's basically useless web but in cooler

## Usage
You can either use Python by installing the required libraries from `reqirements.txt` and then running `python src/main.py` or alternatively using pipenv. If you want to use Docker, you can use the provided `docker-compose.yml`.

When using the `docker-compose` file you will need to create a network called `proxy_net`.

| Endpoint   | Result                             |
| :--------: | :--------------------------------: |
| /          | Redirect to a random fun website   |
| /<url>     | Redirect to a specific run website |
| /all_urls  | JSON of all possible fun websites  |