#!/bin/bash

docker stop avina-ui || true
docker rm avina-ui || true
docker run \
    --name avina-ui \
    -d \
    avina-ui
