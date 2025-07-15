//! Atmel 24 Series EEPROM BSP Driver
//!
//! This module provides a Board Support Package (BSP) for Atmel 24 series EEPROM chips
//! using the generic I2C trait interface.
//!
//! Supported EEPROM sizes:
//! - 24C01, 24C02 (1Kbit, 2Kbit)
//! - 24C04 (4Kbit)
//! - 24C08 (8Kbit)
//! - 24C16 (16Kbit)
//! - 24C32 and above (32Kbit+)

use crate::driver::i2c::{Event, I2c, Result};
use core::marker::PhantomData;

/// Default I2C address for 24 series EEPROM (0xA0 >> 1 = 0x50)
pub const EE24_ADDRESS_DEFAULT: u32 = 0x50;

/// EEPROM size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EepromSize {
    /// 1Kbit (128 bytes)
    Kbit1 = 1,
    /// 2Kbit (256 bytes)
    Kbit2 = 2,
    /// 4Kbit (512 bytes)
    Kbit4 = 4,
    /// 8Kbit (1024 bytes)
    Kbit8 = 8,
    /// 16Kbit (2048 bytes)
    Kbit16 = 16,
    /// 32Kbit (4096 bytes)
    Kbit32 = 32,
    /// 64Kbit (8192 bytes)
    Kbit64 = 64,
    /// 128Kbit (16384 bytes)
    Kbit128 = 128,
    /// 256Kbit (32768 bytes)
    Kbit256 = 256,
    /// 512Kbit (65536 bytes)
    Kbit512 = 512,
}

impl EepromSize {
    /// Get the page size for this EEPROM size
    pub fn page_size(&self) -> usize {
        match self {
            EepromSize::Kbit1 | EepromSize::Kbit2 => 8,
            EepromSize::Kbit4 | EepromSize::Kbit8 | EepromSize::Kbit16 => 16,
            _ => 32,
        }
    }

    /// Get the total capacity in bytes
    pub fn capacity(&self) -> usize {
        (*self as usize) * 128 // Convert Kbit to bytes
    }

    /// Check if this EEPROM uses 8-bit addressing
    pub fn uses_8bit_addressing(&self) -> bool {
        match self {
            EepromSize::Kbit1
            | EepromSize::Kbit2
            | EepromSize::Kbit4
            | EepromSize::Kbit8
            | EepromSize::Kbit16 => true,
            _ => false,
        }
    }
}

/// Error types for EEPROM operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EepromError {
    /// I2C communication error
    I2cError(i32),
    /// Invalid address
    InvalidAddress,
    /// Device not ready
    DeviceNotReady,
    /// Write protected
    WriteProtected,
    /// Timeout occurred
    Timeout,
}

impl From<i32> for EepromError {
    fn from(err: i32) -> Self {
        EepromError::I2cError(err)
    }
}

pub type EepromResult<T> = core::result::Result<T, EepromError>;

/// Configuration for the EEPROM driver
pub struct EepromConfig {
    /// I2C address of the EEPROM
    pub address: u32,
    /// Size of the EEPROM
    pub size: EepromSize,
    /// Write protect pin configuration (optional)
    pub wp_pin: Option<fn(bool)>, // Function to control WP pin: true = protected, false = writable
    /// Default timeout in milliseconds
    pub timeout_ms: u32,
}

impl Default for EepromConfig {
    fn default() -> Self {
        Self {
            address: EE24_ADDRESS_DEFAULT,
            size: EepromSize::Kbit32,
            wp_pin: None,
            timeout_ms: 100,
        }
    }
}

/// EEPROM driver handle
pub struct Eeprom<'a, I2C> {
    i2c: I2C,
    config: EepromConfig,
    busy: bool,
    _phantom: PhantomData<&'a ()>,
}

impl<'a, I2C> Eeprom<'a, I2C>
where
    I2C: I2c<'a>,
{
    /// Create a new EEPROM driver instance
    pub fn new(mut i2c: I2C, config: EepromConfig) -> EepromResult<Self> {
        // Initialize I2C with event callback
        i2c.initialize(|event| {
            // Handle I2C events if needed
            if event.contains(Event::BUS_ERROR) {
                // Handle bus error
            }
            if event.contains(Event::TRANSFER_DONE) {
                // Transfer completed
            }
        })
        .map_err(EepromError::from)?;

        let mut eeprom = Self {
            i2c,
            config,
            busy: false,
            _phantom: PhantomData,
        };

        // Test if device is ready
        eeprom.is_device_ready()?;

        Ok(eeprom)
    }

    /// Check if the EEPROM device is ready
    pub fn is_device_ready(&mut self) -> EepromResult<()> {
        // Simple way to check device readiness - try to read status
        let status = self.i2c.get_status();
        if status.bus_error {
            return Err(EepromError::DeviceNotReady);
        }
        Ok(())
    }

    /// Read data from EEPROM
    pub fn read(&mut self, address: u32, data: &mut [u8]) -> EepromResult<()> {
        if self.busy {
            return Err(EepromError::DeviceNotReady);
        }

        if address + data.len() as u32 > self.config.size.capacity() as u32 {
            return Err(EepromError::InvalidAddress);
        }

        self.busy = true;

        let result = match self.config.size {
            EepromSize::Kbit1 | EepromSize::Kbit2 => {
                // 8-bit addressing
                self.read_with_8bit_address(address, data)
            }
            EepromSize::Kbit4 => {
                // 4Kbit uses device address + 8-bit memory address
                self.read_4kbit(address, data)
            }
            EepromSize::Kbit8 => {
                // 8Kbit uses device address + 8-bit memory address
                self.read_8kbit(address, data)
            }
            EepromSize::Kbit16 => {
                // 16Kbit uses device address + 8-bit memory address
                self.read_16kbit(address, data)
            }
            _ => {
                // 16-bit addressing for larger EEPROMs
                self.read_with_16bit_address(address, data)
            }
        };

        self.busy = false;
        result
    }

    /// Write data to EEPROM
    pub fn write(&mut self, address: u32, data: &[u8]) -> EepromResult<()> {
        if self.busy {
            return Err(EepromError::DeviceNotReady);
        }

        if address + data.len() as u32 > self.config.size.capacity() as u32 {
            return Err(EepromError::InvalidAddress);
        }

        self.busy = true;

        // Disable write protection if configured
        if let Some(wp_control) = self.config.wp_pin {
            wp_control(false);
        }

        let result = self.write_pages(address, data);

        // Re-enable write protection if configured
        if let Some(wp_control) = self.config.wp_pin {
            wp_control(true);
        }

        self.busy = false;
        result
    }

    /// Write data with page boundary handling
    fn write_pages(&mut self, mut address: u32, mut data: &[u8]) -> EepromResult<()> {
        let page_size = self.config.size.page_size();

        while !data.is_empty() {
            // Calculate how many bytes we can write in this page
            let bytes_to_page_end = page_size - (address as usize % page_size);
            let write_size = core::cmp::min(data.len(), bytes_to_page_end);

            // Write this chunk
            self.write_chunk(address, &data[..write_size])?;

            // Wait for write cycle to complete (typically 5-10ms)
            self.delay_ms(10);

            // Update for next iteration
            address += write_size as u32;
            data = &data[write_size..];
        }

        Ok(())
    }

    /// Write a single chunk (within page boundary)
    fn write_chunk(&mut self, address: u32, data: &[u8]) -> EepromResult<()> {
        match self.config.size {
            EepromSize::Kbit1 | EepromSize::Kbit2 => self.write_with_8bit_address(address, data),
            EepromSize::Kbit4 => self.write_4kbit(address, data),
            EepromSize::Kbit8 => self.write_8kbit(address, data),
            EepromSize::Kbit16 => self.write_16kbit(address, data),
            _ => self.write_with_16bit_address(address, data),
        }
    }

    /// Read with 8-bit addressing
    fn read_with_8bit_address(&mut self, address: u32, data: &mut [u8]) -> EepromResult<()> {
        // For 8-bit addressing, send address as first byte, then read
        let addr_bytes = [address as u8];

        // Send address
        self.i2c
            .master_transmit(self.config.address, &addr_bytes, true)
            .map_err(EepromError::from)?;

        // Read data
        self.i2c
            .master_receive(self.config.address, data, false)
            .map_err(EepromError::from)?;

        Ok(())
    }

    /// Read with 16-bit addressing
    fn read_with_16bit_address(&mut self, address: u32, data: &mut [u8]) -> EepromResult<()> {
        // For 16-bit addressing, send address as two bytes, then read
        let addr_bytes = [(address >> 8) as u8, address as u8];

        // Send address
        self.i2c
            .master_transmit(self.config.address, &addr_bytes, true)
            .map_err(EepromError::from)?;

        // Read data
        self.i2c
            .master_receive(self.config.address, data, false)
            .map_err(EepromError::from)?;

        Ok(())
    }

    /// Read from 4Kbit EEPROM (uses device address bits)
    fn read_4kbit(&mut self, address: u32, data: &mut [u8]) -> EepromResult<()> {
        let device_addr = self.config.address | ((address & 0x0100) >> 7);
        let mem_addr = [address as u8];

        self.i2c
            .master_transmit(device_addr, &mem_addr, true)
            .map_err(EepromError::from)?;

        self.i2c
            .master_receive(device_addr, data, false)
            .map_err(EepromError::from)?;

        Ok(())
    }

    /// Read from 8Kbit EEPROM (uses device address bits)
    fn read_8kbit(&mut self, address: u32, data: &mut [u8]) -> EepromResult<()> {
        let device_addr = self.config.address | ((address & 0x0300) >> 7);
        let mem_addr = [address as u8];

        self.i2c
            .master_transmit(device_addr, &mem_addr, true)
            .map_err(EepromError::from)?;

        self.i2c
            .master_receive(device_addr, data, false)
            .map_err(EepromError::from)?;

        Ok(())
    }

    /// Read from 16Kbit EEPROM (uses device address bits)
    fn read_16kbit(&mut self, address: u32, data: &mut [u8]) -> EepromResult<()> {
        let device_addr = self.config.address | ((address & 0x0700) >> 7);
        let mem_addr = [address as u8];

        self.i2c
            .master_transmit(device_addr, &mem_addr, true)
            .map_err(EepromError::from)?;

        self.i2c
            .master_receive(device_addr, data, false)
            .map_err(EepromError::from)?;

        Ok(())
    }

    /// Write with 8-bit addressing
    fn write_with_8bit_address(&mut self, address: u32, data: &[u8]) -> EepromResult<()> {
        let mut buffer = [0u8; 256]; // Max page size + address
        buffer[0] = address as u8;
        buffer[1..=data.len()].copy_from_slice(data);

        self.i2c
            .master_transmit(self.config.address, &buffer[..=data.len()], false)
            .map_err(EepromError::from)?;

        Ok(())
    }

    /// Write with 16-bit addressing
    fn write_with_16bit_address(&mut self, address: u32, data: &[u8]) -> EepromResult<()> {
        let mut buffer = [0u8; 256]; // Max page size + address
        buffer[0] = (address >> 8) as u8;
        buffer[1] = address as u8;
        buffer[2..2 + data.len()].copy_from_slice(data);

        self.i2c
            .master_transmit(self.config.address, &buffer[..2 + data.len()], false)
            .map_err(EepromError::from)?;

        Ok(())
    }

    /// Write to 4Kbit EEPROM
    fn write_4kbit(&mut self, address: u32, data: &[u8]) -> EepromResult<()> {
        let device_addr = self.config.address | ((address & 0x0100) >> 7);
        let mut buffer = [0u8; 256];
        buffer[0] = address as u8;
        buffer[1..=data.len()].copy_from_slice(data);

        self.i2c
            .master_transmit(device_addr, &buffer[..=data.len()], false)
            .map_err(EepromError::from)?;

        Ok(())
    }

    /// Write to 8Kbit EEPROM
    fn write_8kbit(&mut self, address: u32, data: &[u8]) -> EepromResult<()> {
        let device_addr = self.config.address | ((address & 0x0300) >> 7);
        let mut buffer = [0u8; 256];
        buffer[0] = address as u8;
        buffer[1..=data.len()].copy_from_slice(data);

        self.i2c
            .master_transmit(device_addr, &buffer[..=data.len()], false)
            .map_err(EepromError::from)?;

        Ok(())
    }

    /// Write to 16Kbit EEPROM
    fn write_16kbit(&mut self, address: u32, data: &[u8]) -> EepromResult<()> {
        let device_addr = self.config.address | ((address & 0x0700) >> 7);
        let mut buffer = [0u8; 256];
        buffer[0] = address as u8;
        buffer[1..=data.len()].copy_from_slice(data);

        self.i2c
            .master_transmit(device_addr, &buffer[..=data.len()], false)
            .map_err(EepromError::from)?;

        Ok(())
    }

    /// Simple delay function - in real implementation, this would use the system timer
    fn delay_ms(&self, _ms: u32) {
        // TODO: Implement proper delay using system timer
        // For now, this is a placeholder
        for _ in 0..1000 {
            // Simple busy wait - replace with proper delay
        }
    }

    /// Get the EEPROM configuration
    pub fn config(&self) -> &EepromConfig {
        &self.config
    }

    /// Check if the driver is busy
    pub fn is_busy(&self) -> bool {
        self.busy
    }
}

/// Convenience functions for common operations
impl<'a, I2C> Eeprom<'a, I2C>
where
    I2C: I2c<'a>,
{
    /// Read a single byte
    pub fn read_byte(&mut self, address: u32) -> EepromResult<u8> {
        let mut data = [0u8; 1];
        self.read(address, &mut data)?;
        Ok(data[0])
    }

    /// Write a single byte
    pub fn write_byte(&mut self, address: u32, data: u8) -> EepromResult<()> {
        self.write(address, &[data])
    }

    /// Read a u16 value (big-endian)
    pub fn read_u16(&mut self, address: u32) -> EepromResult<u16> {
        let mut data = [0u8; 2];
        self.read(address, &mut data)?;
        Ok(u16::from_be_bytes(data))
    }

    /// Write a u16 value (big-endian)
    pub fn write_u16(&mut self, address: u32, data: u16) -> EepromResult<()> {
        self.write(address, &data.to_be_bytes())
    }

    /// Read a u32 value (big-endian)
    pub fn read_u32(&mut self, address: u32) -> EepromResult<u32> {
        let mut data = [0u8; 4];
        self.read(address, &mut data)?;
        Ok(u32::from_be_bytes(data))
    }

    /// Write a u32 value (big-endian)
    pub fn write_u32(&mut self, address: u32, data: u32) -> EepromResult<()> {
        self.write(address, &data.to_be_bytes())
    }

    /// Clear (fill with 0xFF) a range of EEPROM
    pub fn clear_range(&mut self, start_address: u32, length: u32) -> EepromResult<()> {
        let page_size = self.config.size.page_size() as u32;
        let mut address = start_address;
        let mut remaining = length;

        while remaining > 0 {
            let write_size = core::cmp::min(remaining, page_size);
            let fill_data = [0xFF; 32]; // Max page size

            self.write(address, &fill_data[..write_size as usize])?;

            address += write_size;
            remaining -= write_size;
        }

        Ok(())
    }
}
