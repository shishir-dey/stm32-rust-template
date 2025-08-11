pub const TIM2_BASEADDR: u32 = 0x40000000;
pub const TIM3_BASEADDR: u32 = 0x40000400;
pub const TIM4_BASEADDR: u32 = 0x40000800;
pub const TIM5_BASEADDR: u32 = 0x40000C00;
pub const TIM6_BASEADDR: u32 = 0x40001000;
pub const TIM7_BASEADDR: u32 = 0x40001400;
pub const TIM12_BASEADDR: u32 = 0x40001800;
pub const TIM13_BASEADDR: u32 = 0x40001C00;
pub const TIM14_BASEADDR: u32 = 0x40002000;
pub const RTC_BASEADDR: u32 = 0x40002800;
pub const WWDG_BASEADDR: u32 = 0x40002C00;
pub const IWDG_BASEADDR: u32 = 0x40003000;
pub const I2S2EXT_BASEADDR: u32 = 0x40003400;
pub const SPI2_BASEADDR: u32 = 0x40003800;
pub const SPI3_BASEADDR: u32 = 0x40003C00;
pub const I2S3EXT_BASEADDR: u32 = 0x40004000;
pub const USART2_BASEADDR: u32 = 0x40004400;
pub const USART3_BASEADDR: u32 = 0x40004800;
pub const UART4_BASEADDR: u32 = 0x40004C00;
pub const UART5_BASEADDR: u32 = 0x40005000;
pub const I2C1_BASEADDR: u32 = 0x40005400;
pub const I2C2_BASEADDR: u32 = 0x40005800;
pub const I2C3_BASEADDR: u32 = 0x40005C00;
pub const CAN1_BASEADDR: u32 = 0x40006400;
pub const CAN2_BASEADDR: u32 = 0x40006800;
pub const PWR_BASEADDR: u32 = 0x40007000;
pub const DAC_BASEADDR: u32 = 0x40007400;
pub const UART7_BASEADDR: u32 = 0x40007800;
pub const UART8_BASEADDR: u32 = 0x40007C00;
pub const TIM1_BASEADDR: u32 = 0x40010000;
pub const TIM8_BASEADDR: u32 = 0x40010400;
pub const USART1_BASEADDR: u32 = 0x40011000;
pub const USART6_BASEADDR: u32 = 0x40011400;
pub const ADC1_BASEADDR: u32 = 0x40012000;
pub const ADC2_BASEADDR: u32 = 0x40012100;
pub const ADC3_BASEADDR: u32 = 0x40012200;
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
pub const SPI6_BASEADDR: u32 = 0x40015400;
pub const SAI1_BASEADDR: u32 = 0x40015800;
pub const GPIOA_BASEADDR: u32 = 0x40020000;
pub const GPIOB_BASEADDR: u32 = 0x40020400;
pub const GPIOC_BASEADDR: u32 = 0x40020800;
pub const GPIOD_BASEADDR: u32 = 0x40020C00;
pub const GPIOE_BASEADDR: u32 = 0x40021000;
pub const GPIOF_BASEADDR: u32 = 0x40021400;
pub const GPIOG_BASEADDR: u32 = 0x40021800;
pub const GPIOH_BASEADDR: u32 = 0x40021C00;
pub const GPIOI_BASEADDR: u32 = 0x40022000;
pub const GPIOJ_BASEADDR: u32 = 0x40022400;
pub const GPIOK_BASEADDR: u32 = 0x40022800;
pub const CRC_BASEADDR: u32 = 0x40023000;
pub const RCC_BASEADDR: u32 = 0x40023800;
pub const FLASH_R_BASEADDR: u32 = 0x40023C00;
pub const DMA1_BASEADDR: u32 = 0x40026000;
pub const DMA2_BASEADDR: u32 = 0x40026400;
pub const ETH_MAC_BASEADDR: u32 = 0x40028000;
pub const ETH_MMC_BASEADDR: u32 = 0x40028100;
pub const ETH_PTP_BASEADDR: u32 = 0x40028700;
pub const ETH_DMA_BASEADDR: u32 = 0x40029000;
pub const DMA2D_BASEADDR: u32 = 0x4002B000;
pub const RNG_BASEADDR: u32 = 0x50060800;
pub const FSMC_BASEADDR: u32 = 0xA0000000;
pub const DBGMCU_BASEADDR: u32 = 0xE0042000;
pub const USB_OTG_HS_BASEADDR: u32 = 0x40040000;
pub const USB_OTG_FS_BASEADDR: u32 = 0x50000000;
pub const DCMI_BASEADDR: u32 = 0x50050000;
pub const CRYP_BASEADDR: u32 = 0x50060000;
pub const HASH_BASEADDR: u32 = 0x50060400;
pub const LTDC_BASEADDR: u32 = 0x40016800;
pub const FMC_BASEADDR: u32 = 0xA0000000;

/*
 * IRQ(Interrupt Request) Numbers of STM32F407x MCU
 */
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum IRQn {
    WWDG = 0,
    PVD = 1,
    TAMP_STAMP = 2,
    RTC_WKUP = 3,
    // Reserved = 4,
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
    CAN1_TX = 19,
    CAN1_RX0 = 20,
    CAN1_RX1 = 21,
    CAN1_SCE = 22,
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
    USART3 = 39,
    EXTI15_10 = 40,
    RTC_Alarm = 41,
    OTG_FS_WKUP = 42,
    TIM8_BRK_TIM12 = 43,
    TIM8_UP_TIM13 = 44,
    TIM8_TRG_COM_TIM14 = 45,
    TIM8_CC = 46,
    DMA1_Stream7 = 47,
    FSMC = 48,
    SDIO = 49,
    TIM5 = 50,
    SPI3 = 51,
    UART4 = 52,
    UART5 = 53,
    TIM6_DAC = 54,
    TIM7 = 55,
    DMA2_Stream0 = 56,
    DMA2_Stream1 = 57,
    DMA2_Stream2 = 58,
    DMA2_Stream3 = 59,
    DMA2_Stream4 = 60,
    ETH = 61,
    ETH_WKUP = 62,
    CAN2_TX = 63,
    CAN2_RX0 = 64,
    CAN2_RX1 = 65,
    CAN2_SCE = 66,
    OTG_FS = 67,
    DMA2_Stream5 = 68,
    DMA2_Stream6 = 69,
    DMA2_Stream7 = 70,
    USART6 = 71,
    I2C3_EV = 72,
    I2C3_ER = 73,
    OTG_HS_EP1_OUT = 74,
    OTG_HS_EP1_IN = 75,
    OTG_HS_WKUP = 76,
    OTG_HS = 77,
    DCMI = 78,
    CRYP = 79,
    HASH_RNG = 80,
    FPU = 81,
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

pub mod adc;
pub mod gpio;
pub mod i2c;
pub mod rcc;
pub mod spi;
pub mod timer;
pub mod usart;
