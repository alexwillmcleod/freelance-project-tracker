# FROM rust:latest as build

# # Create a new empty shell project
# RUN USER=root cargo new --bin freelance-api
# WORKDIR /freelance-api


# RUN rustup target add x86_64-unknown-linux-musl

# RUN apt-get update -y \
# && apt-get upgrade -y \
# && apt-get install -y cmake pkg-config libssl-dev

# # Copy our manifests
# COPY ./Cargo.lock ./Cargo.lock
# COPY ./Cargo.toml ./Cargo.toml

# # Build only the dependencies to cache them
# RUN cargo build --release
# RUN rm src/*.rs

# # Now that the dependency is built, copy your source code
# COPY ./src ./src

# # Build for release.
# RUN rm ./target/release/deps/freelance_api*
# RUN cargo build --release

# FROM rust:latest

# # Expose our port
# EXPOSE 80 

# # copy the build artifact from the build stage
# COPY --from=build /freelance-api/target/release/freelance-api .


# # Run release file
# CMD ["cargo", "run", "--release"]

FROM rust:latest

ENV MONGODB_URI 'mongodb+srv://root:6x0zFvxVGgxDO8MH@cluster0.kp8qsde.mongodb.net/?retryWrites=true&w=majority'
ENV JWT_SECRET '936DA01F9ABD4d9d80C702AF85C822A8jFfdsasfdsFKD&%*#)4e244324;jfja;fi3'
ENV PORT 80
ENV COOKIE_DOMAIN '127.0.0.1'
ENV RUST_ENV 'DEV'

COPY . .

EXPOSE 80

RUN cargo build --release

CMD ["cargo", "run", "--release"]