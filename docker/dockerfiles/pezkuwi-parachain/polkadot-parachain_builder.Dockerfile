# This file is sourced from https://github.com/paritytech/polkadot-sdk/blob/master/docker/dockerfiles/pezkuwi-parachain/pezkuwi-parachain_builder.Dockerfile
# This is the build stage for pezkuwi-parachain. Here we create the binary in a temporary image.
FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /cumulus
COPY . /cumulus

RUN cargo build --release --locked -p pezkuwi-parachain

# This is the 2nd stage: a very small image where we copy the Pezkuwi binary."
FROM docker.io/library/ubuntu:20.04

LABEL io.parity.image.type="builder" \
    io.parity.image.authors="devops-team@parity.io" \
    io.parity.image.vendor="Parity Technologies" \
    io.parity.image.description="Multistage Docker image for pezkuwi-parachain" \
    io.parity.image.source="https://github.com/paritytech/polkadot-sdk/blob/${VCS_REF}/docker/dockerfiles/pezkuwi-parachain/pezkuwi-parachain_builder.Dockerfile" \
    io.parity.image.documentation="https://github.com/paritytech/polkadot-sdk/tree/master/cumulus"

COPY --from=builder /cumulus/target/release/pezkuwi-parachain /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /cumulus pezkuwi-parachain && \
    mkdir -p /data /cumulus/.local/share && \
    chown -R pezkuwi-parachain:pezkuwi-parachain /data && \
    ln -s /data /cumulus/.local/share/pezkuwi-parachain && \
# unclutter and minimize the attack surface
    rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
    /usr/local/bin/pezkuwi-parachain --version

USER pezkuwi-parachain

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/pezkuwi-parachain"]
