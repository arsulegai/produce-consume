#!/bin/bash

if [[ "clone" == *"${*}"* ]]; then
  git clone https://github.com/arsulegai/sawtooth-sabre
fi

if [[ "checkout" == *"${*}"* ]]; then
  git checkout origin/add_event_wip
fi

cd sawtooth-sabre/

docker build . -f tp/Dockerfile-installed-bionic -t sabre-tp:local
