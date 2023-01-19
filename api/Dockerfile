FROM rust:1.66 as build

# Create a new empty shell project
RUN USER=root cargo new --bin freelance-api
WORKDIR /freelance-api

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Now that the dependency is built, copy your source code
COPY ./src ./src

# Build for release.
RUN rm ./target/release/deps/freelance_api*
RUN cargo build --release

FROM rust:1.66

# Expose our port
EXPOSE 7878

# copy the build artifact from the build stage
COPY --from=build /freelance-api/target/release/freelance-api .

# Run release file
# WORKDIR /freelance-api

CMD ["./freelance-api"]