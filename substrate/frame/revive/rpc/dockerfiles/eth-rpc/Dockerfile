FROM rust AS builder

RUN apt-get update && \
		DEBIAN_FRONTEND=noninteractive apt-get install -y \
		protobuf-compiler \
		clang libclang-dev

WORKDIR /pezkuwi
COPY . /pezkuwi
RUN rustup component add rust-src
RUN cargo build --locked --profile production -p pallet-revive-eth-rpc --bin eth-rpc

FROM docker.io/parity/base-bin:latest
COPY --from=builder /pezkuwi/target/production/eth-rpc /usr/local/bin

USER root
RUN useradd -m -u 1001 -U -s /bin/sh -d /pezkuwi pezkuwi && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
	/usr/local/bin/eth-rpc --help

USER pezkuwi

# 8545 is the default port for the RPC server
# 9616 is the default port for the prometheus metrics
EXPOSE 8545 9616
ENTRYPOINT ["/usr/local/bin/eth-rpc"]

# We call the help by default
CMD ["--help"]
