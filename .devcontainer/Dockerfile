# Use a lightweight base image
FROM ubuntu:20.04

# Set environment variables for non-interactive installation
ENV DEBIAN_FRONTEND=noninteractive

# Update and install required packages
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    wget \
    git \
    curl \
    tar \
    gzip \
    # Dependencies for Rust
    pkg-config \
    libssl-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Install Rust and common embedded tools
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install probe-run && \
    cargo install cargo-binutils && \
    rustup component add llvm-tools-preview && \
    rustup target add thumbv7em-none-eabihf

# Verify installations
RUN rustc --version

# Set the working directory inside the container
WORKDIR /workspaces/stm32-rust-template

# Default command for the container
CMD ["bash"]
