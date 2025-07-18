#!/usr/bin/env bash

OWNER=${OWNER:-parity}
IMAGE_NAME=${IMAGE_NAME:-pezkuwi-parachain}

docker build --no-cache \
    --build-arg IMAGE_NAME=$IMAGE_NAME \
    -t $OWNER/$IMAGE_NAME \
    -f ./docker/dockerfiles/pezkuwi-parachain/pezkuwi-parachain_injected.Dockerfile \
    . && docker images
