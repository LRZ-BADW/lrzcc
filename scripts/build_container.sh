#!/bin/bash

VERSION=0.7.0

docker build \
    --tag "lrzcc-api/lrzcc:v${VERSION}" \
    --tag "lrzcc-api/lrzcc:latest" \
    --file Dockerfile \
    .

if [ $? -eq 0 ]; then
    echo "Successfully built the container"
else
    echo "Failed to build the container"
fi

read -p "Publish container? " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    docker push "lrzcc-api/lrzcc:v${VERSION}"
    docker push "lrzcc-api/lrzcc:latest"
fi
