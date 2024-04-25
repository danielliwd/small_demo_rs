# STAGE1: Build the binary
FROM rust:alpine as builder

# Install build dependencies
RUN apk add --no-cache build-base musl-dev openssl-dev openssl

# Create a new empty shell project
WORKDIR /app

# Copy over the Cargo.toml files to the shell project
COPY  docker_rust_scripts.sh  Cargo.toml Cargo.lock ./

COPY docker_rust_precompile.rs src/main.rs

# Build and cache the dependencies
ARG APPNAME=app
RUN cargo fetch \
    && cargo build --release \
    && rm -rf ./src \
    && ./docker_rust_scripts.sh remove_local_deps_after_build


# Copy the actual code files and build the application
COPY src ./src/
RUN cargo build --release \
    && ./docker_rust_scripts.sh rename_artifact_to_app


# STAGE2: create a slim image with the compiled binary
# docker build --build-arg="APPNAME=real_app_name" .

# https://docs.docker.com/reference/dockerfile/#understand-how-arg-and-from-interact
# FROM 前的ARG在FROM之后不可用,不影响stage
ARG OS_VERSION
FROM alpine as runner

# Copy the binary from the builder stage
WORKDIR /app
COPY --from=builder /app/target/release/app app

CMD ["./app"]
