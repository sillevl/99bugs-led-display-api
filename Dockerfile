# select build image
FROM rust:1.32 as build

# Setup cross-compilation tools
RUN apt update
RUN rustup target add armv7-unknown-linux-gnueabihf
RUN rustup toolchain install stable-armv7-unknown-linux-gnueabihf

# create a new empty shell project
RUN USER=root cargo new --bin led-display-99bugs
WORKDIR /led-display-99bugs

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Set the toolchain of this foldr
RUN rustup override set stable-armv7-unknown-linux-gnueabihf

# this build step will cache your dependencies
RUN cargo build --release --target=armv7-unknown-linux-gnueabihf
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/api_99bugs_display*
RUN cargo build --release --target=armv7-unknown-linux-gnueabihf

# our final base
#FROM alpine:latest
FROM arm32v7/rust:1.32-slim

# copy the build artifact from the build stage
COPY --from=build /led-display-99bugs/target/release/api-99bugs-display .

# set the startup command to run your binary
CMD ["./api-99bugs-display"]
