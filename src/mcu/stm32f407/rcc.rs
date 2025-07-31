// RCC (Reset and Clock Control) peripheral definitions
// Generated from STM32F407 SVD file

use super::{PeripheralAccess, RCC_BASEADDR};

// RCC Register Block
#[repr(C)]
pub struct RegisterBlock {
    pub cr: u32,       // RW: clock control register
    pub pllcfgr: u32,  // RW: PLL configuration register
    pub cfgr: u32,     // RW: clock configuration register
    pub cir: u32,      // RW: clock interrupt register
    pub ahb1rstr: u32, // RW: AHB1 peripheral reset register
    pub ahb2rstr: u32, // RW: AHB2 peripheral reset register
    pub ahb3rstr: u32, // RW: AHB3 peripheral reset register
    _reserved0: u32,
    pub apb1rstr: u32, // RW: APB1 peripheral reset register
    pub apb2rstr: u32, // RW: APB2 peripheral reset register
    _reserved1: [u32; 2],
    pub ahb1enr: u32, // RW: AHB1 peripheral clock register
    pub ahb2enr: u32, // RW: AHB2 peripheral clock enable register
    pub ahb3enr: u32, // RW: AHB3 peripheral clock enable register
    _reserved2: u32,
    pub apb1enr: u32, // RW: APB1 peripheral clock enable register
    pub apb2enr: u32, // RW: APB2 peripheral clock enable register
    _reserved3: [u32; 2],
    pub ahb1lpenr: u32, // RW: AHB1 peripheral clock enable in low power mode register
    pub ahb2lpenr: u32, // RW: AHB2 peripheral clock enable in low power mode register
    pub ahb3lpenr: u32, // RW: AHB3 peripheral clock enable in low power mode register
    _reserved4: u32,
    pub apb1lpenr: u32, // RW: APB1 peripheral clock enable in low power mode register
    pub apb2lpenr: u32, // RW: APB2 peripheral clock enabled in low power mode register
    _reserved5: [u32; 2],
    pub bdcr: u32, // RW: Backup domain control register
    pub csr: u32,  // RW: clock control & status register
    _reserved6: [u32; 2],
    pub sscgr: u32,      // RW: spread spectrum clock generation register
    pub plli2scfgr: u32, // RW: PLLI2S configuration register
}

impl PeripheralAccess for RegisterBlock {
    const BASE_ADDRESS: u32 = RCC_BASEADDR;
    type RegisterBlock = RegisterBlock;
}

// RCC Register Field Definitions

// CR register fields
pub const CR_PLLI2SRDY_POS: u32 = 27;
pub const CR_PLLI2SRDY_WIDTH: u32 = 1;
pub const CR_PLLI2SRDY_MASK: u32 = 0x1 << 27;

pub const CR_PLLI2SON_POS: u32 = 26;
pub const CR_PLLI2SON_WIDTH: u32 = 1;
pub const CR_PLLI2SON_MASK: u32 = 0x1 << 26;

pub const CR_PLLRDY_POS: u32 = 25;
pub const CR_PLLRDY_WIDTH: u32 = 1;
pub const CR_PLLRDY_MASK: u32 = 0x1 << 25;

pub const CR_PLLON_POS: u32 = 24;
pub const CR_PLLON_WIDTH: u32 = 1;
pub const CR_PLLON_MASK: u32 = 0x1 << 24;

pub const CR_CSSON_POS: u32 = 19;
pub const CR_CSSON_WIDTH: u32 = 1;
pub const CR_CSSON_MASK: u32 = 0x1 << 19;
// CSSON enumerated values
pub const CR_CSSON_OFF: u32 = 0 << 19;
pub const CR_CSSON_ON: u32 = 1 << 19;

pub const CR_HSEBYP_POS: u32 = 18;
pub const CR_HSEBYP_WIDTH: u32 = 1;
pub const CR_HSEBYP_MASK: u32 = 0x1 << 18;
// HSEBYP enumerated values
pub const CR_HSEBYP_NOTBYPASSED: u32 = 0 << 18;
pub const CR_HSEBYP_BYPASSED: u32 = 1 << 18;

pub const CR_HSERDY_POS: u32 = 17;
pub const CR_HSERDY_WIDTH: u32 = 1;
pub const CR_HSERDY_MASK: u32 = 0x1 << 17;

pub const CR_HSEON_POS: u32 = 16;
pub const CR_HSEON_WIDTH: u32 = 1;
pub const CR_HSEON_MASK: u32 = 0x1 << 16;

pub const CR_HSICAL_POS: u32 = 8;
pub const CR_HSICAL_WIDTH: u32 = 8;
pub const CR_HSICAL_MASK: u32 = 0xFF << 8;

pub const CR_HSITRIM_POS: u32 = 3;
pub const CR_HSITRIM_WIDTH: u32 = 5;
pub const CR_HSITRIM_MASK: u32 = 0x1F << 3;

pub const CR_HSIRDY_POS: u32 = 1;
pub const CR_HSIRDY_WIDTH: u32 = 1;
pub const CR_HSIRDY_MASK: u32 = 0x1 << 1;
// HSIRDY enumerated values
pub const CR_HSIRDY_NOTREADY: u32 = 0 << 1;
pub const CR_HSIRDY_READY: u32 = 1 << 1;

pub const CR_HSION_POS: u32 = 0;
pub const CR_HSION_WIDTH: u32 = 1;
pub const CR_HSION_MASK: u32 = 0x1 << 0;
// HSION enumerated values
pub const CR_HSION_OFF: u32 = 0 << 0;
pub const CR_HSION_ON: u32 = 1 << 0;

// PLLCFGR register fields
pub const PLLCFGR_PLLSRC_POS: u32 = 22;
pub const PLLCFGR_PLLSRC_WIDTH: u32 = 1;
pub const PLLCFGR_PLLSRC_MASK: u32 = 0x1 << 22;
// PLLSRC enumerated values
pub const PLLCFGR_PLLSRC_HSI: u32 = 0 << 22;
pub const PLLCFGR_PLLSRC_HSE: u32 = 1 << 22;

pub const PLLCFGR_PLLM_POS: u32 = 0;
pub const PLLCFGR_PLLM_WIDTH: u32 = 6;
pub const PLLCFGR_PLLM_MASK: u32 = 0x3F << 0;

pub const PLLCFGR_PLLN_POS: u32 = 6;
pub const PLLCFGR_PLLN_WIDTH: u32 = 9;
pub const PLLCFGR_PLLN_MASK: u32 = 0x1FF << 6;

pub const PLLCFGR_PLLP_POS: u32 = 16;
pub const PLLCFGR_PLLP_WIDTH: u32 = 2;
pub const PLLCFGR_PLLP_MASK: u32 = 0x3 << 16;
// PLLP enumerated values
pub const PLLCFGR_PLLP_DIV2: u32 = 0 << 16;
pub const PLLCFGR_PLLP_DIV4: u32 = 1 << 16;
pub const PLLCFGR_PLLP_DIV6: u32 = 2 << 16;
pub const PLLCFGR_PLLP_DIV8: u32 = 3 << 16;

pub const PLLCFGR_PLLQ_POS: u32 = 24;
pub const PLLCFGR_PLLQ_WIDTH: u32 = 4;
pub const PLLCFGR_PLLQ_MASK: u32 = 0xF << 24;

// CFGR register fields
pub const CFGR_MCO2_POS: u32 = 30;
pub const CFGR_MCO2_WIDTH: u32 = 2;
pub const CFGR_MCO2_MASK: u32 = 0x3 << 30;
// MCO2 enumerated values
pub const CFGR_MCO2_SYSCLK: u32 = 0 << 30;
pub const CFGR_MCO2_PLLI2S: u32 = 1 << 30;
pub const CFGR_MCO2_HSE: u32 = 2 << 30;
pub const CFGR_MCO2_PLL: u32 = 3 << 30;

pub const CFGR_MCO2PRE_POS: u32 = 27;
pub const CFGR_MCO2PRE_WIDTH: u32 = 3;
pub const CFGR_MCO2PRE_MASK: u32 = 0x7 << 27;
// MCO2PRE enumerated values
pub const CFGR_MCO2PRE_DIV1: u32 = 0 << 27;
pub const CFGR_MCO2PRE_DIV2: u32 = 4 << 27;
pub const CFGR_MCO2PRE_DIV3: u32 = 5 << 27;
pub const CFGR_MCO2PRE_DIV4: u32 = 6 << 27;
pub const CFGR_MCO2PRE_DIV5: u32 = 7 << 27;

pub const CFGR_MCO1PRE_POS: u32 = 24;
pub const CFGR_MCO1PRE_WIDTH: u32 = 3;
pub const CFGR_MCO1PRE_MASK: u32 = 0x7 << 24;
// MCO1PRE enumerated values
pub const CFGR_MCO1PRE_DIV1: u32 = 0 << 24;
pub const CFGR_MCO1PRE_DIV2: u32 = 4 << 24;
pub const CFGR_MCO1PRE_DIV3: u32 = 5 << 24;
pub const CFGR_MCO1PRE_DIV4: u32 = 6 << 24;
pub const CFGR_MCO1PRE_DIV5: u32 = 7 << 24;

pub const CFGR_MCO1_POS: u32 = 21;
pub const CFGR_MCO1_WIDTH: u32 = 2;
pub const CFGR_MCO1_MASK: u32 = 0x3 << 21;
// MCO1 enumerated values
pub const CFGR_MCO1_HSI: u32 = 0 << 21;
pub const CFGR_MCO1_LSE: u32 = 1 << 21;
pub const CFGR_MCO1_HSE: u32 = 2 << 21;
pub const CFGR_MCO1_PLL: u32 = 3 << 21;

pub const CFGR_RTCPRE_POS: u32 = 16;
pub const CFGR_RTCPRE_WIDTH: u32 = 5;
pub const CFGR_RTCPRE_MASK: u32 = 0x1F << 16;

pub const CFGR_PPRE2_POS: u32 = 13;
pub const CFGR_PPRE2_WIDTH: u32 = 3;
pub const CFGR_PPRE2_MASK: u32 = 0x7 << 13;
// PPRE2 enumerated values
pub const CFGR_PPRE2_DIV1: u32 = 0 << 13;
pub const CFGR_PPRE2_DIV2: u32 = 4 << 13;
pub const CFGR_PPRE2_DIV4: u32 = 5 << 13;
pub const CFGR_PPRE2_DIV8: u32 = 6 << 13;
pub const CFGR_PPRE2_DIV16: u32 = 7 << 13;

pub const CFGR_PPRE1_POS: u32 = 10;
pub const CFGR_PPRE1_WIDTH: u32 = 3;
pub const CFGR_PPRE1_MASK: u32 = 0x7 << 10;
// PPRE1 enumerated values
pub const CFGR_PPRE1_DIV1: u32 = 0 << 10;
pub const CFGR_PPRE1_DIV2: u32 = 4 << 10;
pub const CFGR_PPRE1_DIV4: u32 = 5 << 10;
pub const CFGR_PPRE1_DIV8: u32 = 6 << 10;
pub const CFGR_PPRE1_DIV16: u32 = 7 << 10;

pub const CFGR_HPRE_POS: u32 = 4;
pub const CFGR_HPRE_WIDTH: u32 = 4;
pub const CFGR_HPRE_MASK: u32 = 0xF << 4;
// HPRE enumerated values
pub const CFGR_HPRE_DIV1: u32 = 0 << 4;
pub const CFGR_HPRE_DIV2: u32 = 8 << 4;
pub const CFGR_HPRE_DIV4: u32 = 9 << 4;
pub const CFGR_HPRE_DIV8: u32 = 10 << 4;
pub const CFGR_HPRE_DIV16: u32 = 11 << 4;
pub const CFGR_HPRE_DIV64: u32 = 12 << 4;
pub const CFGR_HPRE_DIV128: u32 = 13 << 4;
pub const CFGR_HPRE_DIV256: u32 = 14 << 4;
pub const CFGR_HPRE_DIV512: u32 = 15 << 4;

pub const CFGR_SWS_POS: u32 = 2;
pub const CFGR_SWS_WIDTH: u32 = 2;
pub const CFGR_SWS_MASK: u32 = 0x3 << 2;
// SWS enumerated values
pub const CFGR_SWS_HSI: u32 = 0 << 2;
pub const CFGR_SWS_HSE: u32 = 1 << 2;
pub const CFGR_SWS_PLL: u32 = 2 << 2;

pub const CFGR_SW_POS: u32 = 0;
pub const CFGR_SW_WIDTH: u32 = 2;
pub const CFGR_SW_MASK: u32 = 0x3 << 0;
// SW enumerated values
pub const CFGR_SW_HSI: u32 = 0 << 0;
pub const CFGR_SW_HSE: u32 = 1 << 0;
pub const CFGR_SW_PLL: u32 = 2 << 0;

// AHB1ENR register fields
pub const AHB1ENR_DMA2EN_POS: u32 = 22;
pub const AHB1ENR_DMA2EN_WIDTH: u32 = 1;
pub const AHB1ENR_DMA2EN_MASK: u32 = 0x1 << 22;

pub const AHB1ENR_DMA1EN_POS: u32 = 21;
pub const AHB1ENR_DMA1EN_WIDTH: u32 = 1;
pub const AHB1ENR_DMA1EN_MASK: u32 = 0x1 << 21;

pub const AHB1ENR_CRCEN_POS: u32 = 12;
pub const AHB1ENR_CRCEN_WIDTH: u32 = 1;
pub const AHB1ENR_CRCEN_MASK: u32 = 0x1 << 12;

pub const AHB1ENR_GPIOIEN_POS: u32 = 8;
pub const AHB1ENR_GPIOIEN_WIDTH: u32 = 1;
pub const AHB1ENR_GPIOIEN_MASK: u32 = 0x1 << 8;

pub const AHB1ENR_GPIOHEN_POS: u32 = 7;
pub const AHB1ENR_GPIOHEN_WIDTH: u32 = 1;
pub const AHB1ENR_GPIOHEN_MASK: u32 = 0x1 << 7;

pub const AHB1ENR_GPIOGEN_POS: u32 = 6;
pub const AHB1ENR_GPIOGEN_WIDTH: u32 = 1;
pub const AHB1ENR_GPIOGEN_MASK: u32 = 0x1 << 6;

pub const AHB1ENR_GPIOFEN_POS: u32 = 5;
pub const AHB1ENR_GPIOFEN_WIDTH: u32 = 1;
pub const AHB1ENR_GPIOFEN_MASK: u32 = 0x1 << 5;

pub const AHB1ENR_GPIOEEN_POS: u32 = 4;
pub const AHB1ENR_GPIOEEN_WIDTH: u32 = 1;
pub const AHB1ENR_GPIOEEN_MASK: u32 = 0x1 << 4;

pub const AHB1ENR_GPIODEN_POS: u32 = 3;
pub const AHB1ENR_GPIODEN_WIDTH: u32 = 1;
pub const AHB1ENR_GPIODEN_MASK: u32 = 0x1 << 3;

pub const AHB1ENR_GPIOCEN_POS: u32 = 2;
pub const AHB1ENR_GPIOCEN_WIDTH: u32 = 1;
pub const AHB1ENR_GPIOCEN_MASK: u32 = 0x1 << 2;

pub const AHB1ENR_GPIOBEN_POS: u32 = 1;
pub const AHB1ENR_GPIOBEN_WIDTH: u32 = 1;
pub const AHB1ENR_GPIOBEN_MASK: u32 = 0x1 << 1;

pub const AHB1ENR_GPIOAEN_POS: u32 = 0;
pub const AHB1ENR_GPIOAEN_WIDTH: u32 = 1;
pub const AHB1ENR_GPIOAEN_MASK: u32 = 0x1 << 0;

// APB1ENR register fields
pub const APB1ENR_UART8EN_POS: u32 = 31;
pub const APB1ENR_UART8EN_WIDTH: u32 = 1;
pub const APB1ENR_UART8EN_MASK: u32 = 0x1 << 31;

pub const APB1ENR_UART7EN_POS: u32 = 30;
pub const APB1ENR_UART7EN_WIDTH: u32 = 1;
pub const APB1ENR_UART7EN_MASK: u32 = 0x1 << 30;

pub const APB1ENR_DACEN_POS: u32 = 29;
pub const APB1ENR_DACEN_WIDTH: u32 = 1;
pub const APB1ENR_DACEN_MASK: u32 = 0x1 << 29;

pub const APB1ENR_PWREN_POS: u32 = 28;
pub const APB1ENR_PWREN_WIDTH: u32 = 1;
pub const APB1ENR_PWREN_MASK: u32 = 0x1 << 28;

pub const APB1ENR_CAN2EN_POS: u32 = 26;
pub const APB1ENR_CAN2EN_WIDTH: u32 = 1;
pub const APB1ENR_CAN2EN_MASK: u32 = 0x1 << 26;

pub const APB1ENR_CAN1EN_POS: u32 = 25;
pub const APB1ENR_CAN1EN_WIDTH: u32 = 1;
pub const APB1ENR_CAN1EN_MASK: u32 = 0x1 << 25;

pub const APB1ENR_I2C3EN_POS: u32 = 23;
pub const APB1ENR_I2C3EN_WIDTH: u32 = 1;
pub const APB1ENR_I2C3EN_MASK: u32 = 0x1 << 23;

pub const APB1ENR_I2C2EN_POS: u32 = 22;
pub const APB1ENR_I2C2EN_WIDTH: u32 = 1;
pub const APB1ENR_I2C2EN_MASK: u32 = 0x1 << 22;

pub const APB1ENR_I2C1EN_POS: u32 = 21;
pub const APB1ENR_I2C1EN_WIDTH: u32 = 1;
pub const APB1ENR_I2C1EN_MASK: u32 = 0x1 << 21;

pub const APB1ENR_UART5EN_POS: u32 = 20;
pub const APB1ENR_UART5EN_WIDTH: u32 = 1;
pub const APB1ENR_UART5EN_MASK: u32 = 0x1 << 20;

pub const APB1ENR_UART4EN_POS: u32 = 19;
pub const APB1ENR_UART4EN_WIDTH: u32 = 1;
pub const APB1ENR_UART4EN_MASK: u32 = 0x1 << 19;

pub const APB1ENR_USART3EN_POS: u32 = 18;
pub const APB1ENR_USART3EN_WIDTH: u32 = 1;
pub const APB1ENR_USART3EN_MASK: u32 = 0x1 << 18;

pub const APB1ENR_USART2EN_POS: u32 = 17;
pub const APB1ENR_USART2EN_WIDTH: u32 = 1;
pub const APB1ENR_USART2EN_MASK: u32 = 0x1 << 17;

pub const APB1ENR_SPI3EN_POS: u32 = 15;
pub const APB1ENR_SPI3EN_WIDTH: u32 = 1;
pub const APB1ENR_SPI3EN_MASK: u32 = 0x1 << 15;

pub const APB1ENR_SPI2EN_POS: u32 = 14;
pub const APB1ENR_SPI2EN_WIDTH: u32 = 1;
pub const APB1ENR_SPI2EN_MASK: u32 = 0x1 << 14;

pub const APB1ENR_WWDGEN_POS: u32 = 11;
pub const APB1ENR_WWDGEN_WIDTH: u32 = 1;
pub const APB1ENR_WWDGEN_MASK: u32 = 0x1 << 11;

pub const APB1ENR_TIM14EN_POS: u32 = 8;
pub const APB1ENR_TIM14EN_WIDTH: u32 = 1;
pub const APB1ENR_TIM14EN_MASK: u32 = 0x1 << 8;

pub const APB1ENR_TIM13EN_POS: u32 = 7;
pub const APB1ENR_TIM13EN_WIDTH: u32 = 1;
pub const APB1ENR_TIM13EN_MASK: u32 = 0x1 << 7;

pub const APB1ENR_TIM12EN_POS: u32 = 6;
pub const APB1ENR_TIM12EN_WIDTH: u32 = 1;
pub const APB1ENR_TIM12EN_MASK: u32 = 0x1 << 6;

pub const APB1ENR_TIM7EN_POS: u32 = 5;
pub const APB1ENR_TIM7EN_WIDTH: u32 = 1;
pub const APB1ENR_TIM7EN_MASK: u32 = 0x1 << 5;

pub const APB1ENR_TIM6EN_POS: u32 = 4;
pub const APB1ENR_TIM6EN_WIDTH: u32 = 1;
pub const APB1ENR_TIM6EN_MASK: u32 = 0x1 << 4;

pub const APB1ENR_TIM5EN_POS: u32 = 3;
pub const APB1ENR_TIM5EN_WIDTH: u32 = 1;
pub const APB1ENR_TIM5EN_MASK: u32 = 0x1 << 3;

pub const APB1ENR_TIM4EN_POS: u32 = 2;
pub const APB1ENR_TIM4EN_WIDTH: u32 = 1;
pub const APB1ENR_TIM4EN_MASK: u32 = 0x1 << 2;

pub const APB1ENR_TIM3EN_POS: u32 = 1;
pub const APB1ENR_TIM3EN_WIDTH: u32 = 1;
pub const APB1ENR_TIM3EN_MASK: u32 = 0x1 << 1;

pub const APB1ENR_TIM2EN_POS: u32 = 0;
pub const APB1ENR_TIM2EN_WIDTH: u32 = 1;
pub const APB1ENR_TIM2EN_MASK: u32 = 0x1 << 0;

// APB2ENR register fields
pub const APB2ENR_LTDCEN_POS: u32 = 26;
pub const APB2ENR_LTDCEN_WIDTH: u32 = 1;
pub const APB2ENR_LTDCEN_MASK: u32 = 0x1 << 26;

pub const APB2ENR_SAI1EN_POS: u32 = 22;
pub const APB2ENR_SAI1EN_WIDTH: u32 = 1;
pub const APB2ENR_SAI1EN_MASK: u32 = 0x1 << 22;

pub const APB2ENR_SPI6EN_POS: u32 = 21;
pub const APB2ENR_SPI6EN_WIDTH: u32 = 1;
pub const APB2ENR_SPI6EN_MASK: u32 = 0x1 << 21;

pub const APB2ENR_SPI5EN_POS: u32 = 20;
pub const APB2ENR_SPI5EN_WIDTH: u32 = 1;
pub const APB2ENR_SPI5EN_MASK: u32 = 0x1 << 20;

pub const APB2ENR_TIM11EN_POS: u32 = 18;
pub const APB2ENR_TIM11EN_WIDTH: u32 = 1;
pub const APB2ENR_TIM11EN_MASK: u32 = 0x1 << 18;

pub const APB2ENR_TIM10EN_POS: u32 = 17;
pub const APB2ENR_TIM10EN_WIDTH: u32 = 1;
pub const APB2ENR_TIM10EN_MASK: u32 = 0x1 << 17;

pub const APB2ENR_TIM9EN_POS: u32 = 16;
pub const APB2ENR_TIM9EN_WIDTH: u32 = 1;
pub const APB2ENR_TIM9EN_MASK: u32 = 0x1 << 16;

pub const APB2ENR_EXTITEN_POS: u32 = 14;
pub const APB2ENR_EXTITEN_WIDTH: u32 = 1;
pub const APB2ENR_EXTITEN_MASK: u32 = 0x1 << 14;

pub const APB2ENR_SYSCFGEN_POS: u32 = 14;
pub const APB2ENR_SYSCFGEN_WIDTH: u32 = 1;
pub const APB2ENR_SYSCFGEN_MASK: u32 = 0x1 << 14;

pub const APB2ENR_SPI4EN_POS: u32 = 13;
pub const APB2ENR_SPI4EN_WIDTH: u32 = 1;
pub const APB2ENR_SPI4EN_MASK: u32 = 0x1 << 13;

pub const APB2ENR_SPI1EN_POS: u32 = 12;
pub const APB2ENR_SPI1EN_WIDTH: u32 = 1;
pub const APB2ENR_SPI1EN_MASK: u32 = 0x1 << 12;

pub const APB2ENR_SDIOEN_POS: u32 = 11;
pub const APB2ENR_SDIOEN_WIDTH: u32 = 1;
pub const APB2ENR_SDIOEN_MASK: u32 = 0x1 << 11;

pub const APB2ENR_ADC3EN_POS: u32 = 10;
pub const APB2ENR_ADC3EN_WIDTH: u32 = 1;
pub const APB2ENR_ADC3EN_MASK: u32 = 0x1 << 10;

pub const APB2ENR_ADC2EN_POS: u32 = 9;
pub const APB2ENR_ADC2EN_WIDTH: u32 = 1;
pub const APB2ENR_ADC2EN_MASK: u32 = 0x1 << 9;

pub const APB2ENR_ADC1EN_POS: u32 = 8;
pub const APB2ENR_ADC1EN_WIDTH: u32 = 1;
pub const APB2ENR_ADC1EN_MASK: u32 = 0x1 << 8;

pub const APB2ENR_USART6EN_POS: u32 = 5;
pub const APB2ENR_USART6EN_WIDTH: u32 = 1;
pub const APB2ENR_USART6EN_MASK: u32 = 0x1 << 5;

pub const APB2ENR_USART1EN_POS: u32 = 4;
pub const APB2ENR_USART1EN_WIDTH: u32 = 1;
pub const APB2ENR_USART1EN_MASK: u32 = 0x1 << 4;

pub const APB2ENR_TIM8EN_POS: u32 = 1;
pub const APB2ENR_TIM8EN_WIDTH: u32 = 1;
pub const APB2ENR_TIM8EN_MASK: u32 = 0x1 << 1;

pub const APB2ENR_TIM1EN_POS: u32 = 0;
pub const APB2ENR_TIM1EN_WIDTH: u32 = 1;
pub const APB2ENR_TIM1EN_MASK: u32 = 0x1 << 0;

// RCC peripheral instance
pub type RCC = RegisterBlock;
