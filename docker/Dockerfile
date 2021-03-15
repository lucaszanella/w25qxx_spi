FROM balenalib/raspberry-pi-debian:latest

WORKDIR /home/project

RUN export DEBIAN_FRONTEND=noninteractive && apt-get update\
    && apt-get install -y build-essential curl autoconf git m4 iproute2 clang

RUN curl --proto '=https' --tlsv1.2 -o rust.sh https://sh.rustup.rs\
    && /bin/bash rust.sh -y


ENV PATH="/root/.cargo/bin:/usr/local/cmake/bin:${PATH}"
