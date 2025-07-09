extern crate alloc;
use super::{BusSpeed, Event, I2c, Result, Status};
use crate::utils;
use alloc::boxed::Box;
use core::ops::FnMut;
use core::ptr;

/// The APB1 peripheral clock frequency in Hz.
/// This value is used for timing calculations in the I2C peripheral.
///
/// # Note
/// This should be updated to match the actual clock configuration of your MCU.
const PCLK1_HZ: u32 = 16_000_000;

mod reg {
    use crate::mcu::stm32f407;

    #[repr(C)]
    pub struct I2cRegs {
        pub cr1: u32,
        pub cr2: u32,
        pub oar1: u32,
        pub oar2: u32,
        pub dr: u32,
        pub sr1: u32,
        pub sr2: u32,
        pub ccr: u32,
        pub trise: u32,
        pub fltr: u32,
    }

    pub const I2C1_ADDR: u32 = stm32f407::I2C1_BASEADDR;
    pub const I2C2_ADDR: u32 = stm32f407::I2C2_BASEADDR;
    pub const I2C3_ADDR: u32 = stm32f407::I2C3_BASEADDR;
}

mod bits {
    // CR1
    pub const CR1_PE: u32 = 0;
    pub const CR1_START: u32 = 8;
    pub const CR1_STOP: u32 = 9;
    pub const CR1_ACK: u32 = 10;
    pub const CR1_SWRST: u32 = 15;

    // SR1
    pub const SR1_SB: u32 = 0;
    pub const SR1_ADDR: u32 = 1;
    pub const SR1_BTF: u32 = 2;
    pub const SR1_RXNE: u32 = 6;
    pub const SR1_TXE: u32 = 7;
    pub const SR1_BERR: u32 = 8;
    pub const SR1_ARLO: u32 = 9;

    // SR2
    pub const SR2_MSL: u32 = 0;
    pub const SR2_BUSY: u32 = 1;
    pub const SR2_TRA: u32 = 2;
    pub const SR2_GENCALL: u32 = 4;
}

/// Configuration for the I2C driver.
#[derive(Clone, Copy)]
pub struct I2cConfig {
    pub bus_speed: BusSpeed,
    pub own_address: u32,
}

impl Default for I2cConfig {
    fn default() -> Self {
        Self {
            bus_speed: BusSpeed::Standard,
            own_address: 0,
        }
    }
}

impl Default for BusSpeed {
    fn default() -> Self {
        BusSpeed::Standard
    }
}

/// A polling-based I2C driver for STM32F407.
pub struct I2cDriver<'a> {
    regs: *mut reg::I2cRegs,
    _callback: Option<Box<dyn FnMut(Event) + 'a>>,
    config: I2cConfig,
    data_count: u32,
}

impl<'a> I2cDriver<'a> {
    pub fn new(i2c_base_addr: u32, config: I2cConfig) -> Self {
        Self {
            regs: i2c_base_addr as *mut reg::I2cRegs,
            _callback: None,
            config,
            data_count: 0,
        }
    }

    fn regs(&self) -> &mut reg::I2cRegs {
        unsafe { &mut *self.regs }
    }

    fn generate_start_condition(&mut self) {
        let mut cr1 = unsafe { ptr::read_volatile(&self.regs().cr1) };
        cr1 = utils::set_bit(cr1, bits::CR1_START, true);
        unsafe { ptr::write_volatile(&mut self.regs().cr1, cr1) };
    }

    fn generate_stop_condition(&mut self) {
        let mut cr1 = unsafe { ptr::read_volatile(&self.regs().cr1) };
        cr1 = utils::set_bit(cr1, bits::CR1_STOP, true);
        unsafe { ptr::write_volatile(&mut self.regs().cr1, cr1) };
    }

    fn execute_address_phase_write(&mut self, slave_addr: u32) {
        let mut addr = slave_addr << 1;
        addr &= !1; // Clear R/W bit for write
        unsafe { ptr::write_volatile(&mut self.regs().dr, addr) };
    }

    fn execute_address_phase_read(&mut self, slave_addr: u32) {
        let mut addr = slave_addr << 1;
        addr |= 1; // Set R/W bit for read
        unsafe { ptr::write_volatile(&mut self.regs().dr, addr) };
    }

    fn get_flag_status(&self, flag_bit: u32) -> bool {
        let sr1 = unsafe { ptr::read_volatile(&self.regs().sr1) };
        utils::read_bit(sr1, flag_bit)
    }

    fn clear_addr_flag(&mut self) {
        let _ = unsafe { ptr::read_volatile(&self.regs().sr1) };
        let _ = unsafe { ptr::read_volatile(&self.regs().sr2) };
    }

    fn manage_acking(&mut self, enable: bool) {
        let mut cr1 = unsafe { ptr::read_volatile(&self.regs().cr1) };
        cr1 = utils::set_bit(cr1, bits::CR1_ACK, enable);
        unsafe { ptr::write_volatile(&mut self.regs().cr1, cr1) };
    }
}

impl<'a> I2c<'a> for I2cDriver<'a> {
    fn initialize(&mut self, callback: impl FnMut(Event) + 'a) -> Result<()> {
        self._callback = Some(Box::new(callback));

        // Note: The peripheral clock must be enabled before calling this function.

        // Disable peripheral for configuration.
        let mut cr1 = unsafe { ptr::read_volatile(&self.regs().cr1) };
        cr1 = utils::set_bit(cr1, bits::CR1_PE, false);
        unsafe { ptr::write_volatile(&mut self.regs().cr1, cr1) };

        // Configure CR1: Ack control - always enable for master.
        cr1 = unsafe { ptr::read_volatile(&self.regs().cr1) };
        cr1 = utils::set_bit(cr1, bits::CR1_ACK, true);
        unsafe { ptr::write_volatile(&mut self.regs().cr1, cr1) };

        // Configure CR2: Peripheral clock frequency.
        let mut cr2 = unsafe { ptr::read_volatile(&self.regs().cr2) };
        let freq_mhz = PCLK1_HZ / 1_000_000;
        cr2 = utils::set_bits(cr2, freq_mhz, 0, 6);
        unsafe { ptr::write_volatile(&mut self.regs().cr2, cr2) };

        // Configure OAR1: Own address.
        let mut oar1 = unsafe { ptr::read_volatile(&self.regs().oar1) };
        oar1 = utils::set_bits(oar1, self.config.own_address, 1, 7);
        oar1 = utils::set_bit(oar1, 14, true); // This bit must be kept at 1.
        unsafe { ptr::write_volatile(&mut self.regs().oar1, oar1) };

        // Configure CCR: Clock control register.
        let ccr_val;
        let mut ccr_reg = 0;
        match self.config.bus_speed {
            BusSpeed::Standard => {
                ccr_val = PCLK1_HZ / (2 * 100_000); // 100kHz
            }
            BusSpeed::Fast => {
                ccr_reg = utils::set_bit(ccr_reg, 15, true); // F/S mode
                // I2C_FM_DUTY_2 = 0
                ccr_val = PCLK1_HZ / (3 * 400_000); // 400kHz
            }
            _ => return Err(-1),
        }
        ccr_reg |= ccr_val & 0xFFF;
        unsafe { ptr::write_volatile(&mut self.regs().ccr, ccr_reg) };

        // Configure TRISE: Rise time.
        let trise_val = match self.config.bus_speed {
            BusSpeed::Standard => (PCLK1_HZ / 1_000_000) + 1,
            BusSpeed::Fast => (PCLK1_HZ / 1_000_000 * 300 / 1000) + 1,
            _ => return Err(-1),
        };
        unsafe { ptr::write_volatile(&mut self.regs().trise, trise_val & 0x3F) };

        // Enable the peripheral.
        cr1 = unsafe { ptr::read_volatile(&self.regs().cr1) };
        cr1 = utils::set_bit(cr1, bits::CR1_PE, true);
        unsafe { ptr::write_volatile(&mut self.regs().cr1, cr1) };

        Ok(())
    }

    fn uninitialize(&mut self) -> Result<()> {
        let mut cr1 = unsafe { ptr::read_volatile(&self.regs().cr1) };
        cr1 = utils::set_bit(cr1, bits::CR1_PE, false);
        unsafe { ptr::write_volatile(&mut self.regs().cr1, cr1) };
        self._callback = None;
        Ok(())
    }

    fn master_transmit(&mut self, addr: u32, data: &[u8], xfer_pending: bool) -> Result<()> {
        self.data_count = 0;
        self.generate_start_condition();
        while !self.get_flag_status(bits::SR1_SB) {}

        self.execute_address_phase_write(addr);
        while !self.get_flag_status(bits::SR1_ADDR) {}
        self.clear_addr_flag();

        for byte in data {
            while !self.get_flag_status(bits::SR1_TXE) {}
            unsafe { ptr::write_volatile(&mut self.regs().dr, *byte as u32) };
            self.data_count += 1;
        }

        while !self.get_flag_status(bits::SR1_TXE) {}
        while !self.get_flag_status(bits::SR1_BTF) {}

        if !xfer_pending {
            self.generate_stop_condition();
        }
        Ok(())
    }

    fn master_receive(&mut self, addr: u32, data: &mut [u8], xfer_pending: bool) -> Result<()> {
        self.data_count = 0;
        let len = data.len();
        if len == 0 {
            return Ok(());
        }

        self.generate_start_condition();
        while !self.get_flag_status(bits::SR1_SB) {}

        self.execute_address_phase_read(addr);
        while !self.get_flag_status(bits::SR1_ADDR) {}

        if len == 1 {
            self.manage_acking(false);
            self.clear_addr_flag();
            if !xfer_pending {
                self.generate_stop_condition();
            }
            while !self.get_flag_status(bits::SR1_RXNE) {}
            data[0] = unsafe { ptr::read_volatile(&self.regs().dr) as u8 };
            self.data_count = 1;
        } else {
            self.clear_addr_flag();
            for i in (1..=len).rev() {
                while !self.get_flag_status(bits::SR1_RXNE) {}
                if i == 2 {
                    self.manage_acking(false);
                    if !xfer_pending {
                        self.generate_stop_condition();
                    }
                }
                data[len - i] = unsafe { ptr::read_volatile(&self.regs().dr) as u8 };
                self.data_count += 1;
            }
        }

        self.manage_acking(true);
        Ok(())
    }

    fn slave_transmit(&mut self, _data: &[u8]) -> Result<()> {
        todo!("Polling slave transmit not implemented")
    }

    fn slave_receive(&mut self, _data: &mut [u8]) -> Result<()> {
        todo!("Polling slave receive not implemented")
    }

    fn get_data_count(&self) -> Result<u32> {
        Ok(self.data_count)
    }

    fn set_bus_speed(&mut self, _speed: BusSpeed) -> Result<()> {
        todo!("Changing bus speed requires re-initialization")
    }

    fn set_own_address(&mut self, _address: u32) -> Result<()> {
        todo!("Changing own address requires re-initialization")
    }

    fn clear_bus(&mut self) -> Result<()> {
        todo!("Bus clear is not implemented")
    }

    fn abort_transfer(&mut self) -> Result<()> {
        self.generate_stop_condition();
        Ok(())
    }

    fn get_status(&self) -> Status {
        let sr1 = unsafe { ptr::read_volatile(&self.regs().sr1) };
        let sr2 = unsafe { ptr::read_volatile(&self.regs().sr2) };

        Status {
            busy: utils::read_bit(sr2, bits::SR2_BUSY),
            master_mode: utils::read_bit(sr2, bits::SR2_MSL),
            receiving: !utils::read_bit(sr2, bits::SR2_TRA),
            general_call: utils::read_bit(sr2, bits::SR2_GENCALL),
            arbitration_lost: utils::read_bit(sr1, bits::SR1_ARLO),
            bus_error: utils::read_bit(sr1, bits::SR1_BERR),
        }
    }
}
