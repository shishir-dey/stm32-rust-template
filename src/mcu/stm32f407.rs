/*
 * base addresses of Flash and SRAM memories
 */

pub const FLASH_BASEADDR: u32 = 0x08000000;
pub const SRAM1_BASEADDR: u32 = 0x20000000;
pub const SRAM2_BASEADDR: u32 = 0x2001C000;
pub const ROM_BASEADDR: u32 = 0x1FFF0000;
pub const SRAM: u32 = SRAM1_BASEADDR;

/*
 * AHBx and APBx Bus Peripheral base addresses
 */

pub const PERIPH_BASEADDR: u32 = 0x40000000;
pub const APB1PERIPH_BASEADDR: u32 = PERIPH_BASEADDR;
pub const APB2PERIPH_BASEADDR: u32 = 0x40010000;
pub const AHB1PERIPH_BASEADDR: u32 = 0x40020000;
pub const AHB2PERIPH_BASEADDR: u32 = 0x50000000;

/*
 * Base addresses of peripherals which are hanging on AHB1 bus
 */

pub const GPIOA_BASEADDR: u32 = AHB1PERIPH_BASEADDR + 0x0000;
pub const GPIOB_BASEADDR: u32 = AHB1PERIPH_BASEADDR + 0x0400;
pub const GPIOC_BASEADDR: u32 = AHB1PERIPH_BASEADDR + 0x0800;
pub const GPIOD_BASEADDR: u32 = AHB1PERIPH_BASEADDR + 0x0C00;
pub const GPIOE_BASEADDR: u32 = AHB1PERIPH_BASEADDR + 0x1000;
pub const GPIOF_BASEADDR: u32 = AHB1PERIPH_BASEADDR + 0x1400;
pub const GPIOG_BASEADDR: u32 = AHB1PERIPH_BASEADDR + 0x1800;
pub const GPIOH_BASEADDR: u32 = AHB1PERIPH_BASEADDR + 0x1C00;
pub const GPIOI_BASEADDR: u32 = AHB1PERIPH_BASEADDR + 0x2000;
pub const RCC_BASEADDR: u32 = AHB1PERIPH_BASEADDR + 0x3800;
/*
 * Base addresses of peripherals which are hanging on APB1 bus
 */
pub const I2C1_BASEADDR: u32 = APB1PERIPH_BASEADDR + 0x5400;
pub const I2C2_BASEADDR: u32 = APB1PERIPH_BASEADDR + 0x5800;
pub const I2C3_BASEADDR: u32 = APB1PERIPH_BASEADDR + 0x5C00;

pub const SPI2_BASEADDR: u32 = APB1PERIPH_BASEADDR + 0x3800;
pub const SPI3_BASEADDR: u32 = APB1PERIPH_BASEADDR + 0x3C00;

pub const USART2_BASEADDR: u32 = APB1PERIPH_BASEADDR + 0x4400;
pub const USART3_BASEADDR: u32 = APB1PERIPH_BASEADDR + 0x4800;
pub const UART4_BASEADDR: u32 = APB1PERIPH_BASEADDR + 0x4C00;
pub const UART5_BASEADDR: u32 = APB1PERIPH_BASEADDR + 0x5000;

/*
 * Base addresses of peripherals which are hanging on APB2 bus
 */
pub const EXTI_BASEADDR: u32 = APB2PERIPH_BASEADDR + 0x3C00;
pub const SPI1_BASEADDR: u32 = APB2PERIPH_BASEADDR + 0x3000;
pub const SYSCFG_BASEADDR: u32 = APB2PERIPH_BASEADDR + 0x3800;
pub const USART1_BASEADDR: u32 = APB2PERIPH_BASEADDR + 0x1000;
pub const USART6_BASEADDR: u32 = APB2PERIPH_BASEADDR + 0x1400;

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
