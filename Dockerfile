FROM rust:1.45.2

WORKDIR /srv/random-website-redirect
COPY . .

RUN cargo install --path .

EXPOSE 5001
CMD ["random-website-redirect"]