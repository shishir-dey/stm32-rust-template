#[cfg(feature = "stm32f407")]
extern crate alloc;

use super::{BitOrder, Config, Event, FrameFormat, Mode, Result, SlaveSelectMode, Spi, Status};
use crate::mcu::stm32f407::{self, spi::*};
use crate::utils;
use alloc::boxed::Box;
use core::ops::FnMut;
use core::ptr;

/// Default APB bus clock frequencies in Hz.
/// Update these to match your clock configuration.
const PCLK1_HZ: u32 = 16_000_000; // APB1 (SPI2/3)
const PCLK2_HZ: u32 = 16_000_000; // APB2 (SPI1)

/// A polling-based SPI driver for STM32F407.
pub struct SpiDriver<'a> {
    regs: *mut RegisterBlock,
    _callback: Option<Box<dyn FnMut(Event) + 'a>>,
    config: Config,
    data_count: u32,
}

impl<'a> SpiDriver<'a> {
    pub fn new(spi_base_addr: u32, config: Config) -> Self {
        Self {
            regs: spi_base_addr as *mut RegisterBlock,
            _callback: None,
            config,
            data_count: 0,
        }
    }

    /// Create a new SPI1 driver instance (APB2 clock)
    pub fn new_spi1(config: Config) -> Self {
        Self::new(stm32f407::SPI1_BASEADDR, config)
    }

    /// Create a new SPI2 driver instance (APB1 clock)
    pub fn new_spi2(config: Config) -> Self {
        Self::new(stm32f407::SPI2_BASEADDR, config)
    }

    /// Create a new SPI3 driver instance (APB1 clock)
    pub fn new_spi3(config: Config) -> Self {
        Self::new(stm32f407::SPI3_BASEADDR, config)
    }

    fn regs(&self) -> &mut RegisterBlock {
        unsafe { &mut *self.regs }
    }

    fn is_on_apb2(&self) -> bool {
        let base = self.regs as u32;
        base == stm32f407::SPI1_BASEADDR
    }

    fn get_flag_status(&self, flag_bit: u32) -> bool {
        let sr = unsafe { ptr::read_volatile(&self.regs().sr) };
        utils::read_bit(sr, flag_bit)
    }

    fn wait_txe(&self) {
        while !self.get_flag_status(SR_TXE_POS) {}
    }

    fn wait_rxne(&self) {
        while !self.get_flag_status(SR_RXNE_POS) {}
    }

    fn wait_not_busy(&self) {
        while self.get_flag_status(SR_BSY_POS) {}
    }

    fn clear_ovr_flag(&mut self) {
        // Read DR and SR to clear OVR flag
        let _ = unsafe { ptr::read_volatile(&self.regs().dr) };
        let _ = unsafe { ptr::read_volatile(&self.regs().sr) };
    }

    fn configure_cr1(&mut self) -> u32 {
        let mut cr1 = 0;

        // 1. Configure device mode
        match self.config.mode {
            Mode::Master => {
                cr1 = utils::set_bit(cr1, CR1_MSTR_POS, true);
            }
            Mode::Slave => {
                cr1 = utils::set_bit(cr1, CR1_MSTR_POS, false);
            }
            Mode::Inactive => {
                // Keep peripheral disabled
                return cr1;
            }
        }

        // 2. Configure bus configuration (bidirectional mode)
        match self.config.slave_select_mode {
            SlaveSelectMode::MasterHwInput => {
                // Half-duplex mode (BIDIMODE=1, BIDIOE=0)
                cr1 = utils::set_bit(cr1, CR1_BIDIMODE_POS, true);
                cr1 = utils::set_bit(cr1, CR1_BIDIOE_POS, false);
            }
            SlaveSelectMode::MasterHwOutput => {
                // Full-duplex mode (BIDIMODE=0)
                cr1 = utils::set_bit(cr1, CR1_BIDIMODE_POS, false);
            }
            SlaveSelectMode::MasterSoftware => {
                // Full-duplex mode with software SS control
                cr1 = utils::set_bit(cr1, CR1_BIDIMODE_POS, false);
                cr1 = utils::set_bit(cr1, CR1_SSM_POS, true);
                cr1 = utils::set_bit(cr1, CR1_SSI_POS, true);
            }
            SlaveSelectMode::MasterUnused => {
                // Full-duplex mode
                cr1 = utils::set_bit(cr1, CR1_BIDIMODE_POS, false);
            }
            SlaveSelectMode::SlaveHardware => {
                // Slave mode with hardware SS
                cr1 = utils::set_bit(cr1, CR1_SSM_POS, false);
            }
            SlaveSelectMode::SlaveSoftware => {
                // Slave mode with software SS
                cr1 = utils::set_bit(cr1, CR1_SSM_POS, true);
            }
        }

        // 3. Configure serial clock speed (baud rate)
        let pclk = if self.is_on_apb2() {
            PCLK2_HZ
        } else {
            PCLK1_HZ
        };
        let br_div = match self.config.bus_speed_hz {
            speed if speed >= pclk / 2 => CR1_BR_DIV2,
            speed if speed >= pclk / 4 => CR1_BR_DIV4,
            speed if speed >= pclk / 8 => CR1_BR_DIV8,
            speed if speed >= pclk / 16 => CR1_BR_DIV16,
            speed if speed >= pclk / 32 => CR1_BR_DIV32,
            speed if speed >= pclk / 64 => CR1_BR_DIV64,
            speed if speed >= pclk / 128 => CR1_BR_DIV128,
            _ => CR1_BR_DIV256,
        };
        cr1 = utils::set_bits(cr1, br_div, CR1_BR_POS, CR1_BR_WIDTH);

        // 4. Configure data frame format
        match self.config.data_bits {
            8 => {
                cr1 = utils::set_bit(cr1, CR1_DFF_POS, false);
            }
            16 => {
                cr1 = utils::set_bit(cr1, CR1_DFF_POS, true);
            }
            _ => {
                // Unsupported data bits
                return 0;
            }
        }

        // 5. Configure CPOL and CPHA
        match self.config.frame_format {
            FrameFormat::CPOL0_CPHA0 => {
                cr1 = utils::set_bit(cr1, CR1_CPOL_POS, false);
                cr1 = utils::set_bit(cr1, CR1_CPHA_POS, false);
            }
            FrameFormat::CPOL0_CPHA1 => {
                cr1 = utils::set_bit(cr1, CR1_CPOL_POS, false);
                cr1 = utils::set_bit(cr1, CR1_CPHA_POS, true);
            }
            FrameFormat::CPOL1_CPHA0 => {
                cr1 = utils::set_bit(cr1, CR1_CPOL_POS, true);
                cr1 = utils::set_bit(cr1, CR1_CPHA_POS, false);
            }
            FrameFormat::CPOL1_CPHA1 => {
                cr1 = utils::set_bit(cr1, CR1_CPOL_POS, true);
                cr1 = utils::set_bit(cr1, CR1_CPHA_POS, true);
            }
            FrameFormat::TI_SSI => {
                // TI frame format
                cr1 = utils::set_bit(cr1, CR1_CPOL_POS, false);
                cr1 = utils::set_bit(cr1, CR1_CPHA_POS, false);
            }
            FrameFormat::Microwire => {
                // Microwire frame format
                cr1 = utils::set_bit(cr1, CR1_CPOL_POS, false);
                cr1 = utils::set_bit(cr1, CR1_CPHA_POS, false);
            }
        }

        // 6. Configure bit order
        match self.config.bit_order {
            BitOrder::LSB_MSB => {
                cr1 = utils::set_bit(cr1, CR1_LSBFIRST_POS, true);
            }
            BitOrder::MSB_LSB => {
                cr1 = utils::set_bit(cr1, CR1_LSBFIRST_POS, false);
            }
        }

        cr1
    }

    fn configure_cr2(&mut self) -> u32 {
        let mut cr2 = 0;

        // Configure SSOE for hardware slave select output
        if matches!(
            self.config.slave_select_mode,
            SlaveSelectMode::MasterHwOutput
        ) {
            cr2 = utils::set_bit(cr2, CR2_SSOE_POS, true);
        }

        // Configure frame format
        match self.config.frame_format {
            FrameFormat::TI_SSI => {
                cr2 = utils::set_bit(cr2, CR2_FRF_POS, true);
            }
            _ => {
                cr2 = utils::set_bit(cr2, CR2_FRF_POS, false);
            }
        }

        cr2
    }
}

impl<'a> Spi<'a> for SpiDriver<'a> {
    fn initialize(&mut self, callback: impl FnMut(Event) + 'a) -> Result<()> {
        self._callback = Some(Box::new(callback));

        // Note: The peripheral clock must be enabled before calling this function.

        // Disable peripheral for configuration
        let mut cr1 = unsafe { ptr::read_volatile(&self.regs().cr1) };
        cr1 = utils::set_bit(cr1, CR1_SPE_POS, false);
        unsafe { ptr::write_volatile(&mut self.regs().cr1, cr1) };

        // Configure CR1 register
        let cr1_config = self.configure_cr1();
        if cr1_config == 0 {
            return Err(-1); // Invalid configuration
        }
        unsafe { ptr::write_volatile(&mut self.regs().cr1, cr1_config) };

        // Configure CR2 register
        let cr2_config = self.configure_cr2();
        unsafe { ptr::write_volatile(&mut self.regs().cr2, cr2_config) };

        // Enable the peripheral
        let mut cr1 = unsafe { ptr::read_volatile(&self.regs().cr1) };
        cr1 = utils::set_bit(cr1, CR1_SPE_POS, true);
        unsafe { ptr::write_volatile(&mut self.regs().cr1, cr1) };

        Ok(())
    }

    fn uninitialize(&mut self) -> Result<()> {
        let mut cr1 = unsafe { ptr::read_volatile(&self.regs().cr1) };
        cr1 = utils::set_bit(cr1, CR1_SPE_POS, false);
        unsafe { ptr::write_volatile(&mut self.regs().cr1, cr1) };
        self._callback = None;
        Ok(())
    }

    fn configure(&mut self, config: &Config) -> Result<()> {
        self.config = config.clone();

        // Re-initialize with new configuration
        if let Some(callback) = self._callback.take() {
            self.initialize(callback)?;
        }

        Ok(())
    }

    fn send(&mut self, data: &[u8]) -> Result<()> {
        self.data_count = 0;

        for &byte in data {
            self.wait_txe();
            unsafe { ptr::write_volatile(&mut self.regs().dr, byte as u32) };
            self.data_count += 1;
        }

        // Wait for transmission to complete
        self.wait_not_busy();

        if let Some(cb) = &mut self._callback {
            cb(Event::TRANSFER_COMPLETE);
        }

        Ok(())
    }

    fn receive(&mut self, data: &mut [u8]) -> Result<()> {
        self.data_count = 0;

        // For receive-only mode, we need to send dummy data
        for b in data.iter_mut() {
            self.wait_txe();
            unsafe { ptr::write_volatile(&mut self.regs().dr, 0xFF) }; // Send dummy data

            self.wait_rxne();
            *b = unsafe { ptr::read_volatile(&self.regs().dr) as u8 };
            self.data_count += 1;
        }

        if let Some(cb) = &mut self._callback {
            cb(Event::TRANSFER_COMPLETE);
        }

        Ok(())
    }

    fn transfer(&mut self, data_out: &[u8], data_in: &mut [u8]) -> Result<()> {
        if data_in.len() != data_out.len() {
            return Err(-1);
        }

        self.data_count = 0;

        for (i, &byte) in data_out.iter().enumerate() {
            self.wait_txe();
            unsafe { ptr::write_volatile(&mut self.regs().dr, byte as u32) };

            self.wait_rxne();
            data_in[i] = unsafe { ptr::read_volatile(&self.regs().dr) as u8 };
            self.data_count += 1;
        }

        // Wait for transmission to complete
        self.wait_not_busy();

        if let Some(cb) = &mut self._callback {
            cb(Event::TRANSFER_COMPLETE);
        }

        Ok(())
    }

    fn get_data_count(&self) -> u32 {
        self.data_count
    }

    fn get_status(&self) -> Status {
        let sr = unsafe { ptr::read_volatile(&self.regs().sr) };

        Status {
            busy: utils::read_bit(sr, SR_BSY_POS),
            data_lost: utils::read_bit(sr, SR_OVR_POS),
            mode_fault: utils::read_bit(sr, SR_MODF_POS),
        }
    }

    fn control_slave_select(&mut self, active: bool) -> Result<()> {
        // Only applicable for software slave select mode
        if matches!(
            self.config.slave_select_mode,
            SlaveSelectMode::MasterSoftware
        ) {
            let mut cr1 = unsafe { ptr::read_volatile(&self.regs().cr1) };
            cr1 = utils::set_bit(cr1, CR1_SSI_POS, active);
            unsafe { ptr::write_volatile(&mut self.regs().cr1, cr1) };
            Ok(())
        } else {
            Err(-2) // Not supported in current mode
        }
    }
}
