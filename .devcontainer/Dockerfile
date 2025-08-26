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
    ca-certificates \
    tar \
    gzip \
    # Dependencies for Rust
    pkg-config \
    libssl-dev \
    libudev-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Install Rust using rustup
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable

# Set PATH globally for future containers/sessions
ENV PATH="/root/.cargo/bin:${PATH}"

# Install embedded development tools
RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
RUN cargo install cargo-binutils && \
    rustup component add llvm-tools-preview && \
    rustup target add thumbv7em-none-eabihf

# Verify installations
RUN rustc --version && cargo --version

# Set the working directory inside the container
WORKDIR /workspaces/stm32-rust-template

# Default command for the container
CMD ["bash"]
