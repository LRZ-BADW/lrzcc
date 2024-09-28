#!/bin/bash

scripts/init_db.sh
prlimit --pid=$PPID --nofile=10000
