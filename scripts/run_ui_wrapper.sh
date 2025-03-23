#!/bin/bash

django-admin runserver \
    127.0.0.1:8888 \
    --pythonpath=./wrapper \
    --settings=app
