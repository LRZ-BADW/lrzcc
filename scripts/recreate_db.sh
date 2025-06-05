#!/bin/bash

mariadb --defaults-file=.mariadb.cnf -e "DROP DATABASE IF EXISTS avina"
mariadb --defaults-file=.mariadb.cnf -D "" -e "CREATE DATABASE IF NOT EXISTS avina"
sqlx migrate run
