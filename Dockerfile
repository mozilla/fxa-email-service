# Docker 17.05 or higher required for multi-stage builds
FROM rust:1.26.0-stretch as builder
ADD . /app
WORKDIR /app
# Make sure that this matches in .travis.yml
ARG RUST_TOOLCHAIN=nightly
RUN \
    apt-get -qq update && \
    apt-get -qq install -y default-libmysqlclient-dev && \
    \
    rustup default ${RUST_TOOLCHAIN} && \
    cargo --version && \
    rustc --version && \
    # mkdir -m 755 bin && \  TODO: Do we move config to the binary or binary to the config??????
    cargo build --release && \
    cp /app/target/release/service /app


FROM debian:stretch-slim
# FROM debian:stretch  # for debugging docker build
RUN \
    groupadd --gid 10001 app && \
    useradd --uid 10001 --gid 10001 --home /app --create-home app && \
    \
    apt-get -qq update && \
    apt-get -qq install -y default-libmysqlclient-dev libssl-dev ca-certificates && \
    update-ca-certificates && \
    rm -rf /var/lib/apt/lists

COPY --from=builder /app /app

WORKDIR /app
USER app

CMD ["/app/service"]
