FROM rust:1.60 as build

RUN USER=root cargo new --bin lazylog
WORKDIR /lazylog

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/lazylog*
RUN cargo build --release

FROM debian:buster-slim
COPY --from=build /lazylog/target/release/lazylog .

CMD ["./lazylog"]