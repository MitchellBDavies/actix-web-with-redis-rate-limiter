FROM rust:1.73.0 as test_container
WORKDIR /usr/src/myapp
COPY . .
RUN cargo test

FROM rust:1.73.0 as build_container
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release 
#RUN cp target/release/build_rust_with_docker .
CMD ["./build_rust_with_docker"]

FROM rust:1.73.0-slim as actix_server 
WORKDIR /usr/src/myapp
COPY --from=build_container /usr/src/myapp/target/release/build_rust_with_docker .
EXPOSE 8080
CMD ["./build_rust_with_docker"]