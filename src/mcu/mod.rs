//! # MCU-Specific Implementations
//!
//! Contains register definitions, memory maps, and peripheral configurations
//! specific to different STM32 microcontroller families.
//!
//! Each MCU module provides type-safe access to hardware registers and
//! implements the low-level functionality required by the driver layer.
#[cfg(feature = "stm32f407")]
pub mod stm32f407;

#[cfg(feature = "stm32f401")]
pub mod stm32f401;

#[cfg(feature = "stm32f411")]
pub mod stm32f411;

#[cfg(feature = "stm32f103")]
pub mod stm32f103;

#[cfg(feature = "stm32g030")]
pub mod stm32g030;
