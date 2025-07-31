// GPIO (General Purpose Input/Output) peripheral definitions
// Generated from STM32F407 SVD file

use super::{
    GPIOA_BASEADDR, GPIOB_BASEADDR, GPIOC_BASEADDR, GPIOD_BASEADDR, GPIOE_BASEADDR,
    PeripheralAccess,
};

// GPIO Register Block
#[repr(C)]
pub struct RegisterBlock {
    pub moder: u32,   // RW: GPIO port mode register
    pub otyper: u32,  // RW: GPIO port output type register
    pub ospeedr: u32, // RW: GPIO port output speed register
    pub pupdr: u32,   // RW: GPIO port pull-up/pull-down register
    pub idr: u32,     // RO: GPIO port input data register
    pub odr: u32,     // RW: GPIO port output data register
    pub bsrr: u32,    // WO: GPIO port bit set/reset register
    pub lckr: u32,    // RW: GPIO port configuration lock register
    pub afrl: u32,    // RW: GPIO alternate function low register
    pub afrh: u32,    // RW: GPIO alternate function high register
}

// GPIO peripheral instances
pub struct GPIOA;
pub struct GPIOB;
pub struct GPIOC;
pub struct GPIOD;
pub struct GPIOE;

impl PeripheralAccess for GPIOA {
    const BASE_ADDRESS: u32 = GPIOA_BASEADDR;
    type RegisterBlock = RegisterBlock;
}

impl PeripheralAccess for GPIOB {
    const BASE_ADDRESS: u32 = GPIOB_BASEADDR;
    type RegisterBlock = RegisterBlock;
}

impl PeripheralAccess for GPIOC {
    const BASE_ADDRESS: u32 = GPIOC_BASEADDR;
    type RegisterBlock = RegisterBlock;
}

impl PeripheralAccess for GPIOD {
    const BASE_ADDRESS: u32 = GPIOD_BASEADDR;
    type RegisterBlock = RegisterBlock;
}

impl PeripheralAccess for GPIOE {
    const BASE_ADDRESS: u32 = GPIOE_BASEADDR;
    type RegisterBlock = RegisterBlock;
}

// GPIO Register Field Definitions

// MODER register fields
pub const MODER_MODER15_POS: u32 = 30;
pub const MODER_MODER15_WIDTH: u32 = 2;
pub const MODER_MODER15_MASK: u32 = 0x3 << 30;

pub const MODER_MODER14_POS: u32 = 28;
pub const MODER_MODER14_WIDTH: u32 = 2;
pub const MODER_MODER14_MASK: u32 = 0x3 << 28;

pub const MODER_MODER13_POS: u32 = 26;
pub const MODER_MODER13_WIDTH: u32 = 2;
pub const MODER_MODER13_MASK: u32 = 0x3 << 26;

pub const MODER_MODER12_POS: u32 = 24;
pub const MODER_MODER12_WIDTH: u32 = 2;
pub const MODER_MODER12_MASK: u32 = 0x3 << 24;

pub const MODER_MODER11_POS: u32 = 22;
pub const MODER_MODER11_WIDTH: u32 = 2;
pub const MODER_MODER11_MASK: u32 = 0x3 << 22;

pub const MODER_MODER10_POS: u32 = 20;
pub const MODER_MODER10_WIDTH: u32 = 2;
pub const MODER_MODER10_MASK: u32 = 0x3 << 20;

pub const MODER_MODER9_POS: u32 = 18;
pub const MODER_MODER9_WIDTH: u32 = 2;
pub const MODER_MODER9_MASK: u32 = 0x3 << 18;

pub const MODER_MODER8_POS: u32 = 16;
pub const MODER_MODER8_WIDTH: u32 = 2;
pub const MODER_MODER8_MASK: u32 = 0x3 << 16;

pub const MODER_MODER7_POS: u32 = 14;
pub const MODER_MODER7_WIDTH: u32 = 2;
pub const MODER_MODER7_MASK: u32 = 0x3 << 14;

pub const MODER_MODER6_POS: u32 = 12;
pub const MODER_MODER6_WIDTH: u32 = 2;
pub const MODER_MODER6_MASK: u32 = 0x3 << 12;

pub const MODER_MODER5_POS: u32 = 10;
pub const MODER_MODER5_WIDTH: u32 = 2;
pub const MODER_MODER5_MASK: u32 = 0x3 << 10;

pub const MODER_MODER4_POS: u32 = 8;
pub const MODER_MODER4_WIDTH: u32 = 2;
pub const MODER_MODER4_MASK: u32 = 0x3 << 8;

pub const MODER_MODER3_POS: u32 = 6;
pub const MODER_MODER3_WIDTH: u32 = 2;
pub const MODER_MODER3_MASK: u32 = 0x3 << 6;

pub const MODER_MODER2_POS: u32 = 4;
pub const MODER_MODER2_WIDTH: u32 = 2;
pub const MODER_MODER2_MASK: u32 = 0x3 << 4;

pub const MODER_MODER1_POS: u32 = 2;
pub const MODER_MODER1_WIDTH: u32 = 2;
pub const MODER_MODER1_MASK: u32 = 0x3 << 2;

pub const MODER_MODER0_POS: u32 = 0;
pub const MODER_MODER0_WIDTH: u32 = 2;
pub const MODER_MODER0_MASK: u32 = 0x3 << 0;
// MODER0 enumerated values (same for all pins)
pub const MODER_INPUT: u32 = 0;
pub const MODER_OUTPUT: u32 = 1;
pub const MODER_ALTERNATE: u32 = 2;
pub const MODER_ANALOG: u32 = 3;

// OTYPER register fields
pub const OTYPER_OT15_POS: u32 = 15;
pub const OTYPER_OT15_WIDTH: u32 = 1;
pub const OTYPER_OT15_MASK: u32 = 0x1 << 15;

pub const OTYPER_OT14_POS: u32 = 14;
pub const OTYPER_OT14_WIDTH: u32 = 1;
pub const OTYPER_OT14_MASK: u32 = 0x1 << 14;

pub const OTYPER_OT13_POS: u32 = 13;
pub const OTYPER_OT13_WIDTH: u32 = 1;
pub const OTYPER_OT13_MASK: u32 = 0x1 << 13;

pub const OTYPER_OT12_POS: u32 = 12;
pub const OTYPER_OT12_WIDTH: u32 = 1;
pub const OTYPER_OT12_MASK: u32 = 0x1 << 12;

pub const OTYPER_OT11_POS: u32 = 11;
pub const OTYPER_OT11_WIDTH: u32 = 1;
pub const OTYPER_OT11_MASK: u32 = 0x1 << 11;

pub const OTYPER_OT10_POS: u32 = 10;
pub const OTYPER_OT10_WIDTH: u32 = 1;
pub const OTYPER_OT10_MASK: u32 = 0x1 << 10;

pub const OTYPER_OT9_POS: u32 = 9;
pub const OTYPER_OT9_WIDTH: u32 = 1;
pub const OTYPER_OT9_MASK: u32 = 0x1 << 9;

pub const OTYPER_OT8_POS: u32 = 8;
pub const OTYPER_OT8_WIDTH: u32 = 1;
pub const OTYPER_OT8_MASK: u32 = 0x1 << 8;

pub const OTYPER_OT7_POS: u32 = 7;
pub const OTYPER_OT7_WIDTH: u32 = 1;
pub const OTYPER_OT7_MASK: u32 = 0x1 << 7;

pub const OTYPER_OT6_POS: u32 = 6;
pub const OTYPER_OT6_WIDTH: u32 = 1;
pub const OTYPER_OT6_MASK: u32 = 0x1 << 6;

pub const OTYPER_OT5_POS: u32 = 5;
pub const OTYPER_OT5_WIDTH: u32 = 1;
pub const OTYPER_OT5_MASK: u32 = 0x1 << 5;

pub const OTYPER_OT4_POS: u32 = 4;
pub const OTYPER_OT4_WIDTH: u32 = 1;
pub const OTYPER_OT4_MASK: u32 = 0x1 << 4;

pub const OTYPER_OT3_POS: u32 = 3;
pub const OTYPER_OT3_WIDTH: u32 = 1;
pub const OTYPER_OT3_MASK: u32 = 0x1 << 3;

pub const OTYPER_OT2_POS: u32 = 2;
pub const OTYPER_OT2_WIDTH: u32 = 1;
pub const OTYPER_OT2_MASK: u32 = 0x1 << 2;

pub const OTYPER_OT1_POS: u32 = 1;
pub const OTYPER_OT1_WIDTH: u32 = 1;
pub const OTYPER_OT1_MASK: u32 = 0x1 << 1;

pub const OTYPER_OT0_POS: u32 = 0;
pub const OTYPER_OT0_WIDTH: u32 = 1;
pub const OTYPER_OT0_MASK: u32 = 0x1 << 0;
// OTYPER enumerated values (same for all pins)
pub const OTYPER_PUSHPULL: u32 = 0;
pub const OTYPER_OPENDRAIN: u32 = 1;

// OSPEEDR register fields
pub const OSPEEDR_OSPEEDR15_POS: u32 = 30;
pub const OSPEEDR_OSPEEDR15_WIDTH: u32 = 2;
pub const OSPEEDR_OSPEEDR15_MASK: u32 = 0x3 << 30;

pub const OSPEEDR_OSPEEDR14_POS: u32 = 28;
pub const OSPEEDR_OSPEEDR14_WIDTH: u32 = 2;
pub const OSPEEDR_OSPEEDR14_MASK: u32 = 0x3 << 28;

pub const OSPEEDR_OSPEEDR13_POS: u32 = 26;
pub const OSPEEDR_OSPEEDR13_WIDTH: u32 = 2;
pub const OSPEEDR_OSPEEDR13_MASK: u32 = 0x3 << 26;

pub const OSPEEDR_OSPEEDR12_POS: u32 = 24;
pub const OSPEEDR_OSPEEDR12_WIDTH: u32 = 2;
pub const OSPEEDR_OSPEEDR12_MASK: u32 = 0x3 << 24;

pub const OSPEEDR_OSPEEDR11_POS: u32 = 22;
pub const OSPEEDR_OSPEEDR11_WIDTH: u32 = 2;
pub const OSPEEDR_OSPEEDR11_MASK: u32 = 0x3 << 22;

pub const OSPEEDR_OSPEEDR10_POS: u32 = 20;
pub const OSPEEDR_OSPEEDR10_WIDTH: u32 = 2;
pub const OSPEEDR_OSPEEDR10_MASK: u32 = 0x3 << 20;

pub const OSPEEDR_OSPEEDR9_POS: u32 = 18;
pub const OSPEEDR_OSPEEDR9_WIDTH: u32 = 2;
pub const OSPEEDR_OSPEEDR9_MASK: u32 = 0x3 << 18;

pub const OSPEEDR_OSPEEDR8_POS: u32 = 16;
pub const OSPEEDR_OSPEEDR8_WIDTH: u32 = 2;
pub const OSPEEDR_OSPEEDR8_MASK: u32 = 0x3 << 16;

pub const OSPEEDR_OSPEEDR7_POS: u32 = 14;
pub const OSPEEDR_OSPEEDR7_WIDTH: u32 = 2;
pub const OSPEEDR_OSPEEDR7_MASK: u32 = 0x3 << 14;

pub const OSPEEDR_OSPEEDR6_POS: u32 = 12;
pub const OSPEEDR_OSPEEDR6_WIDTH: u32 = 2;
pub const OSPEEDR_OSPEEDR6_MASK: u32 = 0x3 << 12;

pub const OSPEEDR_OSPEEDR5_POS: u32 = 10;
pub const OSPEEDR_OSPEEDR5_WIDTH: u32 = 2;
pub const OSPEEDR_OSPEEDR5_MASK: u32 = 0x3 << 10;

pub const OSPEEDR_OSPEEDR4_POS: u32 = 8;
pub const OSPEEDR_OSPEEDR4_WIDTH: u32 = 2;
pub const OSPEEDR_OSPEEDR4_MASK: u32 = 0x3 << 8;

pub const OSPEEDR_OSPEEDR3_POS: u32 = 6;
pub const OSPEEDR_OSPEEDR3_WIDTH: u32 = 2;
pub const OSPEEDR_OSPEEDR3_MASK: u32 = 0x3 << 6;

pub const OSPEEDR_OSPEEDR2_POS: u32 = 4;
pub const OSPEEDR_OSPEEDR2_WIDTH: u32 = 2;
pub const OSPEEDR_OSPEEDR2_MASK: u32 = 0x3 << 4;

pub const OSPEEDR_OSPEEDR1_POS: u32 = 2;
pub const OSPEEDR_OSPEEDR1_WIDTH: u32 = 2;
pub const OSPEEDR_OSPEEDR1_MASK: u32 = 0x3 << 2;

pub const OSPEEDR_OSPEEDR0_POS: u32 = 0;
pub const OSPEEDR_OSPEEDR0_WIDTH: u32 = 2;
pub const OSPEEDR_OSPEEDR0_MASK: u32 = 0x3 << 0;
// OSPEEDR enumerated values (same for all pins)
pub const OSPEEDR_LOWSPEED: u32 = 0;
pub const OSPEEDR_MEDIUMSPEED: u32 = 1;
pub const OSPEEDR_HIGHSPEED: u32 = 2;
pub const OSPEEDR_VERYHIGHSPEED: u32 = 3;

// PUPDR register fields
pub const PUPDR_PUPDR15_POS: u32 = 30;
pub const PUPDR_PUPDR15_WIDTH: u32 = 2;
pub const PUPDR_PUPDR15_MASK: u32 = 0x3 << 30;

pub const PUPDR_PUPDR14_POS: u32 = 28;
pub const PUPDR_PUPDR14_WIDTH: u32 = 2;
pub const PUPDR_PUPDR14_MASK: u32 = 0x3 << 28;

pub const PUPDR_PUPDR13_POS: u32 = 26;
pub const PUPDR_PUPDR13_WIDTH: u32 = 2;
pub const PUPDR_PUPDR13_MASK: u32 = 0x3 << 26;

pub const PUPDR_PUPDR12_POS: u32 = 24;
pub const PUPDR_PUPDR12_WIDTH: u32 = 2;
pub const PUPDR_PUPDR12_MASK: u32 = 0x3 << 24;

pub const PUPDR_PUPDR11_POS: u32 = 22;
pub const PUPDR_PUPDR11_WIDTH: u32 = 2;
pub const PUPDR_PUPDR11_MASK: u32 = 0x3 << 22;

pub const PUPDR_PUPDR10_POS: u32 = 20;
pub const PUPDR_PUPDR10_WIDTH: u32 = 2;
pub const PUPDR_PUPDR10_MASK: u32 = 0x3 << 20;

pub const PUPDR_PUPDR9_POS: u32 = 18;
pub const PUPDR_PUPDR9_WIDTH: u32 = 2;
pub const PUPDR_PUPDR9_MASK: u32 = 0x3 << 18;

pub const PUPDR_PUPDR8_POS: u32 = 16;
pub const PUPDR_PUPDR8_WIDTH: u32 = 2;
pub const PUPDR_PUPDR8_MASK: u32 = 0x3 << 16;

pub const PUPDR_PUPDR7_POS: u32 = 14;
pub const PUPDR_PUPDR7_WIDTH: u32 = 2;
pub const PUPDR_PUPDR7_MASK: u32 = 0x3 << 14;

pub const PUPDR_PUPDR6_POS: u32 = 12;
pub const PUPDR_PUPDR6_WIDTH: u32 = 2;
pub const PUPDR_PUPDR6_MASK: u32 = 0x3 << 12;

pub const PUPDR_PUPDR5_POS: u32 = 10;
pub const PUPDR_PUPDR5_WIDTH: u32 = 2;
pub const PUPDR_PUPDR5_MASK: u32 = 0x3 << 10;

pub const PUPDR_PUPDR4_POS: u32 = 8;
pub const PUPDR_PUPDR4_WIDTH: u32 = 2;
pub const PUPDR_PUPDR4_MASK: u32 = 0x3 << 8;

pub const PUPDR_PUPDR3_POS: u32 = 6;
pub const PUPDR_PUPDR3_WIDTH: u32 = 2;
pub const PUPDR_PUPDR3_MASK: u32 = 0x3 << 6;

pub const PUPDR_PUPDR2_POS: u32 = 4;
pub const PUPDR_PUPDR2_WIDTH: u32 = 2;
pub const PUPDR_PUPDR2_MASK: u32 = 0x3 << 4;

pub const PUPDR_PUPDR1_POS: u32 = 2;
pub const PUPDR_PUPDR1_WIDTH: u32 = 2;
pub const PUPDR_PUPDR1_MASK: u32 = 0x3 << 2;

pub const PUPDR_PUPDR0_POS: u32 = 0;
pub const PUPDR_PUPDR0_WIDTH: u32 = 2;
pub const PUPDR_PUPDR0_MASK: u32 = 0x3 << 0;
// PUPDR enumerated values (same for all pins)
pub const PUPDR_FLOATING: u32 = 0;
pub const PUPDR_PULLUP: u32 = 1;
pub const PUPDR_PULLDOWN: u32 = 2;

// IDR register fields (individual pins for input data)
pub const IDR_IDR15_POS: u32 = 15;
pub const IDR_IDR15_WIDTH: u32 = 1;
pub const IDR_IDR15_MASK: u32 = 0x1 << 15;

pub const IDR_IDR14_POS: u32 = 14;
pub const IDR_IDR14_WIDTH: u32 = 1;
pub const IDR_IDR14_MASK: u32 = 0x1 << 14;

pub const IDR_IDR13_POS: u32 = 13;
pub const IDR_IDR13_WIDTH: u32 = 1;
pub const IDR_IDR13_MASK: u32 = 0x1 << 13;

pub const IDR_IDR12_POS: u32 = 12;
pub const IDR_IDR12_WIDTH: u32 = 1;
pub const IDR_IDR12_MASK: u32 = 0x1 << 12;

pub const IDR_IDR11_POS: u32 = 11;
pub const IDR_IDR11_WIDTH: u32 = 1;
pub const IDR_IDR11_MASK: u32 = 0x1 << 11;

pub const IDR_IDR10_POS: u32 = 10;
pub const IDR_IDR10_WIDTH: u32 = 1;
pub const IDR_IDR10_MASK: u32 = 0x1 << 10;

pub const IDR_IDR9_POS: u32 = 9;
pub const IDR_IDR9_WIDTH: u32 = 1;
pub const IDR_IDR9_MASK: u32 = 0x1 << 9;

pub const IDR_IDR8_POS: u32 = 8;
pub const IDR_IDR8_WIDTH: u32 = 1;
pub const IDR_IDR8_MASK: u32 = 0x1 << 8;

pub const IDR_IDR7_POS: u32 = 7;
pub const IDR_IDR7_WIDTH: u32 = 1;
pub const IDR_IDR7_MASK: u32 = 0x1 << 7;

pub const IDR_IDR6_POS: u32 = 6;
pub const IDR_IDR6_WIDTH: u32 = 1;
pub const IDR_IDR6_MASK: u32 = 0x1 << 6;

pub const IDR_IDR5_POS: u32 = 5;
pub const IDR_IDR5_WIDTH: u32 = 1;
pub const IDR_IDR5_MASK: u32 = 0x1 << 5;

pub const IDR_IDR4_POS: u32 = 4;
pub const IDR_IDR4_WIDTH: u32 = 1;
pub const IDR_IDR4_MASK: u32 = 0x1 << 4;

pub const IDR_IDR3_POS: u32 = 3;
pub const IDR_IDR3_WIDTH: u32 = 1;
pub const IDR_IDR3_MASK: u32 = 0x1 << 3;

pub const IDR_IDR2_POS: u32 = 2;
pub const IDR_IDR2_WIDTH: u32 = 1;
pub const IDR_IDR2_MASK: u32 = 0x1 << 2;

pub const IDR_IDR1_POS: u32 = 1;
pub const IDR_IDR1_WIDTH: u32 = 1;
pub const IDR_IDR1_MASK: u32 = 0x1 << 1;

pub const IDR_IDR0_POS: u32 = 0;
pub const IDR_IDR0_WIDTH: u32 = 1;
pub const IDR_IDR0_MASK: u32 = 0x1 << 0;

// ODR register fields (individual pins for output data)
pub const ODR_ODR15_POS: u32 = 15;
pub const ODR_ODR15_WIDTH: u32 = 1;
pub const ODR_ODR15_MASK: u32 = 0x1 << 15;

pub const ODR_ODR14_POS: u32 = 14;
pub const ODR_ODR14_WIDTH: u32 = 1;
pub const ODR_ODR14_MASK: u32 = 0x1 << 14;

pub const ODR_ODR13_POS: u32 = 13;
pub const ODR_ODR13_WIDTH: u32 = 1;
pub const ODR_ODR13_MASK: u32 = 0x1 << 13;

pub const ODR_ODR12_POS: u32 = 12;
pub const ODR_ODR12_WIDTH: u32 = 1;
pub const ODR_ODR12_MASK: u32 = 0x1 << 12;

pub const ODR_ODR11_POS: u32 = 11;
pub const ODR_ODR11_WIDTH: u32 = 1;
pub const ODR_ODR11_MASK: u32 = 0x1 << 11;

pub const ODR_ODR10_POS: u32 = 10;
pub const ODR_ODR10_WIDTH: u32 = 1;
pub const ODR_ODR10_MASK: u32 = 0x1 << 10;

pub const ODR_ODR9_POS: u32 = 9;
pub const ODR_ODR9_WIDTH: u32 = 1;
pub const ODR_ODR9_MASK: u32 = 0x1 << 9;

pub const ODR_ODR8_POS: u32 = 8;
pub const ODR_ODR8_WIDTH: u32 = 1;
pub const ODR_ODR8_MASK: u32 = 0x1 << 8;

pub const ODR_ODR7_POS: u32 = 7;
pub const ODR_ODR7_WIDTH: u32 = 1;
pub const ODR_ODR7_MASK: u32 = 0x1 << 7;

pub const ODR_ODR6_POS: u32 = 6;
pub const ODR_ODR6_WIDTH: u32 = 1;
pub const ODR_ODR6_MASK: u32 = 0x1 << 6;

pub const ODR_ODR5_POS: u32 = 5;
pub const ODR_ODR5_WIDTH: u32 = 1;
pub const ODR_ODR5_MASK: u32 = 0x1 << 5;

pub const ODR_ODR4_POS: u32 = 4;
pub const ODR_ODR4_WIDTH: u32 = 1;
pub const ODR_ODR4_MASK: u32 = 0x1 << 4;

pub const ODR_ODR3_POS: u32 = 3;
pub const ODR_ODR3_WIDTH: u32 = 1;
pub const ODR_ODR3_MASK: u32 = 0x1 << 3;

pub const ODR_ODR2_POS: u32 = 2;
pub const ODR_ODR2_WIDTH: u32 = 1;
pub const ODR_ODR2_MASK: u32 = 0x1 << 2;

pub const ODR_ODR1_POS: u32 = 1;
pub const ODR_ODR1_WIDTH: u32 = 1;
pub const ODR_ODR1_MASK: u32 = 0x1 << 1;

pub const ODR_ODR0_POS: u32 = 0;
pub const ODR_ODR0_WIDTH: u32 = 1;
pub const ODR_ODR0_MASK: u32 = 0x1 << 0;

// GPIO Mode enumeration
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GpioMode {
    Input = 0,
    Output = 1,
    Alternate = 2,
    Analog = 3,
}

// GPIO Output Type enumeration
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GpioOutputType {
    PushPull = 0,
    OpenDrain = 1,
}

// GPIO Speed enumeration
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GpioSpeed {
    Low = 0,
    Medium = 1,
    High = 2,
    VeryHigh = 3,
}

// GPIO Pull-up/Pull-down enumeration
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GpioPull {
    None = 0,
    PullUp = 1,
    PullDown = 2,
}

// Helper functions for GPIO configuration
impl RegisterBlock {
    /// Configure a pin mode
    pub fn set_pin_mode(&mut self, pin: u8, mode: GpioMode) {
        if pin < 16 {
            let shift = pin * 2;
            let mask = 0x3 << shift;
            self.moder = (self.moder & !mask) | ((mode as u32) << shift);
        }
    }

    /// Set pin output type
    pub fn set_pin_output_type(&mut self, pin: u8, output_type: GpioOutputType) {
        if pin < 16 {
            if output_type == GpioOutputType::OpenDrain {
                self.otyper |= 1 << pin;
            } else {
                self.otyper &= !(1 << pin);
            }
        }
    }

    /// Set pin speed
    pub fn set_pin_speed(&mut self, pin: u8, speed: GpioSpeed) {
        if pin < 16 {
            let shift = pin * 2;
            let mask = 0x3 << shift;
            self.ospeedr = (self.ospeedr & !mask) | ((speed as u32) << shift);
        }
    }

    /// Set pin pull-up/pull-down
    pub fn set_pin_pull(&mut self, pin: u8, pull: GpioPull) {
        if pin < 16 {
            let shift = pin * 2;
            let mask = 0x3 << shift;
            self.pupdr = (self.pupdr & !mask) | ((pull as u32) << shift);
        }
    }

    /// Set pin output value
    pub fn set_pin(&mut self, pin: u8, value: bool) {
        if pin < 16 {
            if value {
                self.bsrr = 1 << pin; // Set bit
            } else {
                self.bsrr = 1 << (pin + 16); // Reset bit
            }
        }
    }

    /// Get pin input value
    pub fn get_pin(&self, pin: u8) -> bool {
        if pin < 16 {
            (self.idr & (1 << pin)) != 0
        } else {
            false
        }
    }

    /// Toggle pin output value
    pub fn toggle_pin(&mut self, pin: u8) {
        if pin < 16 {
            let current_value = (self.odr & (1 << pin)) != 0;
            self.set_pin(pin, !current_value);
        }
    }
}
