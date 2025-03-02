FROM ubuntu:mantic

RUN apt update && apt upgrade -y && apt install -y git wget curl nano pkg-config \
    build-essential make cmake clang llvm gcc g++ gcc-multilib lld zlib1g-dev libbpf-dev \
    linux-tools-common linux-tools-`uname -r` linux-tools-generic

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN mkdir src && git clone https://gitlab.db.in.tum.de/MatG/portable-decompress.git \
    /src/portable-decompress

WORKDIR /src/portable-decompress
RUN . "$HOME/.cargo/env" && \
    cargo install just libbpf-cargo ripgrep && \
    rustup target add wasm32-unknown-unknown && \
    git fetch && \
    git checkout 9276a6c025b9222f8e43d69c814a77373fb8237a && \
    just init

CMD ["/bin/bash"]
