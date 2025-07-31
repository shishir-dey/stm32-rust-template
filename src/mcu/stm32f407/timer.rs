// Timer peripheral definitions
// Generated from STM32F407 SVD file

use super::{PeripheralAccess, TIM2_BASEADDR, TIM3_BASEADDR, TIM4_BASEADDR, TIM5_BASEADDR};

// Timer Register Block (for general purpose timers TIM2-TIM5)
#[repr(C)]
pub struct RegisterBlock {
    pub cr1: u32,   // RW: control register 1
    pub cr2: u32,   // RW: control register 2
    pub smcr: u32,  // RW: slave mode control register
    pub dier: u32,  // RW: DMA/Interrupt enable register
    pub sr: u32,    // RW: status register
    pub egr: u32,   // WO: event generation register
    pub ccmr1: u32, // RW: capture/compare mode register 1
    pub ccmr2: u32, // RW: capture/compare mode register 2
    pub ccer: u32,  // RW: capture/compare enable register
    pub cnt: u32,   // RW: counter
    pub psc: u32,   // RW: prescaler
    pub arr: u32,   // RW: auto-reload register
    _reserved0: u32,
    pub ccr1: u32, // RW: capture/compare register 1
    pub ccr2: u32, // RW: capture/compare register 2
    pub ccr3: u32, // RW: capture/compare register 3
    pub ccr4: u32, // RW: capture/compare register 4
    _reserved1: u32,
    pub dcr: u32,  // RW: DMA control register
    pub dmar: u32, // RW: DMA address for full transfer
    pub or: u32,   // RW: option register
}

// Timer peripheral instances
pub struct TIM2;
pub struct TIM3;
pub struct TIM4;
pub struct TIM5;

impl PeripheralAccess for TIM2 {
    const BASE_ADDRESS: u32 = TIM2_BASEADDR;
    type RegisterBlock = RegisterBlock;
}

impl PeripheralAccess for TIM3 {
    const BASE_ADDRESS: u32 = TIM3_BASEADDR;
    type RegisterBlock = RegisterBlock;
}

impl PeripheralAccess for TIM4 {
    const BASE_ADDRESS: u32 = TIM4_BASEADDR;
    type RegisterBlock = RegisterBlock;
}

impl PeripheralAccess for TIM5 {
    const BASE_ADDRESS: u32 = TIM5_BASEADDR;
    type RegisterBlock = RegisterBlock;
}

// Timer Register Field Definitions

// CR1 register fields
pub const CR1_CKD_POS: u32 = 8;
pub const CR1_CKD_WIDTH: u32 = 2;
pub const CR1_CKD_MASK: u32 = 0x3 << 8;
// CKD enumerated values
pub const CR1_CKD_DIV1: u32 = 0 << 8;
pub const CR1_CKD_DIV2: u32 = 1 << 8;
pub const CR1_CKD_DIV4: u32 = 2 << 8;

pub const CR1_ARPE_POS: u32 = 7;
pub const CR1_ARPE_WIDTH: u32 = 1;
pub const CR1_ARPE_MASK: u32 = 0x1 << 7;
// ARPE enumerated values
pub const CR1_ARPE_DISABLED: u32 = 0 << 7;
pub const CR1_ARPE_ENABLED: u32 = 1 << 7;

pub const CR1_CMS_POS: u32 = 5;
pub const CR1_CMS_WIDTH: u32 = 2;
pub const CR1_CMS_MASK: u32 = 0x3 << 5;
// CMS enumerated values
pub const CR1_CMS_EDGEALIGNED: u32 = 0 << 5;
pub const CR1_CMS_CENTERALIGNED1: u32 = 1 << 5;
pub const CR1_CMS_CENTERALIGNED2: u32 = 2 << 5;
pub const CR1_CMS_CENTERALIGNED3: u32 = 3 << 5;

pub const CR1_DIR_POS: u32 = 4;
pub const CR1_DIR_WIDTH: u32 = 1;
pub const CR1_DIR_MASK: u32 = 0x1 << 4;
// DIR enumerated values
pub const CR1_DIR_UP: u32 = 0 << 4;
pub const CR1_DIR_DOWN: u32 = 1 << 4;

pub const CR1_OPM_POS: u32 = 3;
pub const CR1_OPM_WIDTH: u32 = 1;
pub const CR1_OPM_MASK: u32 = 0x1 << 3;
// OPM enumerated values
pub const CR1_OPM_DISABLED: u32 = 0 << 3;
pub const CR1_OPM_ENABLED: u32 = 1 << 3;

pub const CR1_URS_POS: u32 = 2;
pub const CR1_URS_WIDTH: u32 = 1;
pub const CR1_URS_MASK: u32 = 0x1 << 2;
// URS enumerated values
pub const CR1_URS_ANYEVENT: u32 = 0 << 2;
pub const CR1_URS_COUNTERONLY: u32 = 1 << 2;

pub const CR1_UDIS_POS: u32 = 1;
pub const CR1_UDIS_WIDTH: u32 = 1;
pub const CR1_UDIS_MASK: u32 = 0x1 << 1;
// UDIS enumerated values
pub const CR1_UDIS_ENABLED: u32 = 0 << 1;
pub const CR1_UDIS_DISABLED: u32 = 1 << 1;

pub const CR1_CEN_POS: u32 = 0;
pub const CR1_CEN_WIDTH: u32 = 1;
pub const CR1_CEN_MASK: u32 = 0x1 << 0;
// CEN enumerated values
pub const CR1_CEN_DISABLED: u32 = 0 << 0;
pub const CR1_CEN_ENABLED: u32 = 1 << 0;

// CR2 register fields
pub const CR2_TI1S_POS: u32 = 7;
pub const CR2_TI1S_WIDTH: u32 = 1;
pub const CR2_TI1S_MASK: u32 = 0x1 << 7;
// TI1S enumerated values
pub const CR2_TI1S_NORMAL: u32 = 0 << 7;
pub const CR2_TI1S_XOR: u32 = 1 << 7;

pub const CR2_MMS_POS: u32 = 4;
pub const CR2_MMS_WIDTH: u32 = 3;
pub const CR2_MMS_MASK: u32 = 0x7 << 4;
// MMS enumerated values
pub const CR2_MMS_RESET: u32 = 0 << 4;
pub const CR2_MMS_ENABLE: u32 = 1 << 4;
pub const CR2_MMS_UPDATE: u32 = 2 << 4;
pub const CR2_MMS_COMPAREPULSE: u32 = 3 << 4;
pub const CR2_MMS_COMPAREOC1: u32 = 4 << 4;
pub const CR2_MMS_COMPAREOC2: u32 = 5 << 4;
pub const CR2_MMS_COMPAREOC3: u32 = 6 << 4;
pub const CR2_MMS_COMPAREOC4: u32 = 7 << 4;

pub const CR2_CCDS_POS: u32 = 3;
pub const CR2_CCDS_WIDTH: u32 = 1;
pub const CR2_CCDS_MASK: u32 = 0x1 << 3;
// CCDS enumerated values
pub const CR2_CCDS_ONCOMPARE: u32 = 0 << 3;
pub const CR2_CCDS_ONUPDATE: u32 = 1 << 3;

// SMCR register fields
pub const SMCR_ETP_POS: u32 = 15;
pub const SMCR_ETP_WIDTH: u32 = 1;
pub const SMCR_ETP_MASK: u32 = 0x1 << 15;
// ETP enumerated values
pub const SMCR_ETP_NOTINVERTED: u32 = 0 << 15;
pub const SMCR_ETP_INVERTED: u32 = 1 << 15;

pub const SMCR_ECE_POS: u32 = 14;
pub const SMCR_ECE_WIDTH: u32 = 1;
pub const SMCR_ECE_MASK: u32 = 0x1 << 14;
// ECE enumerated values
pub const SMCR_ECE_DISABLED: u32 = 0 << 14;
pub const SMCR_ECE_ENABLED: u32 = 1 << 14;

pub const SMCR_ETPS_POS: u32 = 12;
pub const SMCR_ETPS_WIDTH: u32 = 2;
pub const SMCR_ETPS_MASK: u32 = 0x3 << 12;
// ETPS enumerated values
pub const SMCR_ETPS_DIV1: u32 = 0 << 12;
pub const SMCR_ETPS_DIV2: u32 = 1 << 12;
pub const SMCR_ETPS_DIV4: u32 = 2 << 12;
pub const SMCR_ETPS_DIV8: u32 = 3 << 12;

pub const SMCR_ETF_POS: u32 = 8;
pub const SMCR_ETF_WIDTH: u32 = 4;
pub const SMCR_ETF_MASK: u32 = 0xF << 8;
// ETF enumerated values
pub const SMCR_ETF_NOFILTER: u32 = 0 << 8;
pub const SMCR_ETF_FCK_INT_N2: u32 = 1 << 8;
pub const SMCR_ETF_FCK_INT_N4: u32 = 2 << 8;
pub const SMCR_ETF_FCK_INT_N8: u32 = 3 << 8;
pub const SMCR_ETF_FDTS_DIV2_N6: u32 = 4 << 8;
pub const SMCR_ETF_FDTS_DIV2_N8: u32 = 5 << 8;
pub const SMCR_ETF_FDTS_DIV4_N6: u32 = 6 << 8;
pub const SMCR_ETF_FDTS_DIV4_N8: u32 = 7 << 8;
pub const SMCR_ETF_FDTS_DIV8_N6: u32 = 8 << 8;
pub const SMCR_ETF_FDTS_DIV8_N8: u32 = 9 << 8;
pub const SMCR_ETF_FDTS_DIV16_N5: u32 = 10 << 8;
pub const SMCR_ETF_FDTS_DIV16_N6: u32 = 11 << 8;
pub const SMCR_ETF_FDTS_DIV16_N8: u32 = 12 << 8;
pub const SMCR_ETF_FDTS_DIV32_N5: u32 = 13 << 8;
pub const SMCR_ETF_FDTS_DIV32_N6: u32 = 14 << 8;
pub const SMCR_ETF_FDTS_DIV32_N8: u32 = 15 << 8;

pub const SMCR_MSM_POS: u32 = 7;
pub const SMCR_MSM_WIDTH: u32 = 1;
pub const SMCR_MSM_MASK: u32 = 0x1 << 7;
// MSM enumerated values
pub const SMCR_MSM_NOSYNC: u32 = 0 << 7;
pub const SMCR_MSM_SYNC: u32 = 1 << 7;

pub const SMCR_TS_POS: u32 = 4;
pub const SMCR_TS_WIDTH: u32 = 3;
pub const SMCR_TS_MASK: u32 = 0x7 << 4;
// TS enumerated values
pub const SMCR_TS_ITR0: u32 = 0 << 4;
pub const SMCR_TS_ITR1: u32 = 1 << 4;
pub const SMCR_TS_ITR2: u32 = 2 << 4;
pub const SMCR_TS_TI1F_ED: u32 = 4 << 4;
pub const SMCR_TS_TI1FP1: u32 = 5 << 4;
pub const SMCR_TS_TI2FP2: u32 = 6 << 4;
pub const SMCR_TS_ETRF: u32 = 7 << 4;

pub const SMCR_SMS_POS: u32 = 0;
pub const SMCR_SMS_WIDTH: u32 = 3;
pub const SMCR_SMS_MASK: u32 = 0x7 << 0;
// SMS enumerated values
pub const SMCR_SMS_DISABLED: u32 = 0 << 0;
pub const SMCR_SMS_ENCODER_MODE_1: u32 = 1 << 0;
pub const SMCR_SMS_ENCODER_MODE_2: u32 = 2 << 0;
pub const SMCR_SMS_ENCODER_MODE_3: u32 = 3 << 0;
pub const SMCR_SMS_RESET_MODE: u32 = 4 << 0;
pub const SMCR_SMS_GATED_MODE: u32 = 5 << 0;
pub const SMCR_SMS_TRIGGER_MODE: u32 = 6 << 0;
pub const SMCR_SMS_EXT_CLOCK_MODE: u32 = 7 << 0;

// DIER register fields
pub const DIER_TDE_POS: u32 = 14;
pub const DIER_TDE_WIDTH: u32 = 1;
pub const DIER_TDE_MASK: u32 = 0x1 << 14;
// TDE enumerated values
pub const DIER_TDE_DISABLED: u32 = 0 << 14;
pub const DIER_TDE_ENABLED: u32 = 1 << 14;

pub const DIER_CC4DE_POS: u32 = 12;
pub const DIER_CC4DE_WIDTH: u32 = 1;
pub const DIER_CC4DE_MASK: u32 = 0x1 << 12;

pub const DIER_CC3DE_POS: u32 = 11;
pub const DIER_CC3DE_WIDTH: u32 = 1;
pub const DIER_CC3DE_MASK: u32 = 0x1 << 11;

pub const DIER_CC2DE_POS: u32 = 10;
pub const DIER_CC2DE_WIDTH: u32 = 1;
pub const DIER_CC2DE_MASK: u32 = 0x1 << 10;

pub const DIER_CC1DE_POS: u32 = 9;
pub const DIER_CC1DE_WIDTH: u32 = 1;
pub const DIER_CC1DE_MASK: u32 = 0x1 << 9;
// CC1DE enumerated values
pub const DIER_CC1DE_DISABLED: u32 = 0 << 9;
pub const DIER_CC1DE_ENABLED: u32 = 1 << 9;

pub const DIER_UDE_POS: u32 = 8;
pub const DIER_UDE_WIDTH: u32 = 1;
pub const DIER_UDE_MASK: u32 = 0x1 << 8;
// UDE enumerated values
pub const DIER_UDE_DISABLED: u32 = 0 << 8;
pub const DIER_UDE_ENABLED: u32 = 1 << 8;

pub const DIER_TIE_POS: u32 = 6;
pub const DIER_TIE_WIDTH: u32 = 1;
pub const DIER_TIE_MASK: u32 = 0x1 << 6;
// TIE enumerated values
pub const DIER_TIE_DISABLED: u32 = 0 << 6;
pub const DIER_TIE_ENABLED: u32 = 1 << 6;

pub const DIER_CC4IE_POS: u32 = 4;
pub const DIER_CC4IE_WIDTH: u32 = 1;
pub const DIER_CC4IE_MASK: u32 = 0x1 << 4;

pub const DIER_CC3IE_POS: u32 = 3;
pub const DIER_CC3IE_WIDTH: u32 = 1;
pub const DIER_CC3IE_MASK: u32 = 0x1 << 3;

pub const DIER_CC2IE_POS: u32 = 2;
pub const DIER_CC2IE_WIDTH: u32 = 1;
pub const DIER_CC2IE_MASK: u32 = 0x1 << 2;

pub const DIER_CC1IE_POS: u32 = 1;
pub const DIER_CC1IE_WIDTH: u32 = 1;
pub const DIER_CC1IE_MASK: u32 = 0x1 << 1;
// CC1IE enumerated values
pub const DIER_CC1IE_DISABLED: u32 = 0 << 1;
pub const DIER_CC1IE_ENABLED: u32 = 1 << 1;

pub const DIER_UIE_POS: u32 = 0;
pub const DIER_UIE_WIDTH: u32 = 1;
pub const DIER_UIE_MASK: u32 = 0x1 << 0;
// UIE enumerated values
pub const DIER_UIE_DISABLED: u32 = 0 << 0;
pub const DIER_UIE_ENABLED: u32 = 1 << 0;

// SR register fields
pub const SR_CC4OF_POS: u32 = 12;
pub const SR_CC4OF_WIDTH: u32 = 1;
pub const SR_CC4OF_MASK: u32 = 0x1 << 12;

pub const SR_CC3OF_POS: u32 = 11;
pub const SR_CC3OF_WIDTH: u32 = 1;
pub const SR_CC3OF_MASK: u32 = 0x1 << 11;

pub const SR_CC2OF_POS: u32 = 10;
pub const SR_CC2OF_WIDTH: u32 = 1;
pub const SR_CC2OF_MASK: u32 = 0x1 << 10;

pub const SR_CC1OF_POS: u32 = 9;
pub const SR_CC1OF_WIDTH: u32 = 1;
pub const SR_CC1OF_MASK: u32 = 0x1 << 9;
// CC1OF enumerated values
pub const SR_CC1OF_OVERCAPTURE: u32 = 1 << 9;

pub const SR_TIF_POS: u32 = 6;
pub const SR_TIF_WIDTH: u32 = 1;
pub const SR_TIF_MASK: u32 = 0x1 << 6;
// TIF enumerated values
pub const SR_TIF_NOTRIGGER: u32 = 0 << 6;
pub const SR_TIF_TRIGGER: u32 = 1 << 6;

pub const SR_CC4IF_POS: u32 = 4;
pub const SR_CC4IF_WIDTH: u32 = 1;
pub const SR_CC4IF_MASK: u32 = 0x1 << 4;

pub const SR_CC3IF_POS: u32 = 3;
pub const SR_CC3IF_WIDTH: u32 = 1;
pub const SR_CC3IF_MASK: u32 = 0x1 << 3;

pub const SR_CC2IF_POS: u32 = 2;
pub const SR_CC2IF_WIDTH: u32 = 1;
pub const SR_CC2IF_MASK: u32 = 0x1 << 2;

pub const SR_CC1IF_POS: u32 = 1;
pub const SR_CC1IF_WIDTH: u32 = 1;
pub const SR_CC1IF_MASK: u32 = 0x1 << 1;
// CC1IF enumerated values
pub const SR_CC1IF_MATCH: u32 = 1 << 1;

pub const SR_UIF_POS: u32 = 0;
pub const SR_UIF_WIDTH: u32 = 1;
pub const SR_UIF_MASK: u32 = 0x1 << 0;
// UIF enumerated values
pub const SR_UIF_CLEAR: u32 = 0 << 0;
pub const SR_UIF_UPDATEPENDING: u32 = 1 << 0;

// EGR register fields
pub const EGR_TG_POS: u32 = 6;
pub const EGR_TG_WIDTH: u32 = 1;
pub const EGR_TG_MASK: u32 = 0x1 << 6;
// TG enumerated values
pub const EGR_TG_TRIGGER: u32 = 1 << 6;

pub const EGR_CC4G_POS: u32 = 4;
pub const EGR_CC4G_WIDTH: u32 = 1;
pub const EGR_CC4G_MASK: u32 = 0x1 << 4;

pub const EGR_CC3G_POS: u32 = 3;
pub const EGR_CC3G_WIDTH: u32 = 1;
pub const EGR_CC3G_MASK: u32 = 0x1 << 3;

pub const EGR_CC2G_POS: u32 = 2;
pub const EGR_CC2G_WIDTH: u32 = 1;
pub const EGR_CC2G_MASK: u32 = 0x1 << 2;

pub const EGR_CC1G_POS: u32 = 1;
pub const EGR_CC1G_WIDTH: u32 = 1;
pub const EGR_CC1G_MASK: u32 = 0x1 << 1;
// CC1G enumerated values
pub const EGR_CC1G_TRIGGER: u32 = 1 << 1;

pub const EGR_UG_POS: u32 = 0;
pub const EGR_UG_WIDTH: u32 = 1;
pub const EGR_UG_MASK: u32 = 0x1 << 0;
// UG enumerated values
pub const EGR_UG_UPDATE: u32 = 1 << 0;

// CCMR1 register fields (combines output and input modes)
// Output mode fields
pub const CCMR1_OC2CE_POS: u32 = 15;
pub const CCMR1_OC2CE_WIDTH: u32 = 1;
pub const CCMR1_OC2CE_MASK: u32 = 0x1 << 15;

pub const CCMR1_OC2M_POS: u32 = 12;
pub const CCMR1_OC2M_WIDTH: u32 = 3;
pub const CCMR1_OC2M_MASK: u32 = 0x7 << 12;

pub const CCMR1_OC2PE_POS: u32 = 11;
pub const CCMR1_OC2PE_WIDTH: u32 = 1;
pub const CCMR1_OC2PE_MASK: u32 = 0x1 << 11;
// OC2PE enumerated values
pub const CCMR1_OC2PE_DISABLED: u32 = 0 << 11;
pub const CCMR1_OC2PE_ENABLED: u32 = 1 << 11;

pub const CCMR1_OC2FE_POS: u32 = 10;
pub const CCMR1_OC2FE_WIDTH: u32 = 1;
pub const CCMR1_OC2FE_MASK: u32 = 0x1 << 10;

pub const CCMR1_OC1CE_POS: u32 = 7;
pub const CCMR1_OC1CE_WIDTH: u32 = 1;
pub const CCMR1_OC1CE_MASK: u32 = 0x1 << 7;

pub const CCMR1_OC1M_POS: u32 = 4;
pub const CCMR1_OC1M_WIDTH: u32 = 3;
pub const CCMR1_OC1M_MASK: u32 = 0x7 << 4;
// OC1M enumerated values
pub const CCMR1_OC1M_FROZEN: u32 = 0 << 4;
pub const CCMR1_OC1M_ACTIVEONMATCH: u32 = 1 << 4;
pub const CCMR1_OC1M_INACTIVEONMATCH: u32 = 2 << 4;
pub const CCMR1_OC1M_TOGGLE: u32 = 3 << 4;
pub const CCMR1_OC1M_FORCEINACTIVE: u32 = 4 << 4;
pub const CCMR1_OC1M_FORCEACTIVE: u32 = 5 << 4;
pub const CCMR1_OC1M_PWMMODE1: u32 = 6 << 4;
pub const CCMR1_OC1M_PWMMODE2: u32 = 7 << 4;

pub const CCMR1_OC1PE_POS: u32 = 3;
pub const CCMR1_OC1PE_WIDTH: u32 = 1;
pub const CCMR1_OC1PE_MASK: u32 = 0x1 << 3;
// OC1PE enumerated values
pub const CCMR1_OC1PE_DISABLED: u32 = 0 << 3;
pub const CCMR1_OC1PE_ENABLED: u32 = 1 << 3;

pub const CCMR1_OC1FE_POS: u32 = 2;
pub const CCMR1_OC1FE_WIDTH: u32 = 1;
pub const CCMR1_OC1FE_MASK: u32 = 0x1 << 2;

// Input mode fields (same bit positions, different interpretation)
pub const CCMR1_IC2F_POS: u32 = 12;
pub const CCMR1_IC2F_WIDTH: u32 = 4;
pub const CCMR1_IC2F_MASK: u32 = 0xF << 12;

pub const CCMR1_IC2PSC_POS: u32 = 10;
pub const CCMR1_IC2PSC_WIDTH: u32 = 2;
pub const CCMR1_IC2PSC_MASK: u32 = 0x3 << 10;

pub const CCMR1_IC1F_POS: u32 = 4;
pub const CCMR1_IC1F_WIDTH: u32 = 4;
pub const CCMR1_IC1F_MASK: u32 = 0xF << 4;
// IC1F enumerated values
pub const CCMR1_IC1F_NOFILTER: u32 = 0 << 4;
pub const CCMR1_IC1F_FCK_INT_N2: u32 = 1 << 4;
pub const CCMR1_IC1F_FCK_INT_N4: u32 = 2 << 4;
pub const CCMR1_IC1F_FCK_INT_N8: u32 = 3 << 4;
pub const CCMR1_IC1F_FDTS_DIV2_N6: u32 = 4 << 4;
pub const CCMR1_IC1F_FDTS_DIV2_N8: u32 = 5 << 4;
pub const CCMR1_IC1F_FDTS_DIV4_N6: u32 = 6 << 4;
pub const CCMR1_IC1F_FDTS_DIV4_N8: u32 = 7 << 4;
pub const CCMR1_IC1F_FDTS_DIV8_N6: u32 = 8 << 4;
pub const CCMR1_IC1F_FDTS_DIV8_N8: u32 = 9 << 4;
pub const CCMR1_IC1F_FDTS_DIV16_N5: u32 = 10 << 4;
pub const CCMR1_IC1F_FDTS_DIV16_N6: u32 = 11 << 4;
pub const CCMR1_IC1F_FDTS_DIV16_N8: u32 = 12 << 4;
pub const CCMR1_IC1F_FDTS_DIV32_N5: u32 = 13 << 4;
pub const CCMR1_IC1F_FDTS_DIV32_N6: u32 = 14 << 4;
pub const CCMR1_IC1F_FDTS_DIV32_N8: u32 = 15 << 4;

pub const CCMR1_IC1PSC_POS: u32 = 2;
pub const CCMR1_IC1PSC_WIDTH: u32 = 2;
pub const CCMR1_IC1PSC_MASK: u32 = 0x3 << 2;

// Common CC2S and CC1S fields for both modes
pub const CCMR1_CC2S_POS: u32 = 8;
pub const CCMR1_CC2S_WIDTH: u32 = 2;
pub const CCMR1_CC2S_MASK: u32 = 0x3 << 8;
// CC2S enumerated values (output mode)
pub const CCMR1_CC2S_OUTPUT: u32 = 0 << 8;
// CC2S enumerated values (input mode)
pub const CCMR1_CC2S_TI2: u32 = 1 << 8;
pub const CCMR1_CC2S_TI1: u32 = 2 << 8;
pub const CCMR1_CC2S_TRC: u32 = 3 << 8;

pub const CCMR1_CC1S_POS: u32 = 0;
pub const CCMR1_CC1S_WIDTH: u32 = 2;
pub const CCMR1_CC1S_MASK: u32 = 0x3 << 0;
// CC1S enumerated values (output mode)
pub const CCMR1_CC1S_OUTPUT: u32 = 0 << 0;
// CC1S enumerated values (input mode)
pub const CCMR1_CC1S_TI1: u32 = 1 << 0;
pub const CCMR1_CC1S_TI2: u32 = 2 << 0;
pub const CCMR1_CC1S_TRC: u32 = 3 << 0;

// CCMR2 register fields (combines output and input modes)
// Output mode fields
pub const CCMR2_OC4CE_POS: u32 = 15;
pub const CCMR2_OC4CE_WIDTH: u32 = 1;
pub const CCMR2_OC4CE_MASK: u32 = 0x1 << 15;

pub const CCMR2_OC4M_POS: u32 = 12;
pub const CCMR2_OC4M_WIDTH: u32 = 3;
pub const CCMR2_OC4M_MASK: u32 = 0x7 << 12;

pub const CCMR2_OC4PE_POS: u32 = 11;
pub const CCMR2_OC4PE_WIDTH: u32 = 1;
pub const CCMR2_OC4PE_MASK: u32 = 0x1 << 11;
// OC4PE enumerated values
pub const CCMR2_OC4PE_DISABLED: u32 = 0 << 11;
pub const CCMR2_OC4PE_ENABLED: u32 = 1 << 11;

pub const CCMR2_OC4FE_POS: u32 = 10;
pub const CCMR2_OC4FE_WIDTH: u32 = 1;
pub const CCMR2_OC4FE_MASK: u32 = 0x1 << 10;

pub const CCMR2_OC3CE_POS: u32 = 7;
pub const CCMR2_OC3CE_WIDTH: u32 = 1;
pub const CCMR2_OC3CE_MASK: u32 = 0x1 << 7;

pub const CCMR2_OC3M_POS: u32 = 4;
pub const CCMR2_OC3M_WIDTH: u32 = 3;
pub const CCMR2_OC3M_MASK: u32 = 0x7 << 4;
// OC3M enumerated values
pub const CCMR2_OC3M_FROZEN: u32 = 0 << 4;
pub const CCMR2_OC3M_ACTIVEONMATCH: u32 = 1 << 4;
pub const CCMR2_OC3M_INACTIVEONMATCH: u32 = 2 << 4;
pub const CCMR2_OC3M_TOGGLE: u32 = 3 << 4;
pub const CCMR2_OC3M_FORCEINACTIVE: u32 = 4 << 4;
pub const CCMR2_OC3M_FORCEACTIVE: u32 = 5 << 4;
pub const CCMR2_OC3M_PWMMODE1: u32 = 6 << 4;
pub const CCMR2_OC3M_PWMMODE2: u32 = 7 << 4;

pub const CCMR2_OC3PE_POS: u32 = 3;
pub const CCMR2_OC3PE_WIDTH: u32 = 1;
pub const CCMR2_OC3PE_MASK: u32 = 0x1 << 3;
// OC3PE enumerated values
pub const CCMR2_OC3PE_DISABLED: u32 = 0 << 3;
pub const CCMR2_OC3PE_ENABLED: u32 = 1 << 3;

pub const CCMR2_OC3FE_POS: u32 = 2;
pub const CCMR2_OC3FE_WIDTH: u32 = 1;
pub const CCMR2_OC3FE_MASK: u32 = 0x1 << 2;

// Input mode fields
pub const CCMR2_IC4F_POS: u32 = 12;
pub const CCMR2_IC4F_WIDTH: u32 = 4;
pub const CCMR2_IC4F_MASK: u32 = 0xF << 12;

pub const CCMR2_IC4PSC_POS: u32 = 10;
pub const CCMR2_IC4PSC_WIDTH: u32 = 2;
pub const CCMR2_IC4PSC_MASK: u32 = 0x3 << 10;

pub const CCMR2_IC3F_POS: u32 = 4;
pub const CCMR2_IC3F_WIDTH: u32 = 4;
pub const CCMR2_IC3F_MASK: u32 = 0xF << 4;

pub const CCMR2_IC3PSC_POS: u32 = 2;
pub const CCMR2_IC3PSC_WIDTH: u32 = 2;
pub const CCMR2_IC3PSC_MASK: u32 = 0x3 << 2;

// Common CC4S and CC3S fields
pub const CCMR2_CC4S_POS: u32 = 8;
pub const CCMR2_CC4S_WIDTH: u32 = 2;
pub const CCMR2_CC4S_MASK: u32 = 0x3 << 8;
// CC4S enumerated values (output mode)
pub const CCMR2_CC4S_OUTPUT: u32 = 0 << 8;
// CC4S enumerated values (input mode)
pub const CCMR2_CC4S_TI4: u32 = 1 << 8;
pub const CCMR2_CC4S_TI3: u32 = 2 << 8;
pub const CCMR2_CC4S_TRC: u32 = 3 << 8;

pub const CCMR2_CC3S_POS: u32 = 0;
pub const CCMR2_CC3S_WIDTH: u32 = 2;
pub const CCMR2_CC3S_MASK: u32 = 0x3 << 0;
// CC3S enumerated values (output mode)
pub const CCMR2_CC3S_OUTPUT: u32 = 0 << 0;
// CC3S enumerated values (input mode)
pub const CCMR2_CC3S_TI3: u32 = 1 << 0;
pub const CCMR2_CC3S_TI4: u32 = 2 << 0;
pub const CCMR2_CC3S_TRC: u32 = 3 << 0;

// CCER register fields
pub const CCER_CC4NP_POS: u32 = 15;
pub const CCER_CC4NP_WIDTH: u32 = 1;
pub const CCER_CC4NP_MASK: u32 = 0x1 << 15;

pub const CCER_CC4P_POS: u32 = 13;
pub const CCER_CC4P_WIDTH: u32 = 1;
pub const CCER_CC4P_MASK: u32 = 0x1 << 13;

pub const CCER_CC4E_POS: u32 = 12;
pub const CCER_CC4E_WIDTH: u32 = 1;
pub const CCER_CC4E_MASK: u32 = 0x1 << 12;

pub const CCER_CC3NP_POS: u32 = 11;
pub const CCER_CC3NP_WIDTH: u32 = 1;
pub const CCER_CC3NP_MASK: u32 = 0x1 << 11;

pub const CCER_CC3P_POS: u32 = 9;
pub const CCER_CC3P_WIDTH: u32 = 1;
pub const CCER_CC3P_MASK: u32 = 0x1 << 9;

pub const CCER_CC3E_POS: u32 = 8;
pub const CCER_CC3E_WIDTH: u32 = 1;
pub const CCER_CC3E_MASK: u32 = 0x1 << 8;

pub const CCER_CC2NP_POS: u32 = 7;
pub const CCER_CC2NP_WIDTH: u32 = 1;
pub const CCER_CC2NP_MASK: u32 = 0x1 << 7;

pub const CCER_CC2P_POS: u32 = 5;
pub const CCER_CC2P_WIDTH: u32 = 1;
pub const CCER_CC2P_MASK: u32 = 0x1 << 5;

pub const CCER_CC2E_POS: u32 = 4;
pub const CCER_CC2E_WIDTH: u32 = 1;
pub const CCER_CC2E_MASK: u32 = 0x1 << 4;

pub const CCER_CC1NP_POS: u32 = 3;
pub const CCER_CC1NP_WIDTH: u32 = 1;
pub const CCER_CC1NP_MASK: u32 = 0x1 << 3;

pub const CCER_CC1P_POS: u32 = 1;
pub const CCER_CC1P_WIDTH: u32 = 1;
pub const CCER_CC1P_MASK: u32 = 0x1 << 1;

pub const CCER_CC1E_POS: u32 = 0;
pub const CCER_CC1E_WIDTH: u32 = 1;
pub const CCER_CC1E_MASK: u32 = 0x1 << 0;

// CNT register fields
pub const CNT_CNT_POS: u32 = 0;
pub const CNT_CNT_WIDTH: u32 = 32;
pub const CNT_CNT_MASK: u32 = 0xFFFFFFFF << 0;

// PSC register fields
pub const PSC_PSC_POS: u32 = 0;
pub const PSC_PSC_WIDTH: u32 = 16;
pub const PSC_PSC_MASK: u32 = 0xFFFF << 0;

// ARR register fields
pub const ARR_ARR_POS: u32 = 0;
pub const ARR_ARR_WIDTH: u32 = 32;
pub const ARR_ARR_MASK: u32 = 0xFFFFFFFF << 0;

// CCR register fields
pub const CCR1_CCR_POS: u32 = 0;
pub const CCR1_CCR_WIDTH: u32 = 32;
pub const CCR1_CCR_MASK: u32 = 0xFFFFFFFF << 0;

pub const CCR2_CCR_POS: u32 = 0;
pub const CCR2_CCR_WIDTH: u32 = 32;
pub const CCR2_CCR_MASK: u32 = 0xFFFFFFFF << 0;

pub const CCR3_CCR_POS: u32 = 0;
pub const CCR3_CCR_WIDTH: u32 = 32;
pub const CCR3_CCR_MASK: u32 = 0xFFFFFFFF << 0;

pub const CCR4_CCR_POS: u32 = 0;
pub const CCR4_CCR_WIDTH: u32 = 32;
pub const CCR4_CCR_MASK: u32 = 0xFFFFFFFF << 0;

// DCR register fields
pub const DCR_DBL_POS: u32 = 8;
pub const DCR_DBL_WIDTH: u32 = 5;
pub const DCR_DBL_MASK: u32 = 0x1F << 8;

pub const DCR_DBA_POS: u32 = 0;
pub const DCR_DBA_WIDTH: u32 = 5;
pub const DCR_DBA_MASK: u32 = 0x1F << 0;

// DMAR register fields
pub const DMAR_DMAB_POS: u32 = 0;
pub const DMAR_DMAB_WIDTH: u32 = 16;
pub const DMAR_DMAB_MASK: u32 = 0xFFFF << 0;

// OR register fields
pub const OR_ITR1_RMP_POS: u32 = 10;
pub const OR_ITR1_RMP_WIDTH: u32 = 2;
pub const OR_ITR1_RMP_MASK: u32 = 0x3 << 10;

// Timer Direction enumeration
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TimerDirection {
    Up = 0,
    Down = 1,
}

// Helper functions for Timer
impl RegisterBlock {
    /// Enable timer
    pub fn enable(&mut self) {
        self.cr1 |= CR1_CEN_MASK;
    }

    /// Disable timer
    pub fn disable(&mut self) {
        self.cr1 &= !CR1_CEN_MASK;
    }

    /// Set direction (up/down counting)
    pub fn set_direction(&mut self, direction: TimerDirection) {
        if direction == TimerDirection::Down {
            self.cr1 |= CR1_DIR_MASK;
        } else {
            self.cr1 &= !CR1_DIR_MASK;
        }
    }

    /// Set prescaler value
    pub fn set_prescaler(&mut self, prescaler: u16) {
        self.psc = prescaler as u32;
    }

    /// Set auto-reload value
    pub fn set_auto_reload(&mut self, arr: u32) {
        self.arr = arr;
    }

    /// Get current counter value
    pub fn get_counter(&self) -> u32 {
        self.cnt
    }

    /// Set counter value
    pub fn set_counter(&mut self, value: u32) {
        self.cnt = value;
    }

    /// Enable update interrupt
    pub fn enable_update_interrupt(&mut self) {
        self.dier |= DIER_UIE_MASK;
    }

    /// Disable update interrupt
    pub fn disable_update_interrupt(&mut self) {
        self.dier &= !DIER_UIE_MASK;
    }

    /// Check if update interrupt flag is set
    pub fn is_update_interrupt_pending(&self) -> bool {
        (self.sr & SR_UIF_MASK) != 0
    }

    /// Clear update interrupt flag
    pub fn clear_update_interrupt(&mut self) {
        self.sr &= !SR_UIF_MASK;
    }

    /// Generate update event
    pub fn generate_update(&mut self) {
        self.egr |= 0x1; // UG bit
    }

    /// Set capture/compare value for channel 1
    pub fn set_ccr1(&mut self, value: u32) {
        self.ccr1 = value;
    }

    /// Set capture/compare value for channel 2
    pub fn set_ccr2(&mut self, value: u32) {
        self.ccr2 = value;
    }

    /// Set capture/compare value for channel 3
    pub fn set_ccr3(&mut self, value: u32) {
        self.ccr3 = value;
    }

    /// Set capture/compare value for channel 4
    pub fn set_ccr4(&mut self, value: u32) {
        self.ccr4 = value;
    }
}
