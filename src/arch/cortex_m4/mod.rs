//! # Cortex-M4 Architecture Support
//!
//! Provides low-level system functionality and register access for the
//! ARM Cortex-M4 processor core used in STM32 microcontrollers.
//!
//! This module includes implementations for the Nested Vectored Interrupt
//! Controller (NVIC), System Control Block (SCB), and SysTick timer.
pub mod nvic;
pub mod scb;
pub mod systick;
