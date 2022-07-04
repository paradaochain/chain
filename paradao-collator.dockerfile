FROM paradao/base:v0.1.0 as base

FROM docker.io/library/ubuntu:20.04 as collator

RUN apt-get update
COPY --from=base \
    /paradao/target/release/parachain-collator /usr/bin

COPY ./scripts/docker-run-paradao-collator.sh /usr/bin

RUN mkdir /chainspecs
COPY ./chainspecs /paradao/chainspecs



