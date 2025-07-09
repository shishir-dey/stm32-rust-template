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

## Usage

### Building the project

To build the project, run:

```bash
cargo build
```

For a release build, use the `--release` flag:

```bash
cargo build --release
```

### Flashing and Debugging

To flash the application onto the microcontroller and start a debug session, you can use tools like `probe-run` or `gdb` with OpenOCD.

First, ensure you have the necessary tools installed. For example, to install `probe-run`:

```bash
cargo install probe-run
```

You'll also need to configure the runner in `.cargo/config.toml`. Create this file if it doesn't exist, and add the following:

```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = "probe-run --chip STM32F407VGTx" # Replace with your target chip
```

Then you can run the application with:

```bash
cargo run --release
```

### Inspecting the compiled code

To inspect the compiled code, you can use `cargo-binutils`.

Install it with:
```bash
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

Then you can for example view the disassembly:
```bash
cargo objdump --release -- -d
```
or the symbol table:
```bash
cargo nm --release
```
