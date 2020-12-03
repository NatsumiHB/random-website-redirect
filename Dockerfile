FROM rust:1.48-alpine AS build

WORKDIR /srv/random-website-redirect
COPY . .

RUN apk add --no-cache musl-dev

RUN cargo install --path .

FROM alpine:latest

WORKDIR /srv/random-website-redirect
COPY --from=build /usr/local/cargo/bin/random-website-redirect .

EXPOSE 5001
CMD ./random-website-redirect