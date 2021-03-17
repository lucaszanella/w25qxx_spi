FROM arm32v7/debian:buster-slim

WORKDIR /home/project

RUN export DEBIAN_FRONTEND=noninteractive && apt-get update \
    && apt-get install -y build-essential curl autoconf git m4 iproute2 clang \
    llvm-dev libclang-dev

RUN curl --proto '=https' --tlsv1.2 -o rust.sh https://sh.rustup.rs\
    && /bin/bash rust.sh -y

ENV PATH="/root/.cargo/bin:/usr/local/cmake/bin:${PATH}"
ENV LD_LIBRARY_PATH="/home/project/WiringPi/wiringPi:${LD_LIBRARY_PATH}"
