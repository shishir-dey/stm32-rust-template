//! This file requires the `bitflags` crate.
//! Please add `bitflags = "2.5.0"` to your `Cargo.toml`.

#![allow(dead_code)]

use bitflags::bitflags;
use core::ops::FnMut;

/// Describes a single sector of the flash memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectorInfo {
    /// Sector start address.
    pub start: u32,
    /// Sector end address (start + size - 1).
    pub end: u32,
}

/// Provides information about the flash memory device.
/// It is assumed that this information is static for a given device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlashInfo {
    /// A slice containing the sector layout information.
    /// If `None`, the sectors are of uniform size.
    pub sector_info: Option<&'static [SectorInfo]>,
    /// The total number of sectors.
    pub sector_count: u32,
    /// The size of a single sector in bytes if the sectors are uniform.
    /// If `sector_info` is used, this field is typically 0.
    pub sector_size: u32,
    /// The optimal programming page size in bytes.
    pub page_size: u32,
    /// The smallest programmable unit in bytes.
    pub program_unit: u32,
    /// The value of memory after it has been erased (usually 0xFF).
    pub erased_value: u8,
}

/// Represents the status of the flash memory device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Status {
    /// `true` if a flash operation is in progress.
    pub busy: bool,
    /// `true` if a read, program, or erase operation has failed.
    pub error: bool,
}

bitflags! {
    /// Represents events that can occur during flash operations.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Event: u32 {
        /// The flash memory is ready for a new operation.
        const READY = (1 << 0);
        /// An error occurred during a read, program, or erase operation.
        const ERROR = (1 << 1);
    }
}

/// A generic error type for the flash driver, using i32 for error codes.
pub type Error = i32;

/// A specialized Result type for flash operations.
pub type Result<T> = core::result::Result<T, Error>;

/// A trait that defines a standard interface for a flash memory driver.
pub trait Flash<'a> {
    /// Initializes the flash memory peripheral.
    ///
    /// The provided callback will be invoked to signal flash events.
    fn initialize(&mut self, callback: impl FnMut(Event) + 'a) -> Result<()>;

    /// De-initializes the flash memory peripheral.
    fn uninitialize(&mut self) -> Result<()>;

    /// Reads data from the flash memory.
    ///
    /// # Arguments
    ///
    /// * `addr` - The starting address to read from.
    /// * `data` - A mutable buffer to store the read data.
    fn read_data(&mut self, addr: u32, data: &mut [u8]) -> Result<()>;

    /// Programs data into the flash memory.
    ///
    /// # Arguments
    ///
    /// * `addr` - The starting address to write to.
    /// * `data` - The data to be programmed.
    fn program_data(&mut self, addr: u32, data: &[u8]) -> Result<()>;

    /// Erases a single sector of the flash memory.
    ///
    /// # Arguments
    ///
    /// * `addr` - The address of the sector to be erased. Any address within the sector is usually sufficient.
    fn erase_sector(&mut self, addr: u32) -> Result<()>;

    /// Erases the entire flash memory chip.
    ///
    /// This is a potentially destructive operation and may not be supported by all devices.
    fn erase_chip(&mut self) -> Result<()>;

    /// Gets the current status of the flash memory device.
    fn get_status(&self) -> Status;

    /// Gets information about the flash memory device.
    fn get_info(&self) -> &FlashInfo;
}
