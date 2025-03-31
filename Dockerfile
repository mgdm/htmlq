ARG ALPINE_VERSION=3.21
ARG RUST_VERSION=1.85
ARG HTMLQ_VERSION=0.4.0
ARG TARGET=x86_64-unknown-linux-musl

FROM rust:$RUST_VERSION-alpine$ALPINE_VERSION AS build
ARG TARGET
ARG HTMLQ_VERSION
RUN apk add --no-cache musl-dev gcc
RUN rustup target add "$TARGET"
RUN cargo install htmlq --version "$HTMLQ_VERSION" --target "$TARGET" --root /htmlq-build
RUN /htmlq-build/bin/htmlq --version

FROM alpine:$ALPINE_VERSION AS release
COPY --from=build /htmlq-build/bin/htmlq /usr/bin/
