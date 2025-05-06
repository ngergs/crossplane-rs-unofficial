FROM rust:1.86 AS builder
WORKDIR /usr/src/server
COPY . .
RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*
RUN cargo install --path .

# rust builder image also uses debian 12 so same glibc version
FROM gcr.io/distroless/cc-debian12
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
ENTRYPOINT ["/usr/local/bin/server"]
