// USART peripheral definitions
// Generated from STM32F407 SVD file

use super::{PeripheralAccess, USART1_BASEADDR, USART2_BASEADDR, USART3_BASEADDR};

// USART Register Block
#[repr(C)]
pub struct RegisterBlock {
    pub sr: u32,   // RO: Status register
    pub dr: u32,   // RW: Data register
    pub brr: u32,  // RW: Baud rate register
    pub cr1: u32,  // RW: Control register 1
    pub cr2: u32,  // RW: Control register 2
    pub cr3: u32,  // RW: Control register 3
    pub gtpr: u32, // RW: Guard time and prescaler register
}

// USART peripheral instances
pub struct USART1;
pub struct USART2;
pub struct USART3;

impl PeripheralAccess for USART1 {
    const BASE_ADDRESS: u32 = USART1_BASEADDR;
    type RegisterBlock = RegisterBlock;
}

impl PeripheralAccess for USART2 {
    const BASE_ADDRESS: u32 = USART2_BASEADDR;
    type RegisterBlock = RegisterBlock;
}

impl PeripheralAccess for USART3 {
    const BASE_ADDRESS: u32 = USART3_BASEADDR;
    type RegisterBlock = RegisterBlock;
}

// USART Register Field Definitions

// SR register fields
pub const SR_CTS_POS: u32 = 9;
pub const SR_CTS_WIDTH: u32 = 1;
pub const SR_CTS_MASK: u32 = 0x1 << 9;

pub const SR_LBD_POS: u32 = 8;
pub const SR_LBD_WIDTH: u32 = 1;
pub const SR_LBD_MASK: u32 = 0x1 << 8;

pub const SR_TXE_POS: u32 = 7;
pub const SR_TXE_WIDTH: u32 = 1;
pub const SR_TXE_MASK: u32 = 0x1 << 7;

pub const SR_TC_POS: u32 = 6;
pub const SR_TC_WIDTH: u32 = 1;
pub const SR_TC_MASK: u32 = 0x1 << 6;

pub const SR_RXNE_POS: u32 = 5;
pub const SR_RXNE_WIDTH: u32 = 1;
pub const SR_RXNE_MASK: u32 = 0x1 << 5;

pub const SR_IDLE_POS: u32 = 4;
pub const SR_IDLE_WIDTH: u32 = 1;
pub const SR_IDLE_MASK: u32 = 0x1 << 4;

pub const SR_ORE_POS: u32 = 3;
pub const SR_ORE_WIDTH: u32 = 1;
pub const SR_ORE_MASK: u32 = 0x1 << 3;

pub const SR_NF_POS: u32 = 2;
pub const SR_NF_WIDTH: u32 = 1;
pub const SR_NF_MASK: u32 = 0x1 << 2;

pub const SR_FE_POS: u32 = 1;
pub const SR_FE_WIDTH: u32 = 1;
pub const SR_FE_MASK: u32 = 0x1 << 1;

pub const SR_PE_POS: u32 = 0;
pub const SR_PE_WIDTH: u32 = 1;
pub const SR_PE_MASK: u32 = 0x1 << 0;

// DR register fields
pub const DR_DR_POS: u32 = 0;
pub const DR_DR_WIDTH: u32 = 9;
pub const DR_DR_MASK: u32 = 0x1FF << 0;

// BRR register fields
pub const BRR_DIV_MANTISSA_POS: u32 = 4;
pub const BRR_DIV_MANTISSA_WIDTH: u32 = 12;
pub const BRR_DIV_MANTISSA_MASK: u32 = 0xFFF << 4;

pub const BRR_DIV_FRACTION_POS: u32 = 0;
pub const BRR_DIV_FRACTION_WIDTH: u32 = 4;
pub const BRR_DIV_FRACTION_MASK: u32 = 0xF << 0;

// CR1 register fields
pub const CR1_OVER8_POS: u32 = 15;
pub const CR1_OVER8_WIDTH: u32 = 1;
pub const CR1_OVER8_MASK: u32 = 0x1 << 15;
// OVER8 enumerated values
pub const CR1_OVER8_OVERSAMPLE16: u32 = 0 << 15;
pub const CR1_OVER8_OVERSAMPLE8: u32 = 1 << 15;

pub const CR1_UE_POS: u32 = 13;
pub const CR1_UE_WIDTH: u32 = 1;
pub const CR1_UE_MASK: u32 = 0x1 << 13;
// UE enumerated values
pub const CR1_UE_DISABLED: u32 = 0 << 13;
pub const CR1_UE_ENABLED: u32 = 1 << 13;

pub const CR1_M_POS: u32 = 12;
pub const CR1_M_WIDTH: u32 = 1;
pub const CR1_M_MASK: u32 = 0x1 << 12;
// M enumerated values
pub const CR1_M_M8: u32 = 0 << 12;
pub const CR1_M_M9: u32 = 1 << 12;

pub const CR1_WAKE_POS: u32 = 11;
pub const CR1_WAKE_WIDTH: u32 = 1;
pub const CR1_WAKE_MASK: u32 = 0x1 << 11;
// WAKE enumerated values
pub const CR1_WAKE_IDLELINE: u32 = 0 << 11;
pub const CR1_WAKE_ADDRESSMARK: u32 = 1 << 11;

pub const CR1_PCE_POS: u32 = 10;
pub const CR1_PCE_WIDTH: u32 = 1;
pub const CR1_PCE_MASK: u32 = 0x1 << 10;
// PCE enumerated values
pub const CR1_PCE_DISABLED: u32 = 0 << 10;
pub const CR1_PCE_ENABLED: u32 = 1 << 10;

pub const CR1_PS_POS: u32 = 9;
pub const CR1_PS_WIDTH: u32 = 1;
pub const CR1_PS_MASK: u32 = 0x1 << 9;
// PS enumerated values
pub const CR1_PS_EVEN: u32 = 0 << 9;
pub const CR1_PS_ODD: u32 = 1 << 9;

pub const CR1_PEIE_POS: u32 = 8;
pub const CR1_PEIE_WIDTH: u32 = 1;
pub const CR1_PEIE_MASK: u32 = 0x1 << 8;
// PEIE enumerated values
pub const CR1_PEIE_DISABLED: u32 = 0 << 8;
pub const CR1_PEIE_ENABLED: u32 = 1 << 8;

pub const CR1_TXEIE_POS: u32 = 7;
pub const CR1_TXEIE_WIDTH: u32 = 1;
pub const CR1_TXEIE_MASK: u32 = 0x1 << 7;
// TXEIE enumerated values
pub const CR1_TXEIE_DISABLED: u32 = 0 << 7;
pub const CR1_TXEIE_ENABLED: u32 = 1 << 7;

pub const CR1_TCIE_POS: u32 = 6;
pub const CR1_TCIE_WIDTH: u32 = 1;
pub const CR1_TCIE_MASK: u32 = 0x1 << 6;
// TCIE enumerated values
pub const CR1_TCIE_DISABLED: u32 = 0 << 6;
pub const CR1_TCIE_ENABLED: u32 = 1 << 6;

pub const CR1_RXNEIE_POS: u32 = 5;
pub const CR1_RXNEIE_WIDTH: u32 = 1;
pub const CR1_RXNEIE_MASK: u32 = 0x1 << 5;
// RXNEIE enumerated values
pub const CR1_RXNEIE_DISABLED: u32 = 0 << 5;
pub const CR1_RXNEIE_ENABLED: u32 = 1 << 5;

pub const CR1_IDLEIE_POS: u32 = 4;
pub const CR1_IDLEIE_WIDTH: u32 = 1;
pub const CR1_IDLEIE_MASK: u32 = 0x1 << 4;
// IDLEIE enumerated values
pub const CR1_IDLEIE_DISABLED: u32 = 0 << 4;
pub const CR1_IDLEIE_ENABLED: u32 = 1 << 4;

pub const CR1_TE_POS: u32 = 3;
pub const CR1_TE_WIDTH: u32 = 1;
pub const CR1_TE_MASK: u32 = 0x1 << 3;
// TE enumerated values
pub const CR1_TE_DISABLED: u32 = 0 << 3;
pub const CR1_TE_ENABLED: u32 = 1 << 3;

pub const CR1_RE_POS: u32 = 2;
pub const CR1_RE_WIDTH: u32 = 1;
pub const CR1_RE_MASK: u32 = 0x1 << 2;
// RE enumerated values
pub const CR1_RE_DISABLED: u32 = 0 << 2;
pub const CR1_RE_ENABLED: u32 = 1 << 2;

pub const CR1_RWU_POS: u32 = 1;
pub const CR1_RWU_WIDTH: u32 = 1;
pub const CR1_RWU_MASK: u32 = 0x1 << 1;
// RWU enumerated values
pub const CR1_RWU_ACTIVE: u32 = 0 << 1;
pub const CR1_RWU_MUTE: u32 = 1 << 1;

pub const CR1_SBK_POS: u32 = 0;
pub const CR1_SBK_WIDTH: u32 = 1;
pub const CR1_SBK_MASK: u32 = 0x1 << 0;
// SBK enumerated values
pub const CR1_SBK_NOBREAK: u32 = 0 << 0;
pub const CR1_SBK_BREAK: u32 = 1 << 0;

// CR2 register fields
pub const CR2_LINEN_POS: u32 = 14;
pub const CR2_LINEN_WIDTH: u32 = 1;
pub const CR2_LINEN_MASK: u32 = 0x1 << 14;
// LINEN enumerated values
pub const CR2_LINEN_DISABLED: u32 = 0 << 14;
pub const CR2_LINEN_ENABLED: u32 = 1 << 14;

pub const CR2_STOP_POS: u32 = 12;
pub const CR2_STOP_WIDTH: u32 = 2;
pub const CR2_STOP_MASK: u32 = 0x3 << 12;
// STOP enumerated values
pub const CR2_STOP_STOP1: u32 = 0 << 12;
pub const CR2_STOP_STOP0P5: u32 = 1 << 12;
pub const CR2_STOP_STOP2: u32 = 2 << 12;
pub const CR2_STOP_STOP1P5: u32 = 3 << 12;

pub const CR2_CLKEN_POS: u32 = 11;
pub const CR2_CLKEN_WIDTH: u32 = 1;
pub const CR2_CLKEN_MASK: u32 = 0x1 << 11;
// CLKEN enumerated values
pub const CR2_CLKEN_DISABLED: u32 = 0 << 11;
pub const CR2_CLKEN_ENABLED: u32 = 1 << 11;

pub const CR2_CPOL_POS: u32 = 10;
pub const CR2_CPOL_WIDTH: u32 = 1;
pub const CR2_CPOL_MASK: u32 = 0x1 << 10;
// CPOL enumerated values
pub const CR2_CPOL_LOW: u32 = 0 << 10;
pub const CR2_CPOL_HIGH: u32 = 1 << 10;

pub const CR2_CPHA_POS: u32 = 9;
pub const CR2_CPHA_WIDTH: u32 = 1;
pub const CR2_CPHA_MASK: u32 = 0x1 << 9;
// CPHA enumerated values
pub const CR2_CPHA_FIRST: u32 = 0 << 9;
pub const CR2_CPHA_SECOND: u32 = 1 << 9;

pub const CR2_LBCL_POS: u32 = 8;
pub const CR2_LBCL_WIDTH: u32 = 1;
pub const CR2_LBCL_MASK: u32 = 0x1 << 8;

pub const CR2_LBDIE_POS: u32 = 6;
pub const CR2_LBDIE_WIDTH: u32 = 1;
pub const CR2_LBDIE_MASK: u32 = 0x1 << 6;
// LBDIE enumerated values
pub const CR2_LBDIE_DISABLED: u32 = 0 << 6;
pub const CR2_LBDIE_ENABLED: u32 = 1 << 6;

pub const CR2_LBDL_POS: u32 = 5;
pub const CR2_LBDL_WIDTH: u32 = 1;
pub const CR2_LBDL_MASK: u32 = 0x1 << 5;
// LBDL enumerated values
pub const CR2_LBDL_LBDL10: u32 = 0 << 5;
pub const CR2_LBDL_LBDL11: u32 = 1 << 5;

pub const CR2_ADD_POS: u32 = 0;
pub const CR2_ADD_WIDTH: u32 = 4;
pub const CR2_ADD_MASK: u32 = 0xF << 0;

// CR3 register fields
pub const CR3_ONEBIT_POS: u32 = 11;
pub const CR3_ONEBIT_WIDTH: u32 = 1;
pub const CR3_ONEBIT_MASK: u32 = 0x1 << 11;
// ONEBIT enumerated values
pub const CR3_ONEBIT_SAMPLE3: u32 = 0 << 11;
pub const CR3_ONEBIT_SAMPLE1: u32 = 1 << 11;

pub const CR3_CTSIE_POS: u32 = 10;
pub const CR3_CTSIE_WIDTH: u32 = 1;
pub const CR3_CTSIE_MASK: u32 = 0x1 << 10;
// CTSIE enumerated values
pub const CR3_CTSIE_DISABLED: u32 = 0 << 10;
pub const CR3_CTSIE_ENABLED: u32 = 1 << 10;

pub const CR3_CTSE_POS: u32 = 9;
pub const CR3_CTSE_WIDTH: u32 = 1;
pub const CR3_CTSE_MASK: u32 = 0x1 << 9;
// CTSE enumerated values
pub const CR3_CTSE_DISABLED: u32 = 0 << 9;
pub const CR3_CTSE_ENABLED: u32 = 1 << 9;

pub const CR3_RTSE_POS: u32 = 8;
pub const CR3_RTSE_WIDTH: u32 = 1;
pub const CR3_RTSE_MASK: u32 = 0x1 << 8;
// RTSE enumerated values
pub const CR3_RTSE_DISABLED: u32 = 0 << 8;
pub const CR3_RTSE_ENABLED: u32 = 1 << 8;

pub const CR3_DMAT_POS: u32 = 7;
pub const CR3_DMAT_WIDTH: u32 = 1;
pub const CR3_DMAT_MASK: u32 = 0x1 << 7;
// DMAT enumerated values
pub const CR3_DMAT_DISABLED: u32 = 0 << 7;
pub const CR3_DMAT_ENABLED: u32 = 1 << 7;

pub const CR3_DMAR_POS: u32 = 6;
pub const CR3_DMAR_WIDTH: u32 = 1;
pub const CR3_DMAR_MASK: u32 = 0x1 << 6;
// DMAR enumerated values
pub const CR3_DMAR_DISABLED: u32 = 0 << 6;
pub const CR3_DMAR_ENABLED: u32 = 1 << 6;

pub const CR3_SCEN_POS: u32 = 5;
pub const CR3_SCEN_WIDTH: u32 = 1;
pub const CR3_SCEN_MASK: u32 = 0x1 << 5;
// SCEN enumerated values
pub const CR3_SCEN_DISABLED: u32 = 0 << 5;
pub const CR3_SCEN_ENABLED: u32 = 1 << 5;

pub const CR3_NACK_POS: u32 = 4;
pub const CR3_NACK_WIDTH: u32 = 1;
pub const CR3_NACK_MASK: u32 = 0x1 << 4;
// NACK enumerated values
pub const CR3_NACK_DISABLED: u32 = 0 << 4;
pub const CR3_NACK_ENABLED: u32 = 1 << 4;

pub const CR3_HDSEL_POS: u32 = 3;
pub const CR3_HDSEL_WIDTH: u32 = 1;
pub const CR3_HDSEL_MASK: u32 = 0x1 << 3;
// HDSEL enumerated values
pub const CR3_HDSEL_FULLDUPLEX: u32 = 0 << 3;
pub const CR3_HDSEL_HALFDUPLEX: u32 = 1 << 3;

pub const CR3_IRLP_POS: u32 = 2;
pub const CR3_IRLP_WIDTH: u32 = 1;
pub const CR3_IRLP_MASK: u32 = 0x1 << 2;
// IRLP enumerated values
pub const CR3_IRLP_NORMAL: u32 = 0 << 2;
pub const CR3_IRLP_LOWPOWER: u32 = 1 << 2;

pub const CR3_IREN_POS: u32 = 1;
pub const CR3_IREN_WIDTH: u32 = 1;
pub const CR3_IREN_MASK: u32 = 0x1 << 1;
// IREN enumerated values
pub const CR3_IREN_DISABLED: u32 = 0 << 1;
pub const CR3_IREN_ENABLED: u32 = 1 << 1;

pub const CR3_EIE_POS: u32 = 0;
pub const CR3_EIE_WIDTH: u32 = 1;
pub const CR3_EIE_MASK: u32 = 0x1 << 0;
// EIE enumerated values
pub const CR3_EIE_DISABLED: u32 = 0 << 0;
pub const CR3_EIE_ENABLED: u32 = 1 << 0;

// GTPR register fields
pub const GTPR_GT_POS: u32 = 8;
pub const GTPR_GT_WIDTH: u32 = 8;
pub const GTPR_GT_MASK: u32 = 0xFF << 8;

pub const GTPR_PSC_POS: u32 = 0;
pub const GTPR_PSC_WIDTH: u32 = 8;
pub const GTPR_PSC_MASK: u32 = 0xFF << 0;

// Helper functions for USART
impl RegisterBlock {
    /// Enable USART
    pub fn enable(&mut self) {
        self.cr1 |= CR1_UE_MASK;
    }

    /// Disable USART
    pub fn disable(&mut self) {
        self.cr1 &= !CR1_UE_MASK;
    }

    /// Enable transmitter
    pub fn enable_tx(&mut self) {
        self.cr1 |= CR1_TE_MASK;
    }

    /// Enable receiver
    pub fn enable_rx(&mut self) {
        self.cr1 |= CR1_RE_MASK;
    }

    /// Set baud rate
    pub fn set_baud_rate(&mut self, brr_value: u32) {
        self.brr = brr_value & 0xFFFF;
    }

    /// Check if transmit buffer is empty
    pub fn is_tx_empty(&self) -> bool {
        (self.sr & SR_TXE_MASK) != 0
    }

    /// Check if data is received
    pub fn is_rx_not_empty(&self) -> bool {
        (self.sr & SR_RXNE_MASK) != 0
    }

    /// Write data
    pub fn write_data(&mut self, data: u8) {
        self.dr = data as u32;
    }

    /// Read data
    pub fn read_data(&self) -> u8 {
        (self.dr & 0xFF) as u8
    }
}
