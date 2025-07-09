//! This file requires the `bitflags` crate.
//! Please add `bitflags = "2.5.0"` to your `Cargo.toml`.

#![allow(dead_code)]

use bitflags::bitflags;
use core::ops::FnMut;

/// Defines the possible bus speeds for I2C communication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusSpeed {
    /// Standard Speed (100kHz)
    Standard,
    /// Fast Speed (400kHz)
    Fast,
    /// Fast+ Speed (1MHz)
    FastPlus,
    /// High Speed (3.4MHz)
    High,
}

/// Represents the status of the I2C peripheral.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Status {
    /// Busy flag
    pub busy: bool,
    /// Mode: false=Slave, true=Master
    pub master_mode: bool,
    /// Direction: false=Transmitter, true=Receiver
    pub receiving: bool,
    /// General Call indication
    pub general_call: bool,
    /// Master lost arbitration
    pub arbitration_lost: bool,
    /// Bus error detected
    pub bus_error: bool,
}

bitflags! {
    /// Represents I2C communication events.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Event: u32 {
        /// Master/Slave Transmit/Receive finished
        const TRANSFER_DONE = (1 << 0);
        /// Master/Slave Transmit/Receive incomplete
        const TRANSFER_INCOMPLETE = (1 << 1);
        /// Addressed as Slave Transmitter
        const SLAVE_TRANSMIT = (1 << 2);
        /// Addressed as Slave Receiver
        const SLAVE_RECEIVE = (1 << 3);
        /// Address not acknowledged
        const ADDRESS_NACK = (1 << 4);
        /// Slave addressed with general call
        const GENERAL_CALL = (1 << 5);
        /// Master lost arbitration
        const ARBITRATION_LOST = (1 << 6);
        /// Bus error detected
        const BUS_ERROR = (1 << 7);
        /// Bus clear finished
        const BUS_CLEARED = (1 << 8);
    }
}

/// A generic error type for the I2C driver, using i32 for error codes.
pub type Error = i32;

/// A specialized Result type for I2C operations.
pub type Result<T> = core::result::Result<T, Error>;

/// A trait that defines a standard interface for an I2C driver.
pub trait I2c<'a> {
    /// Initializes the I2C peripheral.
    ///
    /// The provided callback will be invoked to signal communication events.
    fn initialize(&mut self, callback: impl FnMut(Event) + 'a) -> Result<()>;

    /// De-initializes the I2C peripheral.
    fn uninitialize(&mut self) -> Result<()>;

    /// Transmits data as an I2C master.
    ///
    /// # Arguments
    ///
    /// * `addr` - 7-bit slave address.
    /// * `data` - The data buffer to transmit.
    /// * `xfer_pending` - If true, a STOP condition is not generated after the transfer,
    ///                    allowing for a subsequent transfer to the same or different device.
    fn master_transmit(&mut self, addr: u32, data: &[u8], xfer_pending: bool) -> Result<()>;

    /// Receives data as an I2C master.
    ///
    /// # Arguments
    ///
    /// * `addr` - 7-bit slave address.
    /// * `data` - A mutable buffer to store the received data.
    /// * `xfer_pending` - If true, a repeated START is generated after the transfer,
    ///                    instead of a STOP condition.
    fn master_receive(&mut self, addr: u32, data: &mut [u8], xfer_pending: bool) -> Result<()>;

    /// Transmits data as an I2C slave.
    fn slave_transmit(&mut self, data: &[u8]) -> Result<()>;

    /// Receives data as an I2C slave.
    fn slave_receive(&mut self, data: &mut [u8]) -> Result<()>;

    /// Gets the number of bytes transferred in the last transaction.
    fn get_data_count(&self) -> Result<u32>;

    /// Sets the bus speed for the I2C communication.
    fn set_bus_speed(&mut self, speed: BusSpeed) -> Result<()>;

    /// Sets the slave address for the I2C peripheral when in slave mode.
    fn set_own_address(&mut self, address: u32) -> Result<()>;

    /// Initiates a bus clear operation.
    fn clear_bus(&mut self) -> Result<()>;

    /// Aborts an ongoing I2C transfer.
    fn abort_transfer(&mut self) -> Result<()>;

    /// Gets the current status of the I2C peripheral.
    fn get_status(&self) -> Status;
}

pub mod stm32f407;
