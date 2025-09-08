# STM32 Rust Template

[![Rust](https://github.com/shishir-dey/stm32-rust-template/actions/workflows/rust.yml/badge.svg)](https://github.com/shishir-dey/stm32-rust-template/actions/workflows/rust.yml) [![Dependabot Updates](https://github.com/shishir-dey/stm32-rust-template/actions/workflows/dependabot/dependabot-updates/badge.svg)](https://github.com/shishir-dey/stm32-rust-template/actions/workflows/dependabot/dependabot-updates)

A template repository for bare-metal Rust projects on STM32 microcontrollers.

## Project Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── memory.x
├── README.md
├── run.sh
├── src
│   ├── apps
│   ├── arch
│   │   ├── cortex_m4
│   │   │   ├── nvic.rs
│   │   │   ├── scb.rs
│   │   │   └── systick.rs
│   ├── bsp
│   │   ├── at24.rs
│   │   ├── ds1307.rs
│   │   ├── spi_flash.rs
│   ├── components
│   │   ├── data         // Host-testable crate
│   │   └── libiot       // IoT library components
│   ├── driver
│   │   ├── adc
│   │   ├── can
│   │   ├── dac
│   │   ├── flash
│   │   ├── gpio
│   │   ├── i2c
│   │   ├── spi
│   │   ├── usart
│   │   ├── rtc
│   │   ├── sai
│   │   ├── timer
│   │   ├── usb
│   │   │   ├── device.rs
│   │   │   └── host.rs
│   │   └── wwdg
│   ├── main.rs
│   ├── mcu
│   │   ├── stm32f407           // STM32F407-specific modules
│   │   │   ├── adc.rs          // ADC register definitions
│   │   │   ├── gpio.rs         // GPIO register definitions
│   │   │   ├── i2c.rs          // I2C register definitions
│   │   │   ├── mod.rs          // Base addresses and IRQ numbers
│   │   │   ├── rcc.rs          // RCC register definitions
│   │   │   ├── spi.rs          // SPI register definitions
│   │   │   ├── timer.rs        // Timer register definitions
│   │   │   └── usart.rs        // USART register definitions
│   │   ├── stm32f401           // STM32F401-specific modules
│   │   ├── stm32f411           // STM32F411-specific modules
│   │   ├── stm32f103           // STM32F103-specific modules
│   │   └── stm32g030           // STM32G030-specific modules
│   └── utils
└── tests
```

## References

- [CMSIS-Driver specification](https://arm-software.github.io/CMSIS_5/Driver/html/index.html)
- [SPI FLASH Library for STM32](https://github.com/nimaltd/spif)

## Setup

Before you begin, you need to install a few tools.

```bash
# For flashing and debugging the application on the target
cargo install probe-rs

# For inspecting the compiled binary
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

You'll also need to configure the runner in `.cargo/config.toml`. Create this file if it doesn't exist, and add the following:

```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = ["probe-rs", "run", "--chip", "STM32F407VG", "--log-format=oneline"] # Replace with your target chip
```

## Commands

| Command | Description |
| :--- | :--- |
| `./run.sh build-all-host` | Build all host-testable components. |
| `./run.sh test-all-host` | Test all host-testable components. |
| `./run.sh build-host <pkg>` | Build a specific host component (e.g., `data`). |
| `./run.sh test-host <pkg>` | Test a specific host component (e.g., `data`). |
| `./run.sh build-target` | Build the main application for the target device (default: STM32F407). |
| `./run.sh build-target-mcu <mcu>` | Build for specific MCU (stm32f407, stm32f401, stm32f411, stm32f103). |
| `./run.sh run-target` | Build and run the main application on the target device (default: STM32F407). |
| `./run.sh run-target-mcu <mcu>` | Build and run for specific MCU (stm32f407, stm32f401, stm32f411, stm32f103). |
| `./run.sh test-target <mod>` | Run an on-target test suite (e.g., `mod`). |
| `./run.sh objdump [args...]` | View the disassembly of the release binary. |
| `./run.sh nm [args...]`| List the symbols in the release binary. |
