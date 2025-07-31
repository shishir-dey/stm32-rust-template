// I2C peripheral definitions
// Generated from STM32F407 SVD file

use super::{I2C1_BASEADDR, I2C2_BASEADDR, I2C3_BASEADDR, PeripheralAccess};

// I2C Register Block
#[repr(C)]
pub struct RegisterBlock {
    pub cr1: u32,   // RW: Control register 1
    pub cr2: u32,   // RW: Control register 2
    pub oar1: u32,  // RW: Own address register 1
    pub oar2: u32,  // RW: Own address register 2
    pub dr: u32,    // RW: Data register
    pub sr1: u32,   // RW: Status register 1
    pub sr2: u32,   // RO: Status register 2
    pub ccr: u32,   // RW: Clock control register
    pub trise: u32, // RW: TRISE register
    pub fltr: u32,  // RW: FLTR register
}

// I2C peripheral instances
pub struct I2C1;
pub struct I2C2;
pub struct I2C3;

impl PeripheralAccess for I2C1 {
    const BASE_ADDRESS: u32 = I2C1_BASEADDR;
    type RegisterBlock = RegisterBlock;
}

impl PeripheralAccess for I2C2 {
    const BASE_ADDRESS: u32 = I2C2_BASEADDR;
    type RegisterBlock = RegisterBlock;
}

impl PeripheralAccess for I2C3 {
    const BASE_ADDRESS: u32 = I2C3_BASEADDR;
    type RegisterBlock = RegisterBlock;
}

// I2C Register Field Definitions

// CR1 register fields
pub const CR1_SWRST_POS: u32 = 15;
pub const CR1_SWRST_WIDTH: u32 = 1;
pub const CR1_SWRST_MASK: u32 = 0x1 << 15;
// SWRST enumerated values
pub const CR1_SWRST_NOTRESET: u32 = 0 << 15;
pub const CR1_SWRST_RESET: u32 = 1 << 15;

pub const CR1_ALERT_POS: u32 = 13;
pub const CR1_ALERT_WIDTH: u32 = 1;
pub const CR1_ALERT_MASK: u32 = 0x1 << 13;
// ALERT enumerated values
pub const CR1_ALERT_RELEASE: u32 = 0 << 13;
pub const CR1_ALERT_DRIVE: u32 = 1 << 13;

pub const CR1_PEC_POS: u32 = 12;
pub const CR1_PEC_WIDTH: u32 = 1;
pub const CR1_PEC_MASK: u32 = 0x1 << 12;
// PEC enumerated values
pub const CR1_PEC_DISABLED: u32 = 0 << 12;
pub const CR1_PEC_ENABLED: u32 = 1 << 12;

pub const CR1_POS_POS: u32 = 11;
pub const CR1_POS_WIDTH: u32 = 1;
pub const CR1_POS_MASK: u32 = 0x1 << 11;
// POS enumerated values
pub const CR1_POS_CURRENT: u32 = 0 << 11;
pub const CR1_POS_NEXT: u32 = 1 << 11;

pub const CR1_ACK_POS: u32 = 10;
pub const CR1_ACK_WIDTH: u32 = 1;
pub const CR1_ACK_MASK: u32 = 0x1 << 10;
// ACK enumerated values
pub const CR1_ACK_NAK: u32 = 0 << 10;
pub const CR1_ACK_ACK: u32 = 1 << 10;

pub const CR1_STOP_POS: u32 = 9;
pub const CR1_STOP_WIDTH: u32 = 1;
pub const CR1_STOP_MASK: u32 = 0x1 << 9;
// STOP enumerated values
pub const CR1_STOP_NOSTOP: u32 = 0 << 9;
pub const CR1_STOP_STOP: u32 = 1 << 9;

pub const CR1_START_POS: u32 = 8;
pub const CR1_START_WIDTH: u32 = 1;
pub const CR1_START_MASK: u32 = 0x1 << 8;
// START enumerated values
pub const CR1_START_NOSTART: u32 = 0 << 8;
pub const CR1_START_START: u32 = 1 << 8;

pub const CR1_NOSTRETCH_POS: u32 = 7;
pub const CR1_NOSTRETCH_WIDTH: u32 = 1;
pub const CR1_NOSTRETCH_MASK: u32 = 0x1 << 7;
// NOSTRETCH enumerated values
pub const CR1_NOSTRETCH_ENABLED: u32 = 0 << 7;
pub const CR1_NOSTRETCH_DISABLED: u32 = 1 << 7;

pub const CR1_ENGC_POS: u32 = 6;
pub const CR1_ENGC_WIDTH: u32 = 1;
pub const CR1_ENGC_MASK: u32 = 0x1 << 6;
// ENGC enumerated values
pub const CR1_ENGC_DISABLED: u32 = 0 << 6;
pub const CR1_ENGC_ENABLED: u32 = 1 << 6;

pub const CR1_ENPEC_POS: u32 = 5;
pub const CR1_ENPEC_WIDTH: u32 = 1;
pub const CR1_ENPEC_MASK: u32 = 0x1 << 5;
// ENPEC enumerated values
pub const CR1_ENPEC_DISABLED: u32 = 0 << 5;
pub const CR1_ENPEC_ENABLED: u32 = 1 << 5;

pub const CR1_ENARP_POS: u32 = 4;
pub const CR1_ENARP_WIDTH: u32 = 1;
pub const CR1_ENARP_MASK: u32 = 0x1 << 4;
// ENARP enumerated values
pub const CR1_ENARP_DISABLED: u32 = 0 << 4;
pub const CR1_ENARP_ENABLED: u32 = 1 << 4;

pub const CR1_SMBTYPE_POS: u32 = 3;
pub const CR1_SMBTYPE_WIDTH: u32 = 1;
pub const CR1_SMBTYPE_MASK: u32 = 0x1 << 3;
// SMBTYPE enumerated values
pub const CR1_SMBTYPE_DEVICE: u32 = 0 << 3;
pub const CR1_SMBTYPE_HOST: u32 = 1 << 3;

pub const CR1_SMBUS_POS: u32 = 1;
pub const CR1_SMBUS_WIDTH: u32 = 1;
pub const CR1_SMBUS_MASK: u32 = 0x1 << 1;
// SMBUS enumerated values
pub const CR1_SMBUS_I2C: u32 = 0 << 1;
pub const CR1_SMBUS_SMBUS: u32 = 1 << 1;

pub const CR1_PE_POS: u32 = 0;
pub const CR1_PE_WIDTH: u32 = 1;
pub const CR1_PE_MASK: u32 = 0x1 << 0;
// PE enumerated values
pub const CR1_PE_DISABLED: u32 = 0 << 0;
pub const CR1_PE_ENABLED: u32 = 1 << 0;

// CR2 register fields
pub const CR2_LAST_POS: u32 = 12;
pub const CR2_LAST_WIDTH: u32 = 1;
pub const CR2_LAST_MASK: u32 = 0x1 << 12;
// LAST enumerated values
pub const CR2_LAST_NOTLAST: u32 = 0 << 12;
pub const CR2_LAST_LAST: u32 = 1 << 12;

pub const CR2_DMAEN_POS: u32 = 11;
pub const CR2_DMAEN_WIDTH: u32 = 1;
pub const CR2_DMAEN_MASK: u32 = 0x1 << 11;
// DMAEN enumerated values
pub const CR2_DMAEN_DISABLED: u32 = 0 << 11;
pub const CR2_DMAEN_ENABLED: u32 = 1 << 11;

pub const CR2_ITBUFEN_POS: u32 = 10;
pub const CR2_ITBUFEN_WIDTH: u32 = 1;
pub const CR2_ITBUFEN_MASK: u32 = 0x1 << 10;
// ITBUFEN enumerated values
pub const CR2_ITBUFEN_DISABLED: u32 = 0 << 10;
pub const CR2_ITBUFEN_ENABLED: u32 = 1 << 10;

pub const CR2_ITEVTEN_POS: u32 = 9;
pub const CR2_ITEVTEN_WIDTH: u32 = 1;
pub const CR2_ITEVTEN_MASK: u32 = 0x1 << 9;
// ITEVTEN enumerated values
pub const CR2_ITEVTEN_DISABLED: u32 = 0 << 9;
pub const CR2_ITEVTEN_ENABLED: u32 = 1 << 9;

pub const CR2_ITERREN_POS: u32 = 8;
pub const CR2_ITERREN_WIDTH: u32 = 1;
pub const CR2_ITERREN_MASK: u32 = 0x1 << 8;
// ITERREN enumerated values
pub const CR2_ITERREN_DISABLED: u32 = 0 << 8;
pub const CR2_ITERREN_ENABLED: u32 = 1 << 8;

pub const CR2_FREQ_POS: u32 = 0;
pub const CR2_FREQ_WIDTH: u32 = 6;
pub const CR2_FREQ_MASK: u32 = 0x3F << 0;

// OAR1 register fields
pub const OAR1_ADDMODE_POS: u32 = 15;
pub const OAR1_ADDMODE_WIDTH: u32 = 1;
pub const OAR1_ADDMODE_MASK: u32 = 0x1 << 15;
// ADDMODE enumerated values
pub const OAR1_ADDMODE_ADD7: u32 = 0 << 15;
pub const OAR1_ADDMODE_ADD10: u32 = 1 << 15;

pub const OAR1_ADD_POS: u32 = 0;
pub const OAR1_ADD_WIDTH: u32 = 10;
pub const OAR1_ADD_MASK: u32 = 0x3FF << 0;

// OAR2 register fields
pub const OAR2_ADD2_POS: u32 = 1;
pub const OAR2_ADD2_WIDTH: u32 = 7;
pub const OAR2_ADD2_MASK: u32 = 0x7F << 1;

pub const OAR2_ENDUAL_POS: u32 = 0;
pub const OAR2_ENDUAL_WIDTH: u32 = 1;
pub const OAR2_ENDUAL_MASK: u32 = 0x1 << 0;
// ENDUAL enumerated values
pub const OAR2_ENDUAL_SINGLE: u32 = 0 << 0;
pub const OAR2_ENDUAL_DUAL: u32 = 1 << 0;

// DR register fields
pub const DR_DR_POS: u32 = 0;
pub const DR_DR_WIDTH: u32 = 8;
pub const DR_DR_MASK: u32 = 0xFF << 0;

// SR1 register fields
pub const SR1_SMBALERT_POS: u32 = 15;
pub const SR1_SMBALERT_WIDTH: u32 = 1;
pub const SR1_SMBALERT_MASK: u32 = 0x1 << 15;
// SMBALERT enumerated values
pub const SR1_SMBALERT_NOALERT: u32 = 0 << 15;
pub const SR1_SMBALERT_ALERT: u32 = 1 << 15;

pub const SR1_TIMEOUT_POS: u32 = 14;
pub const SR1_TIMEOUT_WIDTH: u32 = 1;
pub const SR1_TIMEOUT_MASK: u32 = 0x1 << 14;
// TIMEOUT enumerated values
pub const SR1_TIMEOUT_NOTIMEOUT: u32 = 0 << 14;
pub const SR1_TIMEOUT_TIMEOUT: u32 = 1 << 14;

pub const SR1_PECERR_POS: u32 = 12;
pub const SR1_PECERR_WIDTH: u32 = 1;
pub const SR1_PECERR_MASK: u32 = 0x1 << 12;
// PECERR enumerated values
pub const SR1_PECERR_NOERROR: u32 = 0 << 12;
pub const SR1_PECERR_ERROR: u32 = 1 << 12;

pub const SR1_OVR_POS: u32 = 11;
pub const SR1_OVR_WIDTH: u32 = 1;
pub const SR1_OVR_MASK: u32 = 0x1 << 11;
// OVR enumerated values
pub const SR1_OVR_NOOVERRUN: u32 = 0 << 11;
pub const SR1_OVR_OVERRUN: u32 = 1 << 11;

pub const SR1_AF_POS: u32 = 10;
pub const SR1_AF_WIDTH: u32 = 1;
pub const SR1_AF_MASK: u32 = 0x1 << 10;
// AF enumerated values
pub const SR1_AF_NOFAILURE: u32 = 0 << 10;
pub const SR1_AF_FAILURE: u32 = 1 << 10;

pub const SR1_ARLO_POS: u32 = 9;
pub const SR1_ARLO_WIDTH: u32 = 1;
pub const SR1_ARLO_MASK: u32 = 0x1 << 9;
// ARLO enumerated values
pub const SR1_ARLO_NOLOST: u32 = 0 << 9;
pub const SR1_ARLO_LOST: u32 = 1 << 9;

pub const SR1_BERR_POS: u32 = 8;
pub const SR1_BERR_WIDTH: u32 = 1;
pub const SR1_BERR_MASK: u32 = 0x1 << 8;
// BERR enumerated values
pub const SR1_BERR_NOERROR: u32 = 0 << 8;
pub const SR1_BERR_ERROR: u32 = 1 << 8;

pub const SR1_TXE_POS: u32 = 7;
pub const SR1_TXE_WIDTH: u32 = 1;
pub const SR1_TXE_MASK: u32 = 0x1 << 7;
// TXE enumerated values
pub const SR1_TXE_NOTEMPTY: u32 = 0 << 7;
pub const SR1_TXE_EMPTY: u32 = 1 << 7;

pub const SR1_RXNE_POS: u32 = 6;
pub const SR1_RXNE_WIDTH: u32 = 1;
pub const SR1_RXNE_MASK: u32 = 0x1 << 6;
// RXNE enumerated values
pub const SR1_RXNE_EMPTY: u32 = 0 << 6;
pub const SR1_RXNE_NOTEMPTY: u32 = 1 << 6;

pub const SR1_STOPF_POS: u32 = 4;
pub const SR1_STOPF_WIDTH: u32 = 1;
pub const SR1_STOPF_MASK: u32 = 0x1 << 4;
// STOPF enumerated values
pub const SR1_STOPF_NOSTOP: u32 = 0 << 4;
pub const SR1_STOPF_STOP: u32 = 1 << 4;

pub const SR1_ADD10_POS: u32 = 3;
pub const SR1_ADD10_WIDTH: u32 = 1;
pub const SR1_ADD10_MASK: u32 = 0x1 << 3;

pub const SR1_BTF_POS: u32 = 2;
pub const SR1_BTF_WIDTH: u32 = 1;
pub const SR1_BTF_MASK: u32 = 0x1 << 2;
// BTF enumerated values
pub const SR1_BTF_NOTFINISHED: u32 = 0 << 2;
pub const SR1_BTF_FINISHED: u32 = 1 << 2;

pub const SR1_ADDR_POS: u32 = 1;
pub const SR1_ADDR_WIDTH: u32 = 1;
pub const SR1_ADDR_MASK: u32 = 0x1 << 1;
// ADDR enumerated values
pub const SR1_ADDR_NOTMATCH: u32 = 0 << 1;
pub const SR1_ADDR_MATCH: u32 = 1 << 1;

pub const SR1_SB_POS: u32 = 0;
pub const SR1_SB_WIDTH: u32 = 1;
pub const SR1_SB_MASK: u32 = 0x1 << 0;
// SB enumerated values
pub const SR1_SB_NOSTART: u32 = 0 << 0;
pub const SR1_SB_START: u32 = 1 << 0;

// SR2 register fields
pub const SR2_PEC_POS: u32 = 8;
pub const SR2_PEC_WIDTH: u32 = 8;
pub const SR2_PEC_MASK: u32 = 0xFF << 8;

pub const SR2_DUALF_POS: u32 = 7;
pub const SR2_DUALF_WIDTH: u32 = 1;
pub const SR2_DUALF_MASK: u32 = 0x1 << 7;

pub const SR2_SMBHOST_POS: u32 = 6;
pub const SR2_SMBHOST_WIDTH: u32 = 1;
pub const SR2_SMBHOST_MASK: u32 = 0x1 << 6;

pub const SR2_SMBDEFAULT_POS: u32 = 5;
pub const SR2_SMBDEFAULT_WIDTH: u32 = 1;
pub const SR2_SMBDEFAULT_MASK: u32 = 0x1 << 5;

pub const SR2_GENCALL_POS: u32 = 4;
pub const SR2_GENCALL_WIDTH: u32 = 1;
pub const SR2_GENCALL_MASK: u32 = 0x1 << 4;

pub const SR2_TRA_POS: u32 = 2;
pub const SR2_TRA_WIDTH: u32 = 1;
pub const SR2_TRA_MASK: u32 = 0x1 << 2;

pub const SR2_BUSY_POS: u32 = 1;
pub const SR2_BUSY_WIDTH: u32 = 1;
pub const SR2_BUSY_MASK: u32 = 0x1 << 1;

pub const SR2_MSL_POS: u32 = 0;
pub const SR2_MSL_WIDTH: u32 = 1;
pub const SR2_MSL_MASK: u32 = 0x1 << 0;

// CCR register fields
pub const CCR_F_S_POS: u32 = 15;
pub const CCR_F_S_WIDTH: u32 = 1;
pub const CCR_F_S_MASK: u32 = 0x1 << 15;
// F_S enumerated values
pub const CCR_F_S_STANDARD: u32 = 0 << 15;
pub const CCR_F_S_FAST: u32 = 1 << 15;

pub const CCR_DUTY_POS: u32 = 14;
pub const CCR_DUTY_WIDTH: u32 = 1;
pub const CCR_DUTY_MASK: u32 = 0x1 << 14;
// DUTY enumerated values
pub const CCR_DUTY_DUTY2_1: u32 = 0 << 14;
pub const CCR_DUTY_DUTY16_9: u32 = 1 << 14;

pub const CCR_CCR_POS: u32 = 0;
pub const CCR_CCR_WIDTH: u32 = 12;
pub const CCR_CCR_MASK: u32 = 0xFFF << 0;

// TRISE register fields
pub const TRISE_TRISE_POS: u32 = 0;
pub const TRISE_TRISE_WIDTH: u32 = 6;
pub const TRISE_TRISE_MASK: u32 = 0x3F << 0;

// Helper functions for I2C
impl RegisterBlock {
    /// Enable I2C peripheral
    pub fn enable(&mut self) {
        self.cr1 |= CR1_PE_MASK;
    }

    /// Disable I2C peripheral
    pub fn disable(&mut self) {
        self.cr1 &= !CR1_PE_MASK;
    }

    /// Generate START condition
    pub fn generate_start(&mut self) {
        self.cr1 |= CR1_START_MASK;
    }

    /// Generate STOP condition
    pub fn generate_stop(&mut self) {
        self.cr1 |= CR1_STOP_MASK;
    }

    /// Enable/disable ACK
    pub fn set_ack(&mut self, enable: bool) {
        if enable {
            self.cr1 |= CR1_ACK_MASK;
        } else {
            self.cr1 &= !CR1_ACK_MASK;
        }
    }

    /// Check if START bit is sent
    pub fn is_start_sent(&self) -> bool {
        (self.sr1 & SR1_SB_MASK) != 0
    }

    /// Check if address is sent
    pub fn is_addr_sent(&self) -> bool {
        (self.sr1 & SR1_ADDR_MASK) != 0
    }

    /// Check if transmit buffer is empty
    pub fn is_tx_empty(&self) -> bool {
        (self.sr1 & SR1_TXE_MASK) != 0
    }

    /// Check if receive buffer is not empty
    pub fn is_rx_not_empty(&self) -> bool {
        (self.sr1 & SR1_RXNE_MASK) != 0
    }

    /// Check if byte transfer is finished
    pub fn is_btf(&self) -> bool {
        (self.sr1 & SR1_BTF_MASK) != 0
    }

    /// Check if bus is busy
    pub fn is_busy(&self) -> bool {
        (self.sr2 & SR2_BUSY_MASK) != 0
    }

    /// Write data
    pub fn write_data(&mut self, data: u8) {
        self.dr = data as u32;
    }

    /// Read data
    pub fn read_data(&self) -> u8 {
        self.dr as u8
    }

    /// Clear ADDR flag (read SR1 then SR2)
    pub fn clear_addr_flag(&self) {
        let _ = self.sr1;
        let _ = self.sr2;
    }

    /// Set I2C clock frequency (in MHz)
    pub fn set_clock_freq(&mut self, freq_mhz: u8) {
        self.cr2 = (self.cr2 & !0x3F) | (freq_mhz as u32 & 0x3F);
    }
}
