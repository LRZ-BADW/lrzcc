#!/bin/bash

docker stop lrzcc-api || true
docker rm lrzcc-api || true
docker run \
    --name lrzcc-api \
    -e APP_OPENSTACK__KEYSTONE_ENDPOINT="${OS_AUTH_URL}" \
    -e APP_OPENSTACK__USERNAME="${OS_USERNAME}" \
    -e APP_OPENSTACK__PASSWORD="${OS_PASSWORD}" \
    -e APP_OPENSTACK__PROJECT="${OS_PROJECT_NAME}" \
    -e APP_OPENSTACK__PROJECT_ID="${OS_PROJECT_ID}" \
    -e APP_OPENSTACK__DOMAIN="${OS_USER_DOMAIN_NAME}" \
    -e APP_OPENSTACK__DOMAIN_ID="${OS_PROJECT_DOMAIN_ID}" \
    -d \
    lrzcc-api

