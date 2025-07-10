# STM32 Rust Template

A template repository for bare-metal Rust projects on STM32 microcontrollers.

## Project Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── STM32F407VGTX_FLASH.x
├── src
│   ├── apps
│   │   └── mod.rs
│   ├── arch
│   │   ├── cortex_m4
│   │   │   ├── mod.rs
│   │   │   └── nvic.rs
│   │   └── mod.rs
│   ├── bsp
│   │   └── mod.rs
│   ├── components
│   │   ├── data         // Host-testable crate
│   │   ├── mod.rs
│   │   └── storage
│   │       └── mod.rs
│   ├── driver
│   │   ├── i2c
│   │   │   ├── mod.rs
│   │   │   └── stm32f407.rs
│   │   └── mod.rs
│   ├── main.rs
│   ├── mcu
│   │   ├── mod.rs
│   │   └── stm32f407.rs
│   └── utils
│       └── mod.rs
└── tests
    └── mod.rs
```

## Setup

Before you begin, you need to install a few tools.

```bash
# For flashing and debugging the application on the target
cargo install probe-run

# For inspecting the compiled binary
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

You'll also need to configure the runner in `.cargo/config.toml`. Create this file if it doesn't exist, and add the following:

```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = "probe-run --chip STM32F407VGTx" # Replace with your target chip
```

## Commands

| Command | Description |
| :--- | :--- |
| `cargo build` | Compile the project. |
| `cargo build --release` | Compile the project in release mode for optimization. |
| `cargo run --release` | Build and run the application on the target device in release mode. |
| `cargo test-target` | Run the on-target test suite. |
| `cargo objdump --release -- -d` | View the disassembly of the release binary. |
| `cargo nm --release`| List the symbols in the release binary. |
