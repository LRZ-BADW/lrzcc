#!/bin/bash

docker build \
    --tag gierens/lrzcc:v0.1.0 \
    --tag gierens/lrzcc:latest \
    --file Dockerfile \
    .
