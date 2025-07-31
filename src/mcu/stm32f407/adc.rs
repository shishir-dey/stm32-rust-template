// ADC peripheral definitions
// Generated from STM32F407 SVD file

use super::{ADC1_BASEADDR, ADC2_BASEADDR, ADC3_BASEADDR, PeripheralAccess};

// ADC Register Block
#[repr(C)]
pub struct RegisterBlock {
    pub sr: u32,    // RW: status register
    pub cr1: u32,   // RW: control register 1
    pub cr2: u32,   // RW: control register 2
    pub smpr1: u32, // RW: sample time register 1
    pub smpr2: u32, // RW: sample time register 2
    pub jofr1: u32, // RW: injected channel data offset register x
    pub jofr2: u32, // RW: injected channel data offset register x
    pub jofr3: u32, // RW: injected channel data offset register x
    pub jofr4: u32, // RW: injected channel data offset register x
    pub htr: u32,   // RW: watchdog higher threshold register
    pub ltr: u32,   // RW: watchdog lower threshold register
    pub sqr1: u32,  // RW: regular sequence register 1
    pub sqr2: u32,  // RW: regular sequence register 2
    pub sqr3: u32,  // RW: regular sequence register 3
    pub jsqr: u32,  // RW: injected sequence register
    pub jdr1: u32,  // RO: injected data register x
    pub jdr2: u32,  // RO: injected data register x
    pub jdr3: u32,  // RO: injected data register x
    pub jdr4: u32,  // RO: injected data register x
    pub dr: u32,    // RO: regular data register
}

// ADC peripheral instances
pub struct ADC1;
pub struct ADC2;
pub struct ADC3;

impl PeripheralAccess for ADC1 {
    const BASE_ADDRESS: u32 = ADC1_BASEADDR;
    type RegisterBlock = RegisterBlock;
}

impl PeripheralAccess for ADC2 {
    const BASE_ADDRESS: u32 = ADC2_BASEADDR;
    type RegisterBlock = RegisterBlock;
}

impl PeripheralAccess for ADC3 {
    const BASE_ADDRESS: u32 = ADC3_BASEADDR;
    type RegisterBlock = RegisterBlock;
}

// ADC Register Field Definitions

// SR register fields
pub const SR_OVR_POS: u32 = 5;
pub const SR_OVR_WIDTH: u32 = 1;
pub const SR_OVR_MASK: u32 = 0x1 << 5;
// OVR enumerated values
pub const SR_OVR_NOOVERRUN: u32 = 0 << 5;
pub const SR_OVR_OVERRUN: u32 = 1 << 5;

pub const SR_STRT_POS: u32 = 4;
pub const SR_STRT_WIDTH: u32 = 1;
pub const SR_STRT_MASK: u32 = 0x1 << 4;
// STRT enumerated values
pub const SR_STRT_NOTSTARTED: u32 = 0 << 4;
pub const SR_STRT_STARTED: u32 = 1 << 4;

pub const SR_JSTRT_POS: u32 = 3;
pub const SR_JSTRT_WIDTH: u32 = 1;
pub const SR_JSTRT_MASK: u32 = 0x1 << 3;
// JSTRT enumerated values
pub const SR_JSTRT_NOTSTARTED: u32 = 0 << 3;
pub const SR_JSTRT_STARTED: u32 = 1 << 3;

pub const SR_JEOC_POS: u32 = 2;
pub const SR_JEOC_WIDTH: u32 = 1;
pub const SR_JEOC_MASK: u32 = 0x1 << 2;
// JEOC enumerated values
pub const SR_JEOC_NOTCOMPLETE: u32 = 0 << 2;
pub const SR_JEOC_COMPLETE: u32 = 1 << 2;

pub const SR_EOC_POS: u32 = 1;
pub const SR_EOC_WIDTH: u32 = 1;
pub const SR_EOC_MASK: u32 = 0x1 << 1;
// EOC enumerated values
pub const SR_EOC_NOTCOMPLETE: u32 = 0 << 1;
pub const SR_EOC_COMPLETE: u32 = 1 << 1;

pub const SR_AWD_POS: u32 = 0;
pub const SR_AWD_WIDTH: u32 = 1;
pub const SR_AWD_MASK: u32 = 0x1 << 0;
// AWD enumerated values
pub const SR_AWD_NOEVENT: u32 = 0 << 0;
pub const SR_AWD_EVENT: u32 = 1 << 0;

// CR1 register fields
pub const CR1_OVRIE_POS: u32 = 26;
pub const CR1_OVRIE_WIDTH: u32 = 1;
pub const CR1_OVRIE_MASK: u32 = 0x1 << 26;
// OVRIE enumerated values
pub const CR1_OVRIE_DISABLED: u32 = 0 << 26;
pub const CR1_OVRIE_ENABLED: u32 = 1 << 26;

pub const CR1_RES_POS: u32 = 24;
pub const CR1_RES_WIDTH: u32 = 2;
pub const CR1_RES_MASK: u32 = 0x3 << 24;
// RES enumerated values
pub const CR1_RES_TWELVEBIT: u32 = 0 << 24;
pub const CR1_RES_TENBIT: u32 = 1 << 24;
pub const CR1_RES_EIGHTBIT: u32 = 2 << 24;
pub const CR1_RES_SIXBIT: u32 = 3 << 24;

pub const CR1_AWDEN_POS: u32 = 23;
pub const CR1_AWDEN_WIDTH: u32 = 1;
pub const CR1_AWDEN_MASK: u32 = 0x1 << 23;
// AWDEN enumerated values
pub const CR1_AWDEN_DISABLED: u32 = 0 << 23;
pub const CR1_AWDEN_ENABLED: u32 = 1 << 23;

pub const CR1_JAWDEN_POS: u32 = 22;
pub const CR1_JAWDEN_WIDTH: u32 = 1;
pub const CR1_JAWDEN_MASK: u32 = 0x1 << 22;
// JAWDEN enumerated values
pub const CR1_JAWDEN_DISABLED: u32 = 0 << 22;
pub const CR1_JAWDEN_ENABLED: u32 = 1 << 22;

pub const CR1_DISCNUM_POS: u32 = 13;
pub const CR1_DISCNUM_WIDTH: u32 = 3;
pub const CR1_DISCNUM_MASK: u32 = 0x7 << 13;

pub const CR1_JDISCEN_POS: u32 = 12;
pub const CR1_JDISCEN_WIDTH: u32 = 1;
pub const CR1_JDISCEN_MASK: u32 = 0x1 << 12;
// JDISCEN enumerated values
pub const CR1_JDISCEN_DISABLED: u32 = 0 << 12;
pub const CR1_JDISCEN_ENABLED: u32 = 1 << 12;

pub const CR1_DISCEN_POS: u32 = 11;
pub const CR1_DISCEN_WIDTH: u32 = 1;
pub const CR1_DISCEN_MASK: u32 = 0x1 << 11;
// DISCEN enumerated values
pub const CR1_DISCEN_DISABLED: u32 = 0 << 11;
pub const CR1_DISCEN_ENABLED: u32 = 1 << 11;

pub const CR1_JAUTO_POS: u32 = 10;
pub const CR1_JAUTO_WIDTH: u32 = 1;
pub const CR1_JAUTO_MASK: u32 = 0x1 << 10;
// JAUTO enumerated values
pub const CR1_JAUTO_DISABLED: u32 = 0 << 10;
pub const CR1_JAUTO_ENABLED: u32 = 1 << 10;

pub const CR1_AWDSGL_POS: u32 = 9;
pub const CR1_AWDSGL_WIDTH: u32 = 1;
pub const CR1_AWDSGL_MASK: u32 = 0x1 << 9;
// AWDSGL enumerated values
pub const CR1_AWDSGL_ALLCHANNELS: u32 = 0 << 9;
pub const CR1_AWDSGL_SINGLECHANNEL: u32 = 1 << 9;

pub const CR1_SCAN_POS: u32 = 8;
pub const CR1_SCAN_WIDTH: u32 = 1;
pub const CR1_SCAN_MASK: u32 = 0x1 << 8;
// SCAN enumerated values
pub const CR1_SCAN_DISABLED: u32 = 0 << 8;
pub const CR1_SCAN_ENABLED: u32 = 1 << 8;

pub const CR1_JEOCIE_POS: u32 = 7;
pub const CR1_JEOCIE_WIDTH: u32 = 1;
pub const CR1_JEOCIE_MASK: u32 = 0x1 << 7;
// JEOCIE enumerated values
pub const CR1_JEOCIE_DISABLED: u32 = 0 << 7;
pub const CR1_JEOCIE_ENABLED: u32 = 1 << 7;

pub const CR1_AWDIE_POS: u32 = 6;
pub const CR1_AWDIE_WIDTH: u32 = 1;
pub const CR1_AWDIE_MASK: u32 = 0x1 << 6;
// AWDIE enumerated values
pub const CR1_AWDIE_DISABLED: u32 = 0 << 6;
pub const CR1_AWDIE_ENABLED: u32 = 1 << 6;

pub const CR1_EOCIE_POS: u32 = 5;
pub const CR1_EOCIE_WIDTH: u32 = 1;
pub const CR1_EOCIE_MASK: u32 = 0x1 << 5;
// EOCIE enumerated values
pub const CR1_EOCIE_DISABLED: u32 = 0 << 5;
pub const CR1_EOCIE_ENABLED: u32 = 1 << 5;

pub const CR1_AWDCH_POS: u32 = 0;
pub const CR1_AWDCH_WIDTH: u32 = 5;
pub const CR1_AWDCH_MASK: u32 = 0x1F << 0;

// CR2 register fields
pub const CR2_SWSTART_POS: u32 = 30;
pub const CR2_SWSTART_WIDTH: u32 = 1;
pub const CR2_SWSTART_MASK: u32 = 0x1 << 30;
// SWSTART enumerated values
pub const CR2_SWSTART_START: u32 = 1 << 30;

pub const CR2_EXTEN_POS: u32 = 28;
pub const CR2_EXTEN_WIDTH: u32 = 2;
pub const CR2_EXTEN_MASK: u32 = 0x3 << 28;
// EXTEN enumerated values
pub const CR2_EXTEN_DISABLED: u32 = 0 << 28;
pub const CR2_EXTEN_RISINGEDGE: u32 = 1 << 28;
pub const CR2_EXTEN_FALLINGEDGE: u32 = 2 << 28;
pub const CR2_EXTEN_BOTHEDGES: u32 = 3 << 28;

pub const CR2_EXTSEL_POS: u32 = 24;
pub const CR2_EXTSEL_WIDTH: u32 = 4;
pub const CR2_EXTSEL_MASK: u32 = 0xF << 24;
// EXTSEL enumerated values
pub const CR2_EXTSEL_TIM1CC1: u32 = 0 << 24;
pub const CR2_EXTSEL_TIM1CC2: u32 = 1 << 24;
pub const CR2_EXTSEL_TIM1CC3: u32 = 2 << 24;
pub const CR2_EXTSEL_TIM2CC2: u32 = 3 << 24;
pub const CR2_EXTSEL_TIM2CC3: u32 = 4 << 24;
pub const CR2_EXTSEL_TIM2CC4: u32 = 5 << 24;
pub const CR2_EXTSEL_TIM2TRGO: u32 = 6 << 24;

pub const CR2_JSWSTART_POS: u32 = 22;
pub const CR2_JSWSTART_WIDTH: u32 = 1;
pub const CR2_JSWSTART_MASK: u32 = 0x1 << 22;
// JSWSTART enumerated values
pub const CR2_JSWSTART_START: u32 = 1 << 22;

pub const CR2_JEXTEN_POS: u32 = 20;
pub const CR2_JEXTEN_WIDTH: u32 = 2;
pub const CR2_JEXTEN_MASK: u32 = 0x3 << 20;
// JEXTEN enumerated values
pub const CR2_JEXTEN_DISABLED: u32 = 0 << 20;
pub const CR2_JEXTEN_RISINGEDGE: u32 = 1 << 20;
pub const CR2_JEXTEN_FALLINGEDGE: u32 = 2 << 20;
pub const CR2_JEXTEN_BOTHEDGES: u32 = 3 << 20;

pub const CR2_JEXTSEL_POS: u32 = 16;
pub const CR2_JEXTSEL_WIDTH: u32 = 4;
pub const CR2_JEXTSEL_MASK: u32 = 0xF << 16;
// JEXTSEL enumerated values
pub const CR2_JEXTSEL_TIM1TRGO: u32 = 0 << 16;
pub const CR2_JEXTSEL_TIM1CC4: u32 = 1 << 16;
pub const CR2_JEXTSEL_TIM2TRGO: u32 = 2 << 16;
pub const CR2_JEXTSEL_TIM2CC1: u32 = 3 << 16;
pub const CR2_JEXTSEL_TIM3CC4: u32 = 4 << 16;
pub const CR2_JEXTSEL_TIM4TRGO: u32 = 5 << 16;
pub const CR2_JEXTSEL_TIM8CC4: u32 = 7 << 16;
pub const CR2_JEXTSEL_TIM1TRGO2: u32 = 8 << 16;
pub const CR2_JEXTSEL_TIM8TRGO: u32 = 9 << 16;
pub const CR2_JEXTSEL_TIM8TRGO2: u32 = 10 << 16;
pub const CR2_JEXTSEL_TIM3CC3: u32 = 11 << 16;
pub const CR2_JEXTSEL_TIM5TRGO: u32 = 12 << 16;
pub const CR2_JEXTSEL_TIM3CC1: u32 = 13 << 16;
pub const CR2_JEXTSEL_TIM6TRGO: u32 = 14 << 16;

pub const CR2_ALIGN_POS: u32 = 11;
pub const CR2_ALIGN_WIDTH: u32 = 1;
pub const CR2_ALIGN_MASK: u32 = 0x1 << 11;
// ALIGN enumerated values
pub const CR2_ALIGN_RIGHT: u32 = 0 << 11;
pub const CR2_ALIGN_LEFT: u32 = 1 << 11;

pub const CR2_EOCS_POS: u32 = 10;
pub const CR2_EOCS_WIDTH: u32 = 1;
pub const CR2_EOCS_MASK: u32 = 0x1 << 10;
// EOCS enumerated values
pub const CR2_EOCS_EACHSEQUENCE: u32 = 0 << 10;
pub const CR2_EOCS_EACHCONVERSION: u32 = 1 << 10;

pub const CR2_DDS_POS: u32 = 9;
pub const CR2_DDS_WIDTH: u32 = 1;
pub const CR2_DDS_MASK: u32 = 0x1 << 9;
// DDS enumerated values
pub const CR2_DDS_SINGLE: u32 = 0 << 9;
pub const CR2_DDS_CONTINUOUS: u32 = 1 << 9;

pub const CR2_DMA_POS: u32 = 8;
pub const CR2_DMA_WIDTH: u32 = 1;
pub const CR2_DMA_MASK: u32 = 0x1 << 8;
// DMA enumerated values
pub const CR2_DMA_DISABLED: u32 = 0 << 8;
pub const CR2_DMA_ENABLED: u32 = 1 << 8;

pub const CR2_CONT_POS: u32 = 1;
pub const CR2_CONT_WIDTH: u32 = 1;
pub const CR2_CONT_MASK: u32 = 0x1 << 1;
// CONT enumerated values
pub const CR2_CONT_SINGLE: u32 = 0 << 1;
pub const CR2_CONT_CONTINUOUS: u32 = 1 << 1;

pub const CR2_ADON_POS: u32 = 0;
pub const CR2_ADON_WIDTH: u32 = 1;
pub const CR2_ADON_MASK: u32 = 0x1 << 0;
// ADON enumerated values
pub const CR2_ADON_DISABLED: u32 = 0 << 0;
pub const CR2_ADON_ENABLED: u32 = 1 << 0;

// SMPR1 register fields
pub const SMPR1_SMP18_POS: u32 = 24;
pub const SMPR1_SMP18_WIDTH: u32 = 3;
pub const SMPR1_SMP18_MASK: u32 = 0x7 << 24;

pub const SMPR1_SMP17_POS: u32 = 21;
pub const SMPR1_SMP17_WIDTH: u32 = 3;
pub const SMPR1_SMP17_MASK: u32 = 0x7 << 21;

pub const SMPR1_SMP16_POS: u32 = 18;
pub const SMPR1_SMP16_WIDTH: u32 = 3;
pub const SMPR1_SMP16_MASK: u32 = 0x7 << 18;

pub const SMPR1_SMP15_POS: u32 = 15;
pub const SMPR1_SMP15_WIDTH: u32 = 3;
pub const SMPR1_SMP15_MASK: u32 = 0x7 << 15;

pub const SMPR1_SMP14_POS: u32 = 12;
pub const SMPR1_SMP14_WIDTH: u32 = 3;
pub const SMPR1_SMP14_MASK: u32 = 0x7 << 12;

pub const SMPR1_SMP13_POS: u32 = 9;
pub const SMPR1_SMP13_WIDTH: u32 = 3;
pub const SMPR1_SMP13_MASK: u32 = 0x7 << 9;

pub const SMPR1_SMP12_POS: u32 = 6;
pub const SMPR1_SMP12_WIDTH: u32 = 3;
pub const SMPR1_SMP12_MASK: u32 = 0x7 << 6;

pub const SMPR1_SMP11_POS: u32 = 3;
pub const SMPR1_SMP11_WIDTH: u32 = 3;
pub const SMPR1_SMP11_MASK: u32 = 0x7 << 3;

pub const SMPR1_SMP10_POS: u32 = 0;
pub const SMPR1_SMP10_WIDTH: u32 = 3;
pub const SMPR1_SMP10_MASK: u32 = 0x7 << 0;
// SMP10 enumerated values
pub const SMPR1_SMP10_CYCLES3: u32 = 0 << 0;
pub const SMPR1_SMP10_CYCLES15: u32 = 1 << 0;
pub const SMPR1_SMP10_CYCLES28: u32 = 2 << 0;
pub const SMPR1_SMP10_CYCLES56: u32 = 3 << 0;
pub const SMPR1_SMP10_CYCLES84: u32 = 4 << 0;
pub const SMPR1_SMP10_CYCLES112: u32 = 5 << 0;
pub const SMPR1_SMP10_CYCLES144: u32 = 6 << 0;
pub const SMPR1_SMP10_CYCLES480: u32 = 7 << 0;

// SMPR2 register fields
pub const SMPR2_SMP9_POS: u32 = 27;
pub const SMPR2_SMP9_WIDTH: u32 = 3;
pub const SMPR2_SMP9_MASK: u32 = 0x7 << 27;

pub const SMPR2_SMP8_POS: u32 = 24;
pub const SMPR2_SMP8_WIDTH: u32 = 3;
pub const SMPR2_SMP8_MASK: u32 = 0x7 << 24;

pub const SMPR2_SMP7_POS: u32 = 21;
pub const SMPR2_SMP7_WIDTH: u32 = 3;
pub const SMPR2_SMP7_MASK: u32 = 0x7 << 21;

pub const SMPR2_SMP6_POS: u32 = 18;
pub const SMPR2_SMP6_WIDTH: u32 = 3;
pub const SMPR2_SMP6_MASK: u32 = 0x7 << 18;

pub const SMPR2_SMP5_POS: u32 = 15;
pub const SMPR2_SMP5_WIDTH: u32 = 3;
pub const SMPR2_SMP5_MASK: u32 = 0x7 << 15;

pub const SMPR2_SMP4_POS: u32 = 12;
pub const SMPR2_SMP4_WIDTH: u32 = 3;
pub const SMPR2_SMP4_MASK: u32 = 0x7 << 12;

pub const SMPR2_SMP3_POS: u32 = 9;
pub const SMPR2_SMP3_WIDTH: u32 = 3;
pub const SMPR2_SMP3_MASK: u32 = 0x7 << 9;

pub const SMPR2_SMP2_POS: u32 = 6;
pub const SMPR2_SMP2_WIDTH: u32 = 3;
pub const SMPR2_SMP2_MASK: u32 = 0x7 << 6;

pub const SMPR2_SMP1_POS: u32 = 3;
pub const SMPR2_SMP1_WIDTH: u32 = 3;
pub const SMPR2_SMP1_MASK: u32 = 0x7 << 3;

pub const SMPR2_SMP0_POS: u32 = 0;
pub const SMPR2_SMP0_WIDTH: u32 = 3;
pub const SMPR2_SMP0_MASK: u32 = 0x7 << 0;
// SMP0 enumerated values
pub const SMPR2_SMP0_CYCLES3: u32 = 0 << 0;
pub const SMPR2_SMP0_CYCLES15: u32 = 1 << 0;
pub const SMPR2_SMP0_CYCLES28: u32 = 2 << 0;
pub const SMPR2_SMP0_CYCLES56: u32 = 3 << 0;
pub const SMPR2_SMP0_CYCLES84: u32 = 4 << 0;
pub const SMPR2_SMP0_CYCLES112: u32 = 5 << 0;
pub const SMPR2_SMP0_CYCLES144: u32 = 6 << 0;
pub const SMPR2_SMP0_CYCLES480: u32 = 7 << 0;

// JOFR register fields (injected channel data offset registers)
pub const JOFR1_JOFFSET_POS: u32 = 0;
pub const JOFR1_JOFFSET_WIDTH: u32 = 12;
pub const JOFR1_JOFFSET_MASK: u32 = 0xFFF << 0;

pub const JOFR2_JOFFSET_POS: u32 = 0;
pub const JOFR2_JOFFSET_WIDTH: u32 = 12;
pub const JOFR2_JOFFSET_MASK: u32 = 0xFFF << 0;

pub const JOFR3_JOFFSET_POS: u32 = 0;
pub const JOFR3_JOFFSET_WIDTH: u32 = 12;
pub const JOFR3_JOFFSET_MASK: u32 = 0xFFF << 0;

pub const JOFR4_JOFFSET_POS: u32 = 0;
pub const JOFR4_JOFFSET_WIDTH: u32 = 12;
pub const JOFR4_JOFFSET_MASK: u32 = 0xFFF << 0;

// HTR register fields
pub const HTR_HT_POS: u32 = 0;
pub const HTR_HT_WIDTH: u32 = 12;
pub const HTR_HT_MASK: u32 = 0xFFF << 0;

// LTR register fields
pub const LTR_LT_POS: u32 = 0;
pub const LTR_LT_WIDTH: u32 = 12;
pub const LTR_LT_MASK: u32 = 0xFFF << 0;

// SQR1 register fields
pub const SQR1_L_POS: u32 = 20;
pub const SQR1_L_WIDTH: u32 = 4;
pub const SQR1_L_MASK: u32 = 0xF << 20;

pub const SQR1_SQ16_POS: u32 = 15;
pub const SQR1_SQ16_WIDTH: u32 = 5;
pub const SQR1_SQ16_MASK: u32 = 0x1F << 15;

pub const SQR1_SQ15_POS: u32 = 10;
pub const SQR1_SQ15_WIDTH: u32 = 5;
pub const SQR1_SQ15_MASK: u32 = 0x1F << 10;

pub const SQR1_SQ14_POS: u32 = 5;
pub const SQR1_SQ14_WIDTH: u32 = 5;
pub const SQR1_SQ14_MASK: u32 = 0x1F << 5;

pub const SQR1_SQ13_POS: u32 = 0;
pub const SQR1_SQ13_WIDTH: u32 = 5;
pub const SQR1_SQ13_MASK: u32 = 0x1F << 0;

// SQR2 register fields
pub const SQR2_SQ12_POS: u32 = 25;
pub const SQR2_SQ12_WIDTH: u32 = 5;
pub const SQR2_SQ12_MASK: u32 = 0x1F << 25;

pub const SQR2_SQ11_POS: u32 = 20;
pub const SQR2_SQ11_WIDTH: u32 = 5;
pub const SQR2_SQ11_MASK: u32 = 0x1F << 20;

pub const SQR2_SQ10_POS: u32 = 15;
pub const SQR2_SQ10_WIDTH: u32 = 5;
pub const SQR2_SQ10_MASK: u32 = 0x1F << 15;

pub const SQR2_SQ9_POS: u32 = 10;
pub const SQR2_SQ9_WIDTH: u32 = 5;
pub const SQR2_SQ9_MASK: u32 = 0x1F << 10;

pub const SQR2_SQ8_POS: u32 = 5;
pub const SQR2_SQ8_WIDTH: u32 = 5;
pub const SQR2_SQ8_MASK: u32 = 0x1F << 5;

pub const SQR2_SQ7_POS: u32 = 0;
pub const SQR2_SQ7_WIDTH: u32 = 5;
pub const SQR2_SQ7_MASK: u32 = 0x1F << 0;

// SQR3 register fields
pub const SQR3_SQ6_POS: u32 = 25;
pub const SQR3_SQ6_WIDTH: u32 = 5;
pub const SQR3_SQ6_MASK: u32 = 0x1F << 25;

pub const SQR3_SQ5_POS: u32 = 20;
pub const SQR3_SQ5_WIDTH: u32 = 5;
pub const SQR3_SQ5_MASK: u32 = 0x1F << 20;

pub const SQR3_SQ4_POS: u32 = 15;
pub const SQR3_SQ4_WIDTH: u32 = 5;
pub const SQR3_SQ4_MASK: u32 = 0x1F << 15;

pub const SQR3_SQ3_POS: u32 = 10;
pub const SQR3_SQ3_WIDTH: u32 = 5;
pub const SQR3_SQ3_MASK: u32 = 0x1F << 10;

pub const SQR3_SQ2_POS: u32 = 5;
pub const SQR3_SQ2_WIDTH: u32 = 5;
pub const SQR3_SQ2_MASK: u32 = 0x1F << 5;

pub const SQR3_SQ1_POS: u32 = 0;
pub const SQR3_SQ1_WIDTH: u32 = 5;
pub const SQR3_SQ1_MASK: u32 = 0x1F << 0;

// JSQR register fields
pub const JSQR_JL_POS: u32 = 20;
pub const JSQR_JL_WIDTH: u32 = 2;
pub const JSQR_JL_MASK: u32 = 0x3 << 20;

pub const JSQR_JSQ4_POS: u32 = 15;
pub const JSQR_JSQ4_WIDTH: u32 = 5;
pub const JSQR_JSQ4_MASK: u32 = 0x1F << 15;

pub const JSQR_JSQ3_POS: u32 = 10;
pub const JSQR_JSQ3_WIDTH: u32 = 5;
pub const JSQR_JSQ3_MASK: u32 = 0x1F << 10;

pub const JSQR_JSQ2_POS: u32 = 5;
pub const JSQR_JSQ2_WIDTH: u32 = 5;
pub const JSQR_JSQ2_MASK: u32 = 0x1F << 5;

pub const JSQR_JSQ1_POS: u32 = 0;
pub const JSQR_JSQ1_WIDTH: u32 = 5;
pub const JSQR_JSQ1_MASK: u32 = 0x1F << 0;

// JDR register fields (injected data registers)
pub const JDR1_JDATA_POS: u32 = 0;
pub const JDR1_JDATA_WIDTH: u32 = 16;
pub const JDR1_JDATA_MASK: u32 = 0xFFFF << 0;

pub const JDR2_JDATA_POS: u32 = 0;
pub const JDR2_JDATA_WIDTH: u32 = 16;
pub const JDR2_JDATA_MASK: u32 = 0xFFFF << 0;

pub const JDR3_JDATA_POS: u32 = 0;
pub const JDR3_JDATA_WIDTH: u32 = 16;
pub const JDR3_JDATA_MASK: u32 = 0xFFFF << 0;

pub const JDR4_JDATA_POS: u32 = 0;
pub const JDR4_JDATA_WIDTH: u32 = 16;
pub const JDR4_JDATA_MASK: u32 = 0xFFFF << 0;

// DR register fields
pub const DR_DATA_POS: u32 = 0;
pub const DR_DATA_WIDTH: u32 = 16;
pub const DR_DATA_MASK: u32 = 0xFFFF << 0;

// ADC Resolution enumeration
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AdcResolution {
    Bits12 = 0,
    Bits10 = 1,
    Bits8 = 2,
    Bits6 = 3,
}

// ADC Sample Time enumeration
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AdcSampleTime {
    Cycles3 = 0,
    Cycles15 = 1,
    Cycles28 = 2,
    Cycles56 = 3,
    Cycles84 = 4,
    Cycles112 = 5,
    Cycles144 = 6,
    Cycles480 = 7,
}

// Helper functions for ADC
impl RegisterBlock {
    /// Enable ADC
    pub fn enable(&mut self) {
        self.cr2 |= CR2_ADON_MASK;
    }

    /// Disable ADC
    pub fn disable(&mut self) {
        self.cr2 &= !CR2_ADON_MASK;
    }

    /// Set ADC resolution
    pub fn set_resolution(&mut self, resolution: AdcResolution) {
        self.cr1 = (self.cr1 & !CR1_RES_MASK) | ((resolution as u32) << CR1_RES_POS);
    }

    /// Enable continuous conversion mode
    pub fn enable_continuous(&mut self) {
        self.cr2 |= CR2_CONT_MASK;
    }

    /// Disable continuous conversion mode
    pub fn disable_continuous(&mut self) {
        self.cr2 &= !CR2_CONT_MASK;
    }

    /// Start software conversion
    pub fn start_conversion(&mut self) {
        self.cr2 |= CR2_SWSTART_MASK;
    }

    /// Check if conversion is complete
    pub fn is_conversion_complete(&self) -> bool {
        (self.sr & SR_EOC_MASK) != 0
    }

    /// Clear end of conversion flag
    pub fn clear_eoc_flag(&mut self) {
        self.sr &= !SR_EOC_MASK;
    }

    /// Read conversion result
    pub fn read_data(&self) -> u16 {
        self.dr as u16
    }

    /// Set sample time for channel (0-9)
    pub fn set_sample_time_low(&mut self, channel: u8, sample_time: AdcSampleTime) {
        if channel < 10 {
            let shift = channel * 3;
            let mask = 0x7 << shift;
            self.smpr2 = (self.smpr2 & !mask) | ((sample_time as u32) << shift);
        }
    }

    /// Set sample time for channel (10-18)
    pub fn set_sample_time_high(&mut self, channel: u8, sample_time: AdcSampleTime) {
        if channel >= 10 && channel < 19 {
            let shift = (channel - 10) * 3;
            let mask = 0x7 << shift;
            self.smpr1 = (self.smpr1 & !mask) | ((sample_time as u32) << shift);
        }
    }

    /// Set first regular sequence channel
    pub fn set_regular_sequence_1(&mut self, channel: u8) {
        self.sqr3 = (self.sqr3 & !0x1F) | (channel as u32 & 0x1F);
    }

    /// Set regular sequence length
    pub fn set_regular_sequence_length(&mut self, length: u8) {
        if length > 0 && length <= 16 {
            let len = (length - 1) as u32;
            self.sqr1 = (self.sqr1 & !0xF00000) | ((len & 0xF) << 20);
        }
    }

    /// Perform single conversion on a channel
    pub fn single_conversion(&mut self, channel: u8) -> u16 {
        // Set channel as first in sequence
        self.set_regular_sequence_1(channel);
        self.set_regular_sequence_length(1);

        // Start conversion
        self.start_conversion();

        // Wait for completion
        while !self.is_conversion_complete() {}

        // Read result
        self.read_data()
    }
}
