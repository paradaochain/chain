FROM paradao/base:v0.1.0 as base

FROM docker.io/library/ubuntu:20.04 as collator

RUN apt-get update && apt-get install jq curl netcat -y && \
    curl -sSo /wait-for-it.sh https://raw.githubusercontent.com/vishnubob/wait-for-it/master/wait-for-it.sh && \
    chmod +x /wait-for-it.sh

COPY --from=base \
    /paradao/target/release/parachain-collator /usr/bin

COPY ./scripts/docker-run-paradao-collator.sh /usr/bin

RUN mkdir /chainspecs
COPY ./chainspecs/ /chainspecs



