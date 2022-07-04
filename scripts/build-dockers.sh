#!/usr/bin/env bash

set -e

VERSION=$1

#Dex-collator
docker build \
    --file ./paradao-collator.dockerfile \
    --target collator \
    --tag paradao/paradao-collator:"$VERSION" \
    .

# Registrar
docker build \
    --file ./registrar.dockerfile \
    --tag paradao/registrar:"$VERSION" \
    .

