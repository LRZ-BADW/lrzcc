#!/bin/bash

VERSION=0.7.0

docker build \
    --tag "gierens/avina:v${VERSION}" \
    --tag "gierens/avina:latest" \
    --file api/Dockerfile \
    .

if [ $? -eq 0 ]; then
    echo "Successfully built the container"
else
    echo "Failed to build the container"
fi

read -p "Publish container? " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    docker push "gierens/avina:v${VERSION}"
    docker push "gierens/avina:latest"
fi
