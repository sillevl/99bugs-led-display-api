# select build image
FROM rust:1.32 as build

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
RUN cargo build --release

# our final base
#FROM alpine:latest
FROM rust:1.32

# copy the build artifact from the build stage
COPY --from=build /led-display-99bugs/target/release/api-99bugs-display .

# set the startup command to run your binary
CMD ["./api-99bugs-display"]
