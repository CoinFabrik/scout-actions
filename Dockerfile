FROM rust:latest

SHELL ["/bin/bash", "-c"]

WORKDIR /usr/src/myapp

# Install dependencies
RUN apt-get update && \
    apt-get install -y curl build-essential libssl-dev pkg-config && \
    rm -rf /var/lib/apt/lists/*

# Install Rust tools
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Rust dependencies
RUN cargo install cargo-dylint dylint-link
RUN cargo install mdbook


# Copy source code
COPY . .
RUN cd apps/cargo-scout-audit
RUN cargo install --path ./apps/cargo-scout-audit/
# Display Cargo.toml for debugging
RUN cat ./vesting/Cargo.toml

COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]

