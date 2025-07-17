# STM32 Rust Template

[![Rust](https://github.com/shishir-dey/stm32-rust-template/actions/workflows/rust.yml/badge.svg)](https://github.com/shishir-dey/stm32-rust-template/actions/workflows/rust.yml) [![Dependabot Updates](https://github.com/shishir-dey/stm32-rust-template/actions/workflows/dependabot/dependabot-updates/badge.svg)](https://github.com/shishir-dey/stm32-rust-template/actions/workflows/dependabot/dependabot-updates)

A template repository for bare-metal Rust projects on STM32 microcontrollers.

## Project Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── run.sh
├── STM32F407VGTX_FLASH.x
├── src
│   ├── apps
│   ├── arch
│   │   └── cortex_m4
│   │       ├── nvic.rs
│   │       ├── scb.rs
│   │       └── systick.rs
│   ├── bsp
│   │   └── at24.rs
│   ├── components
│   │   └── data         // Host-testable crate
│   ├── driver
│   │   ├── adc
│   │   ├── can
│   │   ├── dac
│   │   ├── flash
│   │   ├── gpio
│   │   ├── i2c
│   │   │   └── stm32f407.rs
│   │   ├── rtc
│   │   ├── sai
│   │   ├── spi
│   │   ├── timer
│   │   ├── usart
│   │   ├── usb
│   │   │   ├── device.rs
│   │   │   └── host.rs
│   │   └── wwdg
│   ├── main.rs
│   ├── mcu
│   │   └── stm32f407.rs
│   └── utils
└── tests
```

## References

- [CMSIS-Driver specification](https://arm-software.github.io/CMSIS_5/Driver/html/index.html)

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
| `./run.sh build-all-host` | Build all host-testable components. |
| `./run.sh test-all-host` | Test all host-testable components. |
| `./run.sh build-host <pkg>` | Build a specific host component (e.g., `data`). |
| `./run.sh test-host <pkg>` | Test a specific host component (e.g., `data`). |
| `./run.sh build-target` | Build the main application for the target device. |
| `./run.sh run-target` | Build and run the main application on the target device. |
| `./run.sh test-target <mod>` | Run an on-target test suite (e.g., `mod`). |
| `./run.sh objdump [args...]` | View the disassembly of the release binary. |
| `./run.sh nm [args...]`| List the symbols in the release binary. |
