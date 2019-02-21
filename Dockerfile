# select build image
FROM rust:1.32 as build

# Setup cross-compilation tools
RUN apt update
RUN apt install -qq -y gcc-arm-linux-gnueabihf
RUN rustup target add armv7-unknown-linux-gnueabihf
RUN rustup toolchain install stable-armv7-unknown-linux-gnueabihf

# create a new empty shell project
RUN USER=root cargo new --bin led-display-99bugs
WORKDIR /led-display-99bugs

# copy settings cargo config file
COPY ./.cargo ./.cargo

# # copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# # this build step will cache your dependencies
RUN cargo build --release --target=armv7-unknown-linux-gnueabihf
RUN rm src/*.rs
RUN rm ./target/armv7-unknown-linux-gnueabihf/release/deps/api_99bugs_display*

# copy your source tree
COPY ./src ./src

# build for release
RUN cargo build --release --target=armv7-unknown-linux-gnueabihf

# our final base
FROM arm32v7/rust:1.32-slim

# copy the build artifact from the build stage
COPY --from=build /led-display-99bugs/target/armv7-unknown-linux-gnueabihf/release/api-99bugs-display .

# set the startup command to run your binary
CMD ["./api-99bugs-display"]
