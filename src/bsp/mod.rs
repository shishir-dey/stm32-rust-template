//! # Board Support Package (BSP)
//!
//! Provides hardware-specific implementations for external peripherals and
//! board-level components commonly used with STM32 microcontrollers.
//!
//! This module includes drivers for EEPROM, real-time clocks, and flash
//! memory devices that interface with the MCU through standard protocols.
pub mod at24;
pub mod ds1307;
pub mod spi_flash;
