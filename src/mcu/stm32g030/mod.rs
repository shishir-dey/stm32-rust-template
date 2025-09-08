// STM32G030 MCU-specific definitions

// Base addresses
pub const TIM2_BASEADDR: u32 = 0x40000000;
pub const TIM3_BASEADDR: u32 = 0x40000400;
pub const TIM14_BASEADDR: u32 = 0x40002000;
pub const RTC_BASEADDR: u32 = 0x40002800;
pub const WWDG_BASEADDR: u32 = 0x40002C00;
pub const IWDG_BASEADDR: u32 = 0x40003000;
pub const SPI2_BASEADDR: u32 = 0x40003800;
pub const USART2_BASEADDR: u32 = 0x40004400;
pub const I2C1_BASEADDR: u32 = 0x40005400;
pub const I2C2_BASEADDR: u32 = 0x40005800;
pub const PWR_BASEADDR: u32 = 0x40007000;
pub const SYSCFG_BASEADDR: u32 = 0x40010000;
pub const ADC_BASEADDR: u32 = 0x40012400;
pub const TIM1_BASEADDR: u32 = 0x40012C00;
pub const SPI1_BASEADDR: u32 = 0x40013000;
pub const USART1_BASEADDR: u32 = 0x40013800;
pub const TIM16_BASEADDR: u32 = 0x40014400;
pub const TIM17_BASEADDR: u32 = 0x40014800;
pub const GPIOA_BASEADDR: u32 = 0x50000000;
pub const GPIOB_BASEADDR: u32 = 0x50000400;
pub const GPIOC_BASEADDR: u32 = 0x50000800;
pub const GPIOD_BASEADDR: u32 = 0x50000C00;
pub const GPIOF_BASEADDR: u32 = 0x50001400;
pub const RCC_BASEADDR: u32 = 0x40021000;
pub const EXTI_BASEADDR: u32 = 0x40021800;
pub const FLASH_R_BASEADDR: u32 = 0x40022000;
pub const CRC_BASEADDR: u32 = 0x40023000;
pub const DMA_BASEADDR: u32 = 0x40020000;

/*
 * IRQ(Interrupt Request) Numbers of STM32G030xx MCU
 */
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum IRQn {
    WWDG = 0,
    RTC_TAMP = 2,
    FLASH = 3,
    RCC = 4,
    EXTI0_1 = 5,
    EXTI2_3 = 6,
    EXTI4_15 = 7,
    DMA1_Channel1 = 9,
    DMA1_Channel2_3 = 10,
    DMA1_Ch4_5_DMAMUX1_OVR = 11,
    ADC = 12,
    TIM1_BRK_UP_TRG_COM = 13,
    TIM1_CC = 14,
    TIM2 = 15,
    TIM3 = 16,
    TIM14 = 19,
    TIM16 = 21,
    TIM17 = 22,
    I2C1 = 23,
    I2C2 = 24,
    SPI1 = 25,
    SPI2 = 26,
    USART1 = 27,
    USART2 = 28,
    LPTIM1 = 29,
    LPTIM2 = 30,
    TIM6_DAC_LPTIM1 = 31,
    TIM7_LPTIM2 = 32,
    LPUART1 = 33,
}

pub trait PeripheralAccess {
    const BASE_ADDRESS: u32;
    type RegisterBlock;

    fn ptr() -> *const Self::RegisterBlock {
        Self::BASE_ADDRESS as *const Self::RegisterBlock
    }

    fn ptr_mut() -> *mut Self::RegisterBlock {
        Self::BASE_ADDRESS as *mut Self::RegisterBlock
    }
}

// Include peripheral modules
pub mod gpio;
pub mod rcc;
// Add other peripherals as needed
