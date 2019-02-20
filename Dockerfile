# select build image
FROM rust:1.32 as build

# Setup cross-compilation tools
RUN apt update
RUN apt install -qq gcc-arm-linux-gnueabihf -y
RUN rustup target add armv7-unknown-linux-gnueabihf
RUN mkdir -p ~/.cargo
RUN echo '[target.armv7-unknown-linux-gnueabihf]' >> ~/.cargo/config
RUN echo 'linker = "arm-linux-gnueabihf-gcc"' >> ~/.cargo/config

# create a new empty shell project
RUN USER=root cargo new --bin led-display-99bugs
WORKDIR /led-display-99bugs

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
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
