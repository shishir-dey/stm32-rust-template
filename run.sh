#!/bin/bash

# List of host-side packages
host_packages=("data" "libiot")

# Test a specific host package: ./run.sh test-host data
test_host() {
    pkg="$1"
    cargo test --manifest-path "src/components/${pkg}/Cargo.toml"
}

# Build a specific host package in release mode: ./run.sh build-host data
build_host() {
    pkg="$1"
    cargo build --manifest-path "src/components/${pkg}/Cargo.toml" --release
}

# Build a specific host package for the embedded target: ./run.sh build-target data
build_target() {
    cargo build --target thumbv7em-none-eabihf --release
}

# Run embedded test: ./run.sh test-target mod
test_target() {
    mod="$1"
    cargo run --target thumbv7em-none-eabihf -- --test "$mod"
}

# Build all host packages
build_all_host() {
    for pkg in "${host_packages[@]}"; do
        echo "🔨 Building $pkg"
        cargo build --manifest-path "src/components/${pkg}/Cargo.toml" --release
    done
}

# Test all host packages
test_all_host() {
    for pkg in "${host_packages[@]}"; do
        echo "🧪 Testing $pkg"
        cargo test --manifest-path "src/components/${pkg}/Cargo.toml"
    done
}

# Run the main application on the target
run_target() {
    cargo run --target thumbv7em-none-eabihf --release
}

# View the disassembly of the release binary
objdump() {
    cargo objdump --release -- "$@"
}

# List symbols in the release binary
nm() {
    cargo nm --release -- "$@"
}

usage() {
    echo "Usage:"
    echo "  $0 test-host <pkg>"
    echo "  $0 build-host <pkg>"
    echo "  $0 build-target <pkg>"
    echo "  $0 test-target <mod>"
    echo "  $0 build-all-host"
    echo "  $0 test-all-host"
    echo "  $0 run-target"
    echo "  $0 objdump [args...]"
    echo "  $0 nm [args...]"
    exit 1
}

if [ $# -lt 1 ]; then
    usage
fi

cmd="$1"
shift

case "$cmd" in
    test-host)
        [ $# -eq 1 ] || usage
        test_host "$1"
        ;;
    build-host)
        [ $# -eq 1 ] || usage
        build_host "$1"
        ;;
    build-target)
        [ $# -eq 0 ] || usage
        build_target
        ;;
    test-target)
        [ $# -eq 1 ] || usage
        test_target "$1"
        ;;
    build-all-host)
        build_all_host
        ;;
    test-all-host)
        test_all_host
        ;;
    run-target)
        run_target
        ;;
    objdump)
        objdump "$@"
        ;;
    nm)
        nm "$@"
        ;;
    *)
        usage
        ;;
esac
