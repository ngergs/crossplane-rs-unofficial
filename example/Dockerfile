# needs to run in ../ to include the sdk directory in the docker environment
FROM rust:1.89-bookworm AS builder
WORKDIR /usr/src/crossplane-rs-example-unofficial
COPY . .
RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/* && cargo install kopium
RUN cd example && cargo install --path .

# rust builder image also uses debian 12 so same glibc version
FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=builder /usr/local/cargo/bin/crossplane-rs-example-unofficial /usr/local/bin/crossplane-rs-example-unofficial
EXPOSE 9443
ENTRYPOINT ["/usr/local/bin/crossplane-rs-example-unofficial"]
