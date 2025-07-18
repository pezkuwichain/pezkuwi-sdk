FROM docker.io/library/ubuntu:20.04

# metadata
ARG VCS_REF
ARG BUILD_DATE
ARG IMAGE_NAME

LABEL io.parity.image.authors="devops-team@parity.io" \
	io.parity.image.vendor="Parity Technologies" \
	io.parity.image.title="${IMAGE_NAME}" \
	io.parity.image.description="Cumulus, the Pezkuwi collator." \
	io.parity.image.source="https://github.com/paritytech/polkadot-sdk/blob/${VCS_REF}/docker/dockerfiles/pezkuwi-parachain/pezkuwi-parachain-debug_unsigned_injected.Dockerfile" \
	io.parity.image.revision="${VCS_REF}" \
	io.parity.image.created="${BUILD_DATE}" \
	io.parity.image.documentation="https://github.com/paritytech/polkadot-sdk/tree/master/cumulus"

# show backtraces
ENV RUST_BACKTRACE 1

# install tools and dependencies
RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y \
	libssl1.1 \
	ca-certificates \
	curl && \
	# apt cleanup
	apt-get autoremove -y && \
	apt-get clean && \
	find /var/lib/apt/lists/ -type f -not -name lock -delete; \
	# add user and link ~/.local/share/pezkuwi-parachain to /data
	useradd -m -u 1000 -U -s /bin/sh -d /pezkuwi-parachain pezkuwi-parachain && \
	mkdir -p /data /pezkuwi-parachain/.local/share && \
	chown -R pezkuwi-parachain:pezkuwi-parachain /data && \
	ln -s /data /pezkuwi-parachain/.local/share/pezkuwi-parachain && \
	mkdir -p /specs

# add pezkuwi-parachain binary to the docker image
COPY ./artifacts/pezkuwi-parachain /usr/local/bin
COPY ./cumulus/parachains/chain-specs/*.json /specs/

USER pezkuwi-parachain

# check if executable works in this container
RUN /usr/local/bin/pezkuwi-parachain --version

EXPOSE 30333 9933 9944
VOLUME ["/pezkuwi-parachain"]

ENTRYPOINT ["/usr/local/bin/pezkuwi-parachain"]
