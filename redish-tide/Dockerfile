FROM rust:1.64 as builder

RUN USER=root cargo new --bin redish_tide
WORKDIR .
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/redish_tide
RUN cargo build --release

FROM alpine:latest
ARG APP=/usr/src/app

EXPOSE 8000
