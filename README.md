# meetup-generator

[![Test](https://github.com/snltd/meetup-generator-rs/actions/workflows/test.yml/badge.svg)](https://github.com/snltd/meetup-generator-rs/actions/workflows/test.yml)

Built on an immutable polyglot femtoservice architecture, meetup-generator melds
Deep ML with the power of the Blockchain to deliver planetscale insights into
the direction of the most disruptive tech. On Kubernetes.

Or perhaps it just puts random words into a template?

## API

```sh
$ curl -s localhost:8000/api/talk | json
{
  "talk": "Dockerizing Dockerized Docker with Docker for Docker Users",
  "talker": "David Thomas",
  "role": "Open Source Dragonslayer",
  "company": "prognosticatr.io"
}
```

## Running

```sh
# From Docker Hub (AMD64 Linux)
$ docker run -p 8000:8000 snltd/meetup-generator:latest
# From a Github Checkout
$ cargo run
```

## Building

```sh
# Binary
$ cargo build --release
# Container
$ docker build -t latest .
```

## Contributing

Fork it, raise a PR.
