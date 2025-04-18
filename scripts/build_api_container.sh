#!/bin/bash

VERSION=0.7.0

docker build \
    --tag "gierens/lrzcc:v${VERSION}" \
    --tag "gierens/lrzcc:latest" \
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
    docker push "gierens/lrzcc:v${VERSION}"
    docker push "gierens/lrzcc:latest"
fi
