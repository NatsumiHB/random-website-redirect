FROM rust:1.45.2

WORKDIR /srv/fun-stuff
COPY . .

RUN cargo install --path .

EXPOSE 5001
CMD ["fun-stuff"]