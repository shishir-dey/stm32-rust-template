// STM32F411 MCU-specific definitions

// Base addresses
pub const TIM2_BASEADDR: u32 = 0x40000000;
pub const TIM3_BASEADDR: u32 = 0x40000400;
pub const TIM4_BASEADDR: u32 = 0x40000800;
pub const TIM5_BASEADDR: u32 = 0x40000C00;
pub const RTC_BASEADDR: u32 = 0x40002800;
pub const WWDG_BASEADDR: u32 = 0x40002C00;
pub const IWDG_BASEADDR: u32 = 0x40003000;
pub const SPI2_BASEADDR: u32 = 0x40003800;
pub const SPI3_BASEADDR: u32 = 0x40003C00;
pub const USART2_BASEADDR: u32 = 0x40004400;
pub const I2C1_BASEADDR: u32 = 0x40005400;
pub const I2C2_BASEADDR: u32 = 0x40005800;
pub const I2C3_BASEADDR: u32 = 0x40005C00;
pub const PWR_BASEADDR: u32 = 0x40007000;
pub const TIM1_BASEADDR: u32 = 0x40010000;
pub const USART1_BASEADDR: u32 = 0x40011000;
pub const USART6_BASEADDR: u32 = 0x40011400;
pub const ADC1_BASEADDR: u32 = 0x40012000;
pub const ADC_COMMON_BASEADDR: u32 = 0x40012300;
pub const SDIO_BASEADDR: u32 = 0x40012C00;
pub const SPI1_BASEADDR: u32 = 0x40013000;
pub const SPI4_BASEADDR: u32 = 0x40013400;
pub const SYSCFG_BASEADDR: u32 = 0x40013800;
pub const EXTI_BASEADDR: u32 = 0x40013C00;
pub const TIM9_BASEADDR: u32 = 0x40014000;
pub const TIM10_BASEADDR: u32 = 0x40014400;
pub const TIM11_BASEADDR: u32 = 0x40014800;
pub const SPI5_BASEADDR: u32 = 0x40015000;
pub const GPIOA_BASEADDR: u32 = 0x40020000;
pub const GPIOB_BASEADDR: u32 = 0x40020400;
pub const GPIOC_BASEADDR: u32 = 0x40020800;
pub const GPIOD_BASEADDR: u32 = 0x40020C00;
pub const GPIOE_BASEADDR: u32 = 0x40021000;
pub const GPIOH_BASEADDR: u32 = 0x40021C00;
pub const CRC_BASEADDR: u32 = 0x40023000;
pub const RCC_BASEADDR: u32 = 0x40023800;
pub const FLASH_R_BASEADDR: u32 = 0x40023C00;
pub const DMA1_BASEADDR: u32 = 0x40026000;
pub const DMA2_BASEADDR: u32 = 0x40026400;
pub const USB_OTG_FS_BASEADDR: u32 = 0x50000000;

/*
 * IRQ(Interrupt Request) Numbers of STM32F411xE MCU
 */
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum IRQn {
    WWDG = 0,
    PVD = 1,
    TAMP_STAMP = 2,
    RTC_WKUP = 3,
    FLASH = 4,
    RCC = 5,
    EXTI0 = 6,
    EXTI1 = 7,
    EXTI2 = 8,
    EXTI3 = 9,
    EXTI4 = 10,
    DMA1_Stream0 = 11,
    DMA1_Stream1 = 12,
    DMA1_Stream2 = 13,
    DMA1_Stream3 = 14,
    DMA1_Stream4 = 15,
    DMA1_Stream5 = 16,
    DMA1_Stream6 = 17,
    ADC = 18,
    // Reserved = 19,
    // Reserved = 20,
    // Reserved = 21,
    // Reserved = 22,
    EXTI9_5 = 23,
    TIM1_BRK_TIM9 = 24,
    TIM1_UP_TIM10 = 25,
    TIM1_TRG_COM_TIM11 = 26,
    TIM1_CC = 27,
    TIM2 = 28,
    TIM3 = 29,
    TIM4 = 30,
    I2C1_EV = 31,
    I2C1_ER = 32,
    I2C2_EV = 33,
    I2C2_ER = 34,
    SPI1 = 35,
    SPI2 = 36,
    USART1 = 37,
    USART2 = 38,
    // Reserved = 39,
    EXTI15_10 = 40,
    RTC_Alarm = 41,
    OTG_FS_WKUP = 42,
    // Reserved = 43,
    // Reserved = 44,
    // Reserved = 45,
    // Reserved = 46,
    DMA1_Stream7 = 47,
    // Reserved = 48,
    SDIO = 49,
    TIM5 = 50,
    SPI3 = 51,
    // Reserved = 52,
    // Reserved = 53,
    // Reserved = 54,
    // Reserved = 55,
    DMA2_Stream0 = 56,
    DMA2_Stream1 = 57,
    DMA2_Stream2 = 58,
    DMA2_Stream3 = 59,
    DMA2_Stream4 = 60,
    // Reserved = 61,
    // Reserved = 62,
    // Reserved = 63,
    // Reserved = 64,
    // Reserved = 65,
    // Reserved = 66,
    OTG_FS = 67,
    DMA2_Stream5 = 68,
    DMA2_Stream6 = 69,
    DMA2_Stream7 = 70,
    USART6 = 71,
    I2C3_EV = 72,
    I2C3_ER = 73,
    // Reserved = 74,
    // Reserved = 75,
    // Reserved = 76,
    // Reserved = 77,
    // Reserved = 78,
    // Reserved = 79,
    // Reserved = 80,
    FPU = 81,
    // Reserved = 82,
    // Reserved = 83,
    SPI4 = 84,
    SPI5 = 85,
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
