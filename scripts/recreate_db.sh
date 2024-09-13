#!/bin/bash

mariadb --defaults-file=.mariadb.cnf -e "DROP DATABASE IF EXISTS lrzcc"
mariadb --defaults-file=.mariadb.cnf -D "" -e "CREATE DATABASE IF NOT EXISTS lrzcc"
sqlx migrate run
