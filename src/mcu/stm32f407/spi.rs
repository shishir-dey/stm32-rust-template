// SPI peripheral definitions
// Generated from STM32F407 SVD file

use super::{PeripheralAccess, SPI1_BASEADDR, SPI2_BASEADDR, SPI3_BASEADDR};

// SPI Register Block
#[repr(C)]
pub struct RegisterBlock {
    pub cr1: u32,     // RW: control register 1
    pub cr2: u32,     // RW: control register 2
    pub sr: u32,      // RW: status register
    pub dr: u32,      // RW: data register
    pub crcpr: u32,   // RW: CRC polynomial register
    pub rxcrcr: u32,  // RO: RX CRC register
    pub txcrcr: u32,  // RO: TX CRC register
    pub i2scfgr: u32, // RW: I2S configuration register
    pub i2spr: u32,   // RW: I2S prescaler register
}

// SPI peripheral instances
pub struct SPI1;
pub struct SPI2;
pub struct SPI3;

impl PeripheralAccess for SPI1 {
    const BASE_ADDRESS: u32 = SPI1_BASEADDR;
    type RegisterBlock = RegisterBlock;
}

impl PeripheralAccess for SPI2 {
    const BASE_ADDRESS: u32 = SPI2_BASEADDR;
    type RegisterBlock = RegisterBlock;
}

impl PeripheralAccess for SPI3 {
    const BASE_ADDRESS: u32 = SPI3_BASEADDR;
    type RegisterBlock = RegisterBlock;
}

// SPI Register Field Definitions

// CR1 register fields
pub const CR1_BIDIMODE_POS: u32 = 15;
pub const CR1_BIDIMODE_WIDTH: u32 = 1;
pub const CR1_BIDIMODE_MASK: u32 = 0x1 << 15;
// BIDIMODE enumerated values
pub const CR1_BIDIMODE_UNIDIRECTIONAL: u32 = 0 << 15;
pub const CR1_BIDIMODE_BIDIRECTIONAL: u32 = 1 << 15;

pub const CR1_BIDIOE_POS: u32 = 14;
pub const CR1_BIDIOE_WIDTH: u32 = 1;
pub const CR1_BIDIOE_MASK: u32 = 0x1 << 14;
// BIDIOE enumerated values
pub const CR1_BIDIOE_OUTPUTDISABLED: u32 = 0 << 14;
pub const CR1_BIDIOE_OUTPUTENABLED: u32 = 1 << 14;

pub const CR1_CRCEN_POS: u32 = 13;
pub const CR1_CRCEN_WIDTH: u32 = 1;
pub const CR1_CRCEN_MASK: u32 = 0x1 << 13;
// CRCEN enumerated values
pub const CR1_CRCEN_DISABLED: u32 = 0 << 13;
pub const CR1_CRCEN_ENABLED: u32 = 1 << 13;

pub const CR1_CRCNEXT_POS: u32 = 12;
pub const CR1_CRCNEXT_WIDTH: u32 = 1;
pub const CR1_CRCNEXT_MASK: u32 = 0x1 << 12;
// CRCNEXT enumerated values
pub const CR1_CRCNEXT_TXBUFFER: u32 = 0 << 12;
pub const CR1_CRCNEXT_CRC: u32 = 1 << 12;

pub const CR1_DFF_POS: u32 = 11;
pub const CR1_DFF_WIDTH: u32 = 1;
pub const CR1_DFF_MASK: u32 = 0x1 << 11;
// DFF enumerated values
pub const CR1_DFF_EIGHTBIT: u32 = 0 << 11;
pub const CR1_DFF_SIXTEENBIT: u32 = 1 << 11;

pub const CR1_RXONLY_POS: u32 = 10;
pub const CR1_RXONLY_WIDTH: u32 = 1;
pub const CR1_RXONLY_MASK: u32 = 0x1 << 10;
// RXONLY enumerated values
pub const CR1_RXONLY_FULLDUPLEX: u32 = 0 << 10;
pub const CR1_RXONLY_OUTPUTDISABLED: u32 = 1 << 10;

pub const CR1_SSM_POS: u32 = 9;
pub const CR1_SSM_WIDTH: u32 = 1;
pub const CR1_SSM_MASK: u32 = 0x1 << 9;
// SSM enumerated values
pub const CR1_SSM_DISABLED: u32 = 0 << 9;
pub const CR1_SSM_ENABLED: u32 = 1 << 9;

pub const CR1_SSI_POS: u32 = 8;
pub const CR1_SSI_WIDTH: u32 = 1;
pub const CR1_SSI_MASK: u32 = 0x1 << 8;
// SSI enumerated values
pub const CR1_SSI_SLAVESELECTED: u32 = 0 << 8;
pub const CR1_SSI_SLAVENOTSELECTED: u32 = 1 << 8;

pub const CR1_LSBFIRST_POS: u32 = 7;
pub const CR1_LSBFIRST_WIDTH: u32 = 1;
pub const CR1_LSBFIRST_MASK: u32 = 0x1 << 7;
// LSBFIRST enumerated values
pub const CR1_LSBFIRST_MSBFIRST: u32 = 0 << 7;
pub const CR1_LSBFIRST_LSBFIRST: u32 = 1 << 7;

pub const CR1_SPE_POS: u32 = 6;
pub const CR1_SPE_WIDTH: u32 = 1;
pub const CR1_SPE_MASK: u32 = 0x1 << 6;
// SPE enumerated values
pub const CR1_SPE_DISABLED: u32 = 0 << 6;
pub const CR1_SPE_ENABLED: u32 = 1 << 6;

pub const CR1_BR_POS: u32 = 3;
pub const CR1_BR_WIDTH: u32 = 3;
pub const CR1_BR_MASK: u32 = 0x7 << 3;
// BR enumerated values
pub const CR1_BR_DIV2: u32 = 0 << 3;
pub const CR1_BR_DIV4: u32 = 1 << 3;
pub const CR1_BR_DIV8: u32 = 2 << 3;
pub const CR1_BR_DIV16: u32 = 3 << 3;
pub const CR1_BR_DIV32: u32 = 4 << 3;
pub const CR1_BR_DIV64: u32 = 5 << 3;
pub const CR1_BR_DIV128: u32 = 6 << 3;
pub const CR1_BR_DIV256: u32 = 7 << 3;

pub const CR1_MSTR_POS: u32 = 2;
pub const CR1_MSTR_WIDTH: u32 = 1;
pub const CR1_MSTR_MASK: u32 = 0x1 << 2;
// MSTR enumerated values
pub const CR1_MSTR_SLAVE: u32 = 0 << 2;
pub const CR1_MSTR_MASTER: u32 = 1 << 2;

pub const CR1_CPOL_POS: u32 = 1;
pub const CR1_CPOL_WIDTH: u32 = 1;
pub const CR1_CPOL_MASK: u32 = 0x1 << 1;
// CPOL enumerated values
pub const CR1_CPOL_IDLELOW: u32 = 0 << 1;
pub const CR1_CPOL_IDLEHIGH: u32 = 1 << 1;

pub const CR1_CPHA_POS: u32 = 0;
pub const CR1_CPHA_WIDTH: u32 = 1;
pub const CR1_CPHA_MASK: u32 = 0x1 << 0;
// CPHA enumerated values
pub const CR1_CPHA_FIRSTEDGE: u32 = 0 << 0;
pub const CR1_CPHA_SECONDEDGE: u32 = 1 << 0;

// CR2 register fields
pub const CR2_TXEIE_POS: u32 = 7;
pub const CR2_TXEIE_WIDTH: u32 = 1;
pub const CR2_TXEIE_MASK: u32 = 0x1 << 7;
// TXEIE enumerated values
pub const CR2_TXEIE_MASKED: u32 = 0 << 7;
pub const CR2_TXEIE_NOTMASKED: u32 = 1 << 7;

pub const CR2_RXNEIE_POS: u32 = 6;
pub const CR2_RXNEIE_WIDTH: u32 = 1;
pub const CR2_RXNEIE_MASK: u32 = 0x1 << 6;
// RXNEIE enumerated values
pub const CR2_RXNEIE_MASKED: u32 = 0 << 6;
pub const CR2_RXNEIE_NOTMASKED: u32 = 1 << 6;

pub const CR2_ERRIE_POS: u32 = 5;
pub const CR2_ERRIE_WIDTH: u32 = 1;
pub const CR2_ERRIE_MASK: u32 = 0x1 << 5;
// ERRIE enumerated values
pub const CR2_ERRIE_MASKED: u32 = 0 << 5;
pub const CR2_ERRIE_NOTMASKED: u32 = 1 << 5;

pub const CR2_FRF_POS: u32 = 4;
pub const CR2_FRF_WIDTH: u32 = 1;
pub const CR2_FRF_MASK: u32 = 0x1 << 4;
// FRF enumerated values
pub const CR2_FRF_MOTOROLA: u32 = 0 << 4;
pub const CR2_FRF_TI: u32 = 1 << 4;

pub const CR2_SSOE_POS: u32 = 2;
pub const CR2_SSOE_WIDTH: u32 = 1;
pub const CR2_SSOE_MASK: u32 = 0x1 << 2;
// SSOE enumerated values
pub const CR2_SSOE_DISABLED: u32 = 0 << 2;
pub const CR2_SSOE_ENABLED: u32 = 1 << 2;

pub const CR2_TXDMAEN_POS: u32 = 1;
pub const CR2_TXDMAEN_WIDTH: u32 = 1;
pub const CR2_TXDMAEN_MASK: u32 = 0x1 << 1;
// TXDMAEN enumerated values
pub const CR2_TXDMAEN_DISABLED: u32 = 0 << 1;
pub const CR2_TXDMAEN_ENABLED: u32 = 1 << 1;

pub const CR2_RXDMAEN_POS: u32 = 0;
pub const CR2_RXDMAEN_WIDTH: u32 = 1;
pub const CR2_RXDMAEN_MASK: u32 = 0x1 << 0;
// RXDMAEN enumerated values
pub const CR2_RXDMAEN_DISABLED: u32 = 0 << 0;
pub const CR2_RXDMAEN_ENABLED: u32 = 1 << 0;

// SR register fields
pub const SR_FRE_POS: u32 = 8;
pub const SR_FRE_WIDTH: u32 = 1;
pub const SR_FRE_MASK: u32 = 0x1 << 8;
// FRE enumerated values
pub const SR_FRE_NOERROR: u32 = 0 << 8;
pub const SR_FRE_ERROR: u32 = 1 << 8;

pub const SR_BSY_POS: u32 = 7;
pub const SR_BSY_WIDTH: u32 = 1;
pub const SR_BSY_MASK: u32 = 0x1 << 7;
// BSY enumerated values
pub const SR_BSY_NOTBUSY: u32 = 0 << 7;
pub const SR_BSY_BUSY: u32 = 1 << 7;

pub const SR_OVR_POS: u32 = 6;
pub const SR_OVR_WIDTH: u32 = 1;
pub const SR_OVR_MASK: u32 = 0x1 << 6;
// OVR enumerated values
pub const SR_OVR_NOOVERRUN: u32 = 0 << 6;
pub const SR_OVR_OVERRUN: u32 = 1 << 6;

pub const SR_MODF_POS: u32 = 5;
pub const SR_MODF_WIDTH: u32 = 1;
pub const SR_MODF_MASK: u32 = 0x1 << 5;
// MODF enumerated values
pub const SR_MODF_NOFAULT: u32 = 0 << 5;
pub const SR_MODF_FAULT: u32 = 1 << 5;

pub const SR_CRCERR_POS: u32 = 4;
pub const SR_CRCERR_WIDTH: u32 = 1;
pub const SR_CRCERR_MASK: u32 = 0x1 << 4;
// CRCERR enumerated values
pub const SR_CRCERR_MATCH: u32 = 0 << 4;
pub const SR_CRCERR_NOMATCH: u32 = 1 << 4;

pub const SR_UDR_POS: u32 = 3;
pub const SR_UDR_WIDTH: u32 = 1;
pub const SR_UDR_MASK: u32 = 0x1 << 3;
// UDR enumerated values
pub const SR_UDR_NOUNDERRUN: u32 = 0 << 3;
pub const SR_UDR_UNDERRUN: u32 = 1 << 3;

pub const SR_CHSIDE_POS: u32 = 2;
pub const SR_CHSIDE_WIDTH: u32 = 1;
pub const SR_CHSIDE_MASK: u32 = 0x1 << 2;
// CHSIDE enumerated values
pub const SR_CHSIDE_LEFT: u32 = 0 << 2;
pub const SR_CHSIDE_RIGHT: u32 = 1 << 2;

pub const SR_TXE_POS: u32 = 1;
pub const SR_TXE_WIDTH: u32 = 1;
pub const SR_TXE_MASK: u32 = 0x1 << 1;
// TXE enumerated values
pub const SR_TXE_NOTEMPTY: u32 = 0 << 1;
pub const SR_TXE_EMPTY: u32 = 1 << 1;

pub const SR_RXNE_POS: u32 = 0;
pub const SR_RXNE_WIDTH: u32 = 1;
pub const SR_RXNE_MASK: u32 = 0x1 << 0;
// RXNE enumerated values
pub const SR_RXNE_EMPTY: u32 = 0 << 0;
pub const SR_RXNE_NOTEMPTY: u32 = 1 << 0;

// DR register fields
pub const DR_DR_POS: u32 = 0;
pub const DR_DR_WIDTH: u32 = 16;
pub const DR_DR_MASK: u32 = 0xFFFF << 0;

// CRCPR register fields
pub const CRCPR_CRCPOLY_POS: u32 = 0;
pub const CRCPR_CRCPOLY_WIDTH: u32 = 16;
pub const CRCPR_CRCPOLY_MASK: u32 = 0xFFFF << 0;

// RXCRCR register fields
pub const RXCRCR_RXCRC_POS: u32 = 0;
pub const RXCRCR_RXCRC_WIDTH: u32 = 16;
pub const RXCRCR_RXCRC_MASK: u32 = 0xFFFF << 0;

// TXCRCR register fields
pub const TXCRCR_TXCRC_POS: u32 = 0;
pub const TXCRCR_TXCRC_WIDTH: u32 = 16;
pub const TXCRCR_TXCRC_MASK: u32 = 0xFFFF << 0;

// I2SCFGR register fields
pub const I2SCFGR_I2SMOD_POS: u32 = 11;
pub const I2SCFGR_I2SMOD_WIDTH: u32 = 1;
pub const I2SCFGR_I2SMOD_MASK: u32 = 0x1 << 11;
// I2SMOD enumerated values
pub const I2SCFGR_I2SMOD_SPIMODE: u32 = 0 << 11;
pub const I2SCFGR_I2SMOD_I2SMODE: u32 = 1 << 11;

pub const I2SCFGR_I2SE_POS: u32 = 10;
pub const I2SCFGR_I2SE_WIDTH: u32 = 1;
pub const I2SCFGR_I2SE_MASK: u32 = 0x1 << 10;
// I2SE enumerated values
pub const I2SCFGR_I2SE_DISABLED: u32 = 0 << 10;
pub const I2SCFGR_I2SE_ENABLED: u32 = 1 << 10;

pub const I2SCFGR_I2SCFG_POS: u32 = 8;
pub const I2SCFGR_I2SCFG_WIDTH: u32 = 2;
pub const I2SCFGR_I2SCFG_MASK: u32 = 0x3 << 8;
// I2SCFG enumerated values
pub const I2SCFGR_I2SCFG_SLAVETX: u32 = 0 << 8;
pub const I2SCFGR_I2SCFG_SLAVERX: u32 = 1 << 8;
pub const I2SCFGR_I2SCFG_MASTERTX: u32 = 2 << 8;
pub const I2SCFGR_I2SCFG_MASTERRX: u32 = 3 << 8;

pub const I2SCFGR_PCMSYNC_POS: u32 = 7;
pub const I2SCFGR_PCMSYNC_WIDTH: u32 = 1;
pub const I2SCFGR_PCMSYNC_MASK: u32 = 0x1 << 7;
// PCMSYNC enumerated values
pub const I2SCFGR_PCMSYNC_SHORT: u32 = 0 << 7;
pub const I2SCFGR_PCMSYNC_LONG: u32 = 1 << 7;

pub const I2SCFGR_I2SSTD_POS: u32 = 4;
pub const I2SCFGR_I2SSTD_WIDTH: u32 = 2;
pub const I2SCFGR_I2SSTD_MASK: u32 = 0x3 << 4;
// I2SSTD enumerated values
pub const I2SCFGR_I2SSTD_PHILIPS: u32 = 0 << 4;
pub const I2SCFGR_I2SSTD_MSB: u32 = 1 << 4;
pub const I2SCFGR_I2SSTD_LSB: u32 = 2 << 4;
pub const I2SCFGR_I2SSTD_PCM: u32 = 3 << 4;

pub const I2SCFGR_CKPOL_POS: u32 = 3;
pub const I2SCFGR_CKPOL_WIDTH: u32 = 1;
pub const I2SCFGR_CKPOL_MASK: u32 = 0x1 << 3;
// CKPOL enumerated values
pub const I2SCFGR_CKPOL_IDLELOW: u32 = 0 << 3;
pub const I2SCFGR_CKPOL_IDLEHIGH: u32 = 1 << 3;

pub const I2SCFGR_DATLEN_POS: u32 = 1;
pub const I2SCFGR_DATLEN_WIDTH: u32 = 2;
pub const I2SCFGR_DATLEN_MASK: u32 = 0x3 << 1;
// DATLEN enumerated values
pub const I2SCFGR_DATLEN_SIXTEENBIT: u32 = 0 << 1;
pub const I2SCFGR_DATLEN_TWENTYFOURBIT: u32 = 1 << 1;
pub const I2SCFGR_DATLEN_THIRTYTWOBIT: u32 = 2 << 1;

pub const I2SCFGR_CHLEN_POS: u32 = 0;
pub const I2SCFGR_CHLEN_WIDTH: u32 = 1;
pub const I2SCFGR_CHLEN_MASK: u32 = 0x1 << 0;
// CHLEN enumerated values
pub const I2SCFGR_CHLEN_SIXTEENBIT: u32 = 0 << 0;
pub const I2SCFGR_CHLEN_THIRTYTWOBIT: u32 = 1 << 0;

// I2SPR register fields
pub const I2SPR_MCKOE_POS: u32 = 9;
pub const I2SPR_MCKOE_WIDTH: u32 = 1;
pub const I2SPR_MCKOE_MASK: u32 = 0x1 << 9;
// MCKOE enumerated values
pub const I2SPR_MCKOE_DISABLED: u32 = 0 << 9;
pub const I2SPR_MCKOE_ENABLED: u32 = 1 << 9;

pub const I2SPR_ODD_POS: u32 = 8;
pub const I2SPR_ODD_WIDTH: u32 = 1;
pub const I2SPR_ODD_MASK: u32 = 0x1 << 8;
// ODD enumerated values
pub const I2SPR_ODD_EVEN: u32 = 0 << 8;
pub const I2SPR_ODD_ODD: u32 = 1 << 8;

pub const I2SPR_I2SDIV_POS: u32 = 0;
pub const I2SPR_I2SDIV_WIDTH: u32 = 8;
pub const I2SPR_I2SDIV_MASK: u32 = 0xFF << 0;

// SPI Mode enumeration
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SpiMode {
    Mode0 = 0, // CPOL=0, CPHA=0
    Mode1 = 1, // CPOL=0, CPHA=1
    Mode2 = 2, // CPOL=1, CPHA=0
    Mode3 = 3, // CPOL=1, CPHA=1
}

// SPI Baudrate prescaler enumeration
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SpiBaudRate {
    Div2 = 0,
    Div4 = 1,
    Div8 = 2,
    Div16 = 3,
    Div32 = 4,
    Div64 = 5,
    Div128 = 6,
    Div256 = 7,
}

// Helper functions for SPI
impl RegisterBlock {
    /// Enable SPI
    pub fn enable(&mut self) {
        self.cr1 |= CR1_SPE_MASK;
    }

    /// Disable SPI
    pub fn disable(&mut self) {
        self.cr1 &= !CR1_SPE_MASK;
    }

    /// Set SPI mode (CPOL/CPHA)
    pub fn set_mode(&mut self, mode: SpiMode) {
        self.cr1 =
            (self.cr1 & !(CR1_CPOL_MASK | CR1_CPHA_MASK)) | (((mode as u32) & 0x3) << CR1_CPHA_POS);
    }

    /// Set master mode
    pub fn set_master(&mut self) {
        self.cr1 |= CR1_MSTR_MASK;
    }

    /// Set slave mode
    pub fn set_slave(&mut self) {
        self.cr1 &= !CR1_MSTR_MASK;
    }

    /// Set baud rate prescaler
    pub fn set_baud_rate(&mut self, baudrate: SpiBaudRate) {
        self.cr1 = (self.cr1 & !CR1_BR_MASK) | ((baudrate as u32) << CR1_BR_POS);
    }

    /// Check if transmit buffer is empty
    pub fn is_tx_empty(&self) -> bool {
        (self.sr & SR_TXE_MASK) != 0
    }

    /// Check if receive buffer is not empty
    pub fn is_rx_not_empty(&self) -> bool {
        (self.sr & SR_RXNE_MASK) != 0
    }

    /// Check if SPI is busy
    pub fn is_busy(&self) -> bool {
        (self.sr & SR_BSY_MASK) != 0
    }

    /// Write data
    pub fn write_data(&mut self, data: u16) {
        self.dr = data as u32;
    }

    /// Read data
    pub fn read_data(&self) -> u16 {
        self.dr as u16
    }

    /// Transfer a single byte (blocking)
    pub fn transfer_byte(&mut self, data: u8) -> u8 {
        // Wait for TXE
        while !self.is_tx_empty() {}

        // Send data
        self.write_data(data as u16);

        // Wait for RXNE
        while !self.is_rx_not_empty() {}

        // Read received data
        self.read_data() as u8
    }
}
