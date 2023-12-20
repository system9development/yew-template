FROM rust:alpine3.18
WORKDIR /server
COPY . .
RUN cargo install --path ./crates/server
CMD ["server"]