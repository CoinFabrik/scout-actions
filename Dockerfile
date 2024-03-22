FROM ubuntu:latest

SHELL ["/bin/bash", "-c"]

WORKDIR /usr/src/myapp

# Install dependencies
RUN apt-get update && \
    apt-get install -y curl git build-essential libssl-dev pkg-config && \
    rm -rf /var/lib/apt/lists/*

# Install Rust tools
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN . "$HOME/.cargo/env"
RUN rustup toolchain install nightly-x86_64-unknown-linux-gnu

# Install Rust dependencies
RUN cargo +nightly install cargo-dylint dylint-link
RUN cargo +nightly install mdbook
RUN cargo +nightly install cargo-scout-audit



COPY . .
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x entrypoint.sh
ENTRYPOINT ["/usr/src/myapp/entrypoint.sh", "${TARGET}"]