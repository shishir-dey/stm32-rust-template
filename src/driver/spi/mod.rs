//! This file requires the `bitflags` crate.
//! Please add `bitflags = "2.5.0"` to your `Cargo.toml`.

#![allow(dead_code)]

use bitflags::bitflags;
use core::ops::FnMut;

/// Defines the operational mode of the SPI peripheral.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Inactive,
    Master,
    Slave,
}

/// Defines the SPI frame format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameFormat {
    /// Clock Polarity 0, Clock Phase 0 (default)
    CPOL0_CPHA0,
    /// Clock Polarity 0, Clock Phase 1
    CPOL0_CPHA1,
    /// Clock Polarity 1, Clock Phase 0
    CPOL1_CPHA0,
    /// Clock Polarity 1, Clock Phase 1
    CPOL1_CPHA1,
    /// Texas Instruments Frame Format
    TI_SSI,
    /// National Semiconductor Microwire Frame Format
    Microwire,
}

/// Defines the bit order for data transfers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitOrder {
    /// MSB to LSB (default)
    MSB_LSB,
    /// LSB to MSB
    LSB_MSB,
}

/// Defines the slave select mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlaveSelectMode {
    /// Not used (default)
    MasterUnused,
    /// Software controlled
    MasterSoftware,
    /// Hardware controlled Output
    MasterHwOutput,
    /// Hardware monitored Input
    MasterHwInput,
    /// Hardware monitored (default for slave)
    SlaveHardware,
    /// Software controlled
    SlaveSoftware,
}

/// Represents the status of the SPI peripheral.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Status {
    /// Transmitter/Receiver busy flag
    pub busy: bool,
    /// Data lost: Receive overflow / Transmit underflow
    pub data_lost: bool,
    /// Mode fault detected
    pub mode_fault: bool,
}

bitflags! {
    /// Represents SPI communication events.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Event: u32 {
        /// Data Transfer completed
        const TRANSFER_COMPLETE = (1 << 0);
        /// Data lost: Receive overflow / Transmit underflow
        const DATA_LOST = (1 << 1);
        /// Master Mode Fault (SS deactivated when Master)
        const MODE_FAULT = (1 << 2);
    }
}

/// A generic error type for the SPI driver, using i32 for error codes.
pub type Error = i32;

/// A specialized Result type for SPI operations.
pub type Result<T> = core::result::Result<T, Error>;

/// Holds the configuration for a SPI peripheral.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub mode: Mode,
    pub frame_format: FrameFormat,
    pub bit_order: BitOrder,
    pub slave_select_mode: SlaveSelectMode,
    pub bus_speed_hz: u32,
    pub data_bits: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: Mode::Master,
            frame_format: FrameFormat::CPOL0_CPHA0,
            bit_order: BitOrder::MSB_LSB,
            slave_select_mode: SlaveSelectMode::MasterHwOutput,
            bus_speed_hz: 1_000_000, // 1 MHz
            data_bits: 8,
        }
    }
}

/// A trait that defines a standard interface for a SPI driver.
pub trait Spi<'a> {
    /// Initializes the SPI peripheral.
    fn initialize(&mut self, callback: impl FnMut(Event) + 'a) -> Result<()>;

    /// De-initializes the SPI peripheral.
    fn uninitialize(&mut self) -> Result<()>;

    /// Configures the SPI peripheral.
    fn configure(&mut self, config: &Config) -> Result<()>;

    /// Transmits data over the SPI bus.
    fn send(&mut self, data: &[u8]) -> Result<()>;

    /// Receives data from the SPI bus.
    fn receive(&mut self, data: &mut [u8]) -> Result<()>;

    /// Simultaneously sends and receives data.
    fn transfer(&mut self, data_out: &[u8], data_in: &mut [u8]) -> Result<()>;

    /// Gets the number of bytes transferred in the last transaction.
    fn get_data_count(&self) -> u32;

    /// Gets the current status of the SPI peripheral.
    fn get_status(&self) -> Status;

    /// Controls the slave select line in software-controlled modes.
    fn control_slave_select(&mut self, active: bool) -> Result<()>;
}
