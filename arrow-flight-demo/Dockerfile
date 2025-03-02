FROM rust:1.84.1

WORKDIR /usr/src/arrow-flight-demo
COPY Cargo.toml .
COPY src ./src/

RUN cargo build --bin decoder --release
RUN cargo install --path . --bin decoder

EXPOSE 50051
CMD ["decoder", "-p", "50051"]