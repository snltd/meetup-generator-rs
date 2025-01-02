FROM rust:latest as build

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates
COPY . /build
WORKDIR /build
RUN cargo test 
RUN cargo build --release

FROM gcr.io/distroless/cc as deploy
COPY --from=build /build/target/release/meetup-generator /
COPY Rocket.toml /
COPY resources/ /resources/
COPY templates/ /templates/
COPY static/ /static/
EXPOSE 8000
USER nobody
CMD ["/meetup-generator"]
