FROM rust:latest as build

# create a new empty shell project
RUN USER=root cargo new --bin web_stress_tester
WORKDIR /web_stress_tester

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/web_stress_tester*
RUN cargo build --release

# our final base
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /web_stress_tester/target/release/web_stress_tester .

# We intend to expose 8080 tcp port
EXPOSE 8080

# set the startup command to run your binary
CMD ["./web_stress_tester"]
