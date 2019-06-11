# -*- mode: dockerfile -*-
#
# An example Dockerfile showing how to build a Rust executable using this
# image, and deploy it with a tiny Alpine Linux container.

# You can override this `--build-arg BASE_IMAGE=...` to use different
# version of Rust or OpenSSL.
ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS builder

# Add our source code.
ADD . ./

# At this point:
#RUN pwd -> /home/rust/src

# Fix permissions on source code (rust-musl-builder).
RUN sudo chown -R rust:rust /home/rust

# Build our application.
RUN cargo build --release

# Now, we need to build our _real_ Docker container, copying in `multipart-example`.
FROM alpine:latest
RUN apk --no-cache add ca-certificates

# \ - next line operator
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/multipart-example \
    /usr/local/bin/

EXPOSE 3000

CMD /usr/local/bin/multipart-example