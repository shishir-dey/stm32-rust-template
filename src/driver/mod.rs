//! # Peripheral Drivers
//!
//! Hardware abstraction layer providing safe, idiomatic Rust interfaces for
//! STM32 microcontroller peripherals including GPIO, SPI, I2C, UART, timers,
//! and other on-chip components.
//!
//! Drivers are organized by peripheral type and include MCU-specific
//! implementations for different STM32 families.
pub mod adc;
pub mod can;
pub mod dac;
pub mod flash;
pub mod gpio;
pub mod i2c;
pub mod sai;
pub mod spi;
pub mod timer;
pub mod usart;
pub mod usb;
pub mod wwdg;
