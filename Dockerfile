###############################################################
# BUILD COINTAINER
###############################################################
FROM rust:1.67.1-buster as build
WORKDIR /app
ENV CARGO_TERM_COLOR=always

# Create a dummy project and build the app's dependencies.
RUN USER=root cargo new swimbotrs
WORKDIR /app/swimbotrs/

# Copy our manifests
COPY ./Cargo.lock .
COPY ./Cargo.toml .

# Build only the dependencies to cache them
RUN cargo build --release

# Remove the code by dummy creation
RUN rm src/*.rs

# Remove the unwanted dummy dependency
RUN rm ./target/release/deps/swimbotrs-*

# Now that the dependency is built, copy the source code
COPY ./src ./src

# Test the code
RUN cargo test
# Build for release
RUN cargo build --release

###############################################################
# PACKAGE FOR RUNTIME
###############################################################
# Copy the statically-linked binary into a distroless container
FROM gcr.io/distroless/cc
COPY --from=build /app/swimbotrs/target/release/swimbotrs .
ENV RUST_LOG=info

CMD ["./swimbotrs"]