FROM rust:latest
WORKDIR /usr/src/app
COPY Cargo.toml  ./
RUN mkdir src
COPY ./src/main.rs ./src/
RUN cargo build  --release
CMD ["target/release/test_res_srv"]
