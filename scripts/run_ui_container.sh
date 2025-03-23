#!/bin/bash

docker stop lrzcc-ui || true
docker rm lrzcc-ui || true
docker run \
    --name lrzcc-ui \
    -d \
    lrzcc-ui
