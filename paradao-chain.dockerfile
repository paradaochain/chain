FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /paradao
COPY . /paradao

RUN cargo build --locked --release
