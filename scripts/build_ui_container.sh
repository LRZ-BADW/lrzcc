#!/bin/bash

VERSION=0.0.1

docker build \
    --tag "gierens/lrzcc-ui:v${VERSION}" \
    --tag "gierens/lrzcc-ui:latest" \
    --file ui/Dockerfile \
    .

if [ $? -eq 0 ]; then
    echo "Successfully built the container"
else
    echo "Failed to build the container"
fi

read -p "Publish container? " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    docker push "gierens/lrzcc-ui:v${VERSION}"
    docker push "gierens/lrzcc-ui:latest"
fi
