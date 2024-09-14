#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v mariadb)" ]; then
    echo >&2 "Error: mariadb-client is not installed."
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    echo >&2 "Use:"
    echo >&2 "    cargo install sqlx-cli --no-default-features --features postgres"
    echo >&2 "to install it."
    exit 1
fi

DB_HOST="${MARIADB_HOST:=127.0.0.1}"
DB_USER=${MARIADB_USER:=root}
DB_PASSWORD="${MARIADB_PASSWORD:=password}"
DB_NAME="${MARIADB_DB:=lrzcc}"
DB_PORT="${MARIADB_PORT:=3306}"

if [[ -z "${SKIP_DOCKER}" ]]
then
    docker stop lrzcc-db || true
    docker rm lrzcc-db || true
    docker run \
        -e MARIADB_ROOT_PASSWORD="${DB_PASSWORD}" \
        -e MARIADB_DB="${DB_NAME}" \
        -p "${DB_PORT}":3306 \
        --name lrzcc-db \
        -d mariadb:latest
fi

until mariadb -h "${DB_HOST}" -P "${DB_PORT}" -u "${DB_USER}" -p"${DB_PASSWORD}" -D "" -e "QUIT"; do
    >&2 echo "MariaDB is still unavailable - sleeping"
    sleep 1
done

>&2 echo "MariaDB is up and running on ${DB_HOST} on port ${DB_PORT}!"

export DATABASE_URL=mysql://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run

>&2 echo "MariaDB has been migrated, ready to go!"
