# this image is built on top of existing Zombienet image
ARG ZOMBIENET_IMAGE
# this image uses substrate-relay image built elsewhere
ARG SUBSTRATE_RELAY_IMAGE=docker.io/paritytech/substrate-relay:v1.8.0

# metadata
ARG VCS_REF
ARG BUILD_DATE
ARG IMAGE_NAME

# we need `substrate-relay` binary, built elsewhere
FROM ${SUBSTRATE_RELAY_IMAGE} as relay-builder

# the base image is the zombienet image - we are planning to run zombienet tests using native
# provider here
FROM ${ZOMBIENET_IMAGE}

LABEL io.parity.image.authors="devops-team@parity.io" \
	io.parity.image.vendor="Parity Technologies" \
	io.parity.image.title="${IMAGE_NAME}" \
	io.parity.image.description="Bridges Zombienet tests." \
	io.parity.image.source="https://github.com/paritytech/polkadot-sdk/blob/${VCS_REF}/docker/dockerfiles/bridges_zombienet_tests_injected.Dockerfile" \
	io.parity.image.revision="${VCS_REF}" \
	io.parity.image.created="${BUILD_DATE}" \
	io.parity.image.documentation="https://github.com/paritytech/polkadot-sdk/bridges/testing"

# show backtraces
ENV RUST_BACKTRACE 1
USER root

# for native provider to work (TODO: fix in zn docker?)
RUN apt-get update && apt-get install -y procps sudo
RUN yarn global add @pezkuwi/api-cli

# add pezkuwi binary to the docker image
COPY ./artifacts/pezkuwi /usr/local/bin/
COPY ./artifacts/pezkuwi-execute-worker /usr/local/bin/
COPY ./artifacts/pezkuwi-prepare-worker /usr/local/bin/
# add pezkuwi-parachain binary to the docker image
COPY ./artifacts/pezkuwi-parachain /usr/local/bin
# copy substrate-relay to the docker image
COPY --from=relay-builder /home/user/substrate-relay /usr/local/bin/
# we need bridges zombienet runner and tests
RUN	mkdir -p /home/nonroot/bridges-pezkuwi-sdk
COPY ./artifacts/bridges-pezkuwi-sdk /home/nonroot/bridges-pezkuwi-sdk
# also prepare `generate_hex_encoded_call` for running
RUN set -eux; \
	cd /home/nonroot/bridges-pezkuwi-sdk/bridges/testing/framework/utils/generate_hex_encoded_call; \
	npm install

# use the non-root user
USER node
# check if executable works in this container
RUN /usr/local/bin/pezkuwi --version
RUN /usr/local/bin/pezkuwi-parachain --version
RUN /usr/local/bin/substrate-relay --version

# https://pezkuwi.js.org/apps/?rpc=ws://127.0.0.1:{PORT}#/explorer
EXPOSE 9942 9910 8943 9945 9010 8945
