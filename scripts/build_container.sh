#!/bin/bash

VERSION=0.3.0

docker build \
    --tag "gierens/lrzcc:v${VERSION}" \
    --tag "gierens/lrzcc:latest" \
    --file Dockerfile \
    .

if [ $? -eq 0 ]; then
    echo "Successfully built the container"
else
    echo "Failed to build the container"
fi

read -p "Publish container? " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]
then
    docker publish "gierens/lrzcc:v${VERSION}"
    docker publish "gierens/lrzcc:latest"
fi
