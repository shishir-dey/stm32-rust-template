#[cfg(feature = "stm32f407")]
extern crate alloc;

use super::{Direction, EventTrigger, EventType, Gpio, OutputMode, Pin, PullResistor, Result};
use crate::mcu::stm32f407::{self, gpio, rcc};
use alloc::boxed::Box;
use core::ops::FnMut;

/// Configuration for a GPIO pin
#[derive(Clone, Copy, Debug)]
pub struct GpioConfig {
    pub direction: Direction,
    pub output_mode: OutputMode,
    pub pull_resistor: PullResistor,
    pub event_trigger: EventTrigger,
}

impl Default for GpioConfig {
    fn default() -> Self {
        Self {
            direction: Direction::Input,
            output_mode: OutputMode::PushPull,
            pull_resistor: PullResistor::None,
            event_trigger: EventTrigger::None,
        }
    }
}

/// A GPIO driver for STM32F407
pub struct GpioDriver<'a> {
    /// Callbacks for each pin (indexed by pin number)
    callbacks: [Option<Box<dyn FnMut(Pin, EventType) + 'a>>; 16],
    /// Configuration for each pin
    configs: [GpioConfig; 16],
}

impl<'a> GpioDriver<'a> {
    pub fn new() -> Self {
        Self {
            callbacks: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None,
            ],
            configs: [GpioConfig::default(); 16],
        }
    }

    /// Create a new GPIOA driver instance
    pub fn new_gpioa() -> Self {
        Self::new()
    }

    /// Create a new GPIOB driver instance
    pub fn new_gpiob() -> Self {
        Self::new()
    }

    /// Create a new GPIOC driver instance
    pub fn new_gpioc() -> Self {
        Self::new()
    }

    /// Create a new GPIOD driver instance
    pub fn new_gpiod() -> Self {
        Self::new()
    }

    /// Create a new GPIOE driver instance
    pub fn new_gpioe() -> Self {
        Self::new()
    }

    /// Get GPIO register block for a given port
    fn get_gpio_regs(port: u8) -> *mut gpio::RegisterBlock {
        match port {
            0 => stm32f407::GPIOA_BASEADDR as *mut gpio::RegisterBlock, // GPIOA
            1 => stm32f407::GPIOB_BASEADDR as *mut gpio::RegisterBlock, // GPIOB
            2 => stm32f407::GPIOC_BASEADDR as *mut gpio::RegisterBlock, // GPIOC
            3 => stm32f407::GPIOD_BASEADDR as *mut gpio::RegisterBlock, // GPIOD
            4 => stm32f407::GPIOE_BASEADDR as *mut gpio::RegisterBlock, // GPIOE
            _ => stm32f407::GPIOA_BASEADDR as *mut gpio::RegisterBlock, // Default to GPIOA
        }
    }

    /// Get RCC register block
    fn get_rcc_regs() -> *mut rcc::RegisterBlock {
        stm32f407::RCC_BASEADDR as *mut rcc::RegisterBlock
    }

    /// Enable GPIO port clock
    fn enable_gpio_clock(port: u8) {
        let rcc = unsafe { &mut *Self::get_rcc_regs() };
        let mask = match port {
            0 => rcc::AHB1ENR_GPIOAEN_MASK, // GPIOA
            1 => rcc::AHB1ENR_GPIOBEN_MASK, // GPIOB
            2 => rcc::AHB1ENR_GPIOCEN_MASK, // GPIOC
            3 => rcc::AHB1ENR_GPIODEN_MASK, // GPIOD
            4 => rcc::AHB1ENR_GPIOEEN_MASK, // GPIOE
            _ => rcc::AHB1ENR_GPIOAEN_MASK, // Default to GPIOA
        };
        unsafe {
            rcc.ahb1enr |= mask;
        }
    }

    /// Extract port and pin from Pin identifier
    /// Pin format: (port << 4) | pin_number
    /// where port: 0=GPIOA, 1=GPIOB, 2=GPIOC, 3=GPIOD, 4=GPIOE
    /// and pin_number: 0-15
    fn decode_pin(pin: Pin) -> (u8, u8) {
        let port = ((pin >> 4) & 0xF) as u8;
        let pin_num = (pin & 0xF) as u8;
        (port, pin_num)
    }

    /// Encode port and pin into Pin identifier
    fn encode_pin(port: u8, pin_num: u8) -> Pin {
        ((port as u32) << 4) | (pin_num as u32)
    }

    /// Configure a GPIO pin
    fn configure_pin(&mut self, pin: Pin, config: GpioConfig) -> Result<()> {
        let (port, pin_num) = Self::decode_pin(pin);

        if pin_num >= 16 {
            return Err(-1); // Invalid pin number
        }

        // Enable GPIO port clock
        Self::enable_gpio_clock(port);

        let gpio_regs = unsafe { &mut *Self::get_gpio_regs(port) };

        // Configure pin mode
        let mode_val = match config.direction {
            Direction::Input => gpio::MODER_INPUT,
            Direction::Output => gpio::MODER_OUTPUT,
        };
        let shift = pin_num * 2;
        let mask = 0x3 << shift;
        unsafe {
            gpio_regs.moder = (gpio_regs.moder & !mask) | (mode_val << shift);
        }

        // Configure output type (only relevant for output pins)
        if config.direction == Direction::Output {
            let otype_val = match config.output_mode {
                OutputMode::PushPull => gpio::OTYPER_PUSHPULL,
                OutputMode::OpenDrain => gpio::OTYPER_OPENDRAIN,
            };
            unsafe {
                if otype_val == gpio::OTYPER_OPENDRAIN {
                    gpio_regs.otyper |= 1 << pin_num;
                } else {
                    gpio_regs.otyper &= !(1 << pin_num);
                }
            }
        }

        // Configure pull-up/pull-down
        let pupd_val = match config.pull_resistor {
            PullResistor::None => gpio::PUPDR_FLOATING,
            PullResistor::PullUp => gpio::PUPDR_PULLUP,
            PullResistor::PullDown => gpio::PUPDR_PULLDOWN,
        };
        let shift = pin_num * 2;
        let mask = 0x3 << shift;
        unsafe {
            gpio_regs.pupdr = (gpio_regs.pupdr & !mask) | (pupd_val << shift);
        }

        // Set default output speed to medium
        let shift = pin_num * 2;
        let mask = 0x3 << shift;
        unsafe {
            gpio_regs.ospeedr = (gpio_regs.ospeedr & !mask) | (gpio::OSPEEDR_MEDIUMSPEED << shift);
        }

        // Store configuration
        self.configs[pin_num as usize] = config;

        Ok(())
    }
}

impl<'a> Gpio<'a> for GpioDriver<'a> {
    fn setup(&mut self, pin: Pin, callback: impl FnMut(Pin, EventType) + 'a) -> Result<()> {
        let (_, pin_num) = Self::decode_pin(pin);

        if pin_num >= 16 {
            return Err(-1); // Invalid pin number
        }

        // Store callback
        self.callbacks[pin_num as usize] = Some(Box::new(callback));

        // Configure pin with default settings (input)
        let config = GpioConfig::default();
        self.configure_pin(pin, config)?;

        Ok(())
    }

    fn set_direction(&mut self, pin: Pin, direction: Direction) -> Result<()> {
        let (_, pin_num) = Self::decode_pin(pin);

        if pin_num >= 16 {
            return Err(-1); // Invalid pin number
        }

        let mut config = self.configs[pin_num as usize];
        config.direction = direction;
        self.configure_pin(pin, config)
    }

    fn set_output_mode(&mut self, pin: Pin, mode: OutputMode) -> Result<()> {
        let (_, pin_num) = Self::decode_pin(pin);

        if pin_num >= 16 {
            return Err(-1); // Invalid pin number
        }

        let mut config = self.configs[pin_num as usize];
        config.output_mode = mode;
        self.configure_pin(pin, config)
    }

    fn set_pull_resistor(&mut self, pin: Pin, resistor: PullResistor) -> Result<()> {
        let (_, pin_num) = Self::decode_pin(pin);

        if pin_num >= 16 {
            return Err(-1); // Invalid pin number
        }

        let mut config = self.configs[pin_num as usize];
        config.pull_resistor = resistor;
        self.configure_pin(pin, config)
    }

    fn set_event_trigger(&mut self, pin: Pin, trigger: EventTrigger) -> Result<()> {
        let (_, pin_num) = Self::decode_pin(pin);

        if pin_num >= 16 {
            return Err(-1); // Invalid pin number
        }

        let mut config = self.configs[pin_num as usize];
        config.event_trigger = trigger;
        self.configure_pin(pin, config)?;

        // TODO: Configure EXTI for interrupt-based events
        // For now, we'll just store the configuration

        Ok(())
    }

    fn set_output(&mut self, pin: Pin, value: bool) {
        let (port, pin_num) = Self::decode_pin(pin);

        if pin_num >= 16 {
            return; // Invalid pin number
        }

        let gpio_regs = unsafe { &mut *Self::get_gpio_regs(port) };

        unsafe {
            if value {
                gpio_regs.bsrr = 1 << pin_num; // Set bit
            } else {
                gpio_regs.bsrr = 1 << (pin_num + 16); // Reset bit
            }
        }
    }

    fn get_input(&self, pin: Pin) -> bool {
        let (port, pin_num) = Self::decode_pin(pin);

        if pin_num >= 16 {
            return false; // Invalid pin number
        }

        let gpio_regs = unsafe { &*Self::get_gpio_regs(port) };

        unsafe { (gpio_regs.idr & (1 << pin_num)) != 0 }
    }
}

// Helper functions for creating pin identifiers
pub mod pins {
    use super::Pin;

    // GPIOA pins
    pub const PA0: Pin = 0x00;
    pub const PA1: Pin = 0x01;
    pub const PA2: Pin = 0x02;
    pub const PA3: Pin = 0x03;
    pub const PA4: Pin = 0x04;
    pub const PA5: Pin = 0x05;
    pub const PA6: Pin = 0x06;
    pub const PA7: Pin = 0x07;
    pub const PA8: Pin = 0x08;
    pub const PA9: Pin = 0x09;
    pub const PA10: Pin = 0x0A;
    pub const PA11: Pin = 0x0B;
    pub const PA12: Pin = 0x0C;
    pub const PA13: Pin = 0x0D;
    pub const PA14: Pin = 0x0E;
    pub const PA15: Pin = 0x0F;

    // GPIOB pins
    pub const PB0: Pin = 0x10;
    pub const PB1: Pin = 0x11;
    pub const PB2: Pin = 0x12;
    pub const PB3: Pin = 0x13;
    pub const PB4: Pin = 0x14;
    pub const PB5: Pin = 0x15;
    pub const PB6: Pin = 0x16;
    pub const PB7: Pin = 0x17;
    pub const PB8: Pin = 0x18;
    pub const PB9: Pin = 0x19;
    pub const PB10: Pin = 0x1A;
    pub const PB11: Pin = 0x1B;
    pub const PB12: Pin = 0x1C;
    pub const PB13: Pin = 0x1D;
    pub const PB14: Pin = 0x1E;
    pub const PB15: Pin = 0x1F;

    // GPIOC pins
    pub const PC0: Pin = 0x20;
    pub const PC1: Pin = 0x21;
    pub const PC2: Pin = 0x22;
    pub const PC3: Pin = 0x23;
    pub const PC4: Pin = 0x24;
    pub const PC5: Pin = 0x25;
    pub const PC6: Pin = 0x26;
    pub const PC7: Pin = 0x27;
    pub const PC8: Pin = 0x28;
    pub const PC9: Pin = 0x29;
    pub const PC10: Pin = 0x2A;
    pub const PC11: Pin = 0x2B;
    pub const PC12: Pin = 0x2C;
    pub const PC13: Pin = 0x2D;
    pub const PC14: Pin = 0x2E;
    pub const PC15: Pin = 0x2F;

    // GPIOD pins
    pub const PD0: Pin = 0x30;
    pub const PD1: Pin = 0x31;
    pub const PD2: Pin = 0x32;
    pub const PD3: Pin = 0x33;
    pub const PD4: Pin = 0x34;
    pub const PD5: Pin = 0x35;
    pub const PD6: Pin = 0x36;
    pub const PD7: Pin = 0x37;
    pub const PD8: Pin = 0x38;
    pub const PD9: Pin = 0x39;
    pub const PD10: Pin = 0x3A;
    pub const PD11: Pin = 0x3B;
    pub const PD12: Pin = 0x3C;
    pub const PD13: Pin = 0x3D;
    pub const PD14: Pin = 0x3E;
    pub const PD15: Pin = 0x3F;

    // GPIOE pins
    pub const PE0: Pin = 0x40;
    pub const PE1: Pin = 0x41;
    pub const PE2: Pin = 0x42;
    pub const PE3: Pin = 0x43;
    pub const PE4: Pin = 0x44;
    pub const PE5: Pin = 0x45;
    pub const PE6: Pin = 0x46;
    pub const PE7: Pin = 0x47;
    pub const PE8: Pin = 0x48;
    pub const PE9: Pin = 0x49;
    pub const PE10: Pin = 0x4A;
    pub const PE11: Pin = 0x4B;
    pub const PE12: Pin = 0x4C;
    pub const PE13: Pin = 0x4D;
    pub const PE14: Pin = 0x4E;
    pub const PE15: Pin = 0x4F;
}
