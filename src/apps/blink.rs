use crate::driver::gpio::stm32f407::{GpioDriver, pins};
use crate::driver::gpio::{Direction, Gpio};

pub struct BlinkApp<'a> {
    gpio_driver: Option<GpioDriver<'a>>,
    initialized: bool,
}

impl<'a> BlinkApp<'a> {
    pub fn new() -> Self {
        Self {
            gpio_driver: None,
            initialized: false,
        }
    }

    pub fn init(&mut self) -> Result<(), i32> {
        if self.initialized {
            return Ok(());
        }
        let mut gpio = GpioDriver::new_gpiod();

        // Configure PD12..PD15 as outputs and default OFF
        for &p in &[pins::PD12, pins::PD13, pins::PD14, pins::PD15] {
            gpio.set_direction(p, Direction::Output)?;
            gpio.set_output(p, false);
        }

        self.gpio_driver = Some(gpio);
        self.initialized = true;
        Ok(())
    }

    #[inline(always)]
    fn delay(mut cycles: u32) {
        while cycles > 0 {
            cortex_m::asm::nop();
            cycles -= 1;
        }
    }

    /// Write a 4-bit pattern to PD12..PD15 in one shot using BSRR.
    /// bit0→PD12, bit1→PD13, bit2→PD14, bit3→PD15
    fn write_mask_4(gpio: &mut GpioDriver, mask4: u8) {
        // Change this depending on your board's LED polarity:
        const ACTIVE_LOW: bool = false; // set true if LED turns ON when pin = 0

        let pins = [pins::PD12, pins::PD13, pins::PD14, pins::PD15];
        // Build set/clear masks (we still call per-pin BSRR writes; driver is fast)
        for (i, &p) in pins.iter().enumerate() {
            let want_on = ((mask4 >> i) & 1) != 0;
            let level = if ACTIVE_LOW { !want_on } else { want_on };
            gpio.set_output(p, level);
        }
    }

    /// Smoke test: turn on each LED for a moment so you can confirm hardware.
    fn smoke_test(gpio: &mut GpioDriver) {
        for i in 0..4 {
            for j in 0..4 {
                Self::write_mask_4(gpio, if j == i { 1 << j } else { 0 });
            }
            Self::delay(700_000);
        }
        // all off
        Self::write_mask_4(gpio, 0);
        Self::delay(200_000);
    }

    pub fn run(&mut self) -> ! {
        if !self.initialized && self.init().is_err() {
            loop {
                cortex_m::asm::nop();
            }
        }

        let fast = 120_000;
        let med = 220_000;
        let slow = 320_000;

        if let Some(ref mut gpio) = self.gpio_driver {
            // First, prove all 4 work:
            Self::smoke_test(gpio);

            // Pretty patterns (4-bit masks)
            const CHASE: [u8; 4] = [0b0001, 0b0010, 0b0100, 0b1000];
            const BOUNCE: [u8; 6] = [0b0001, 0b0010, 0b0100, 0b1000, 0b0100, 0b0010];
            const IN_OUT: [u8; 6] = [0b1001, 0b0110, 0b1111, 0b0110, 0b1001, 0b0000];
            const TWINKLE: [u8; 8] = [
                0b0001, 0b1000, 0b0010, 0b0100, 0b1000, 0b0001, 0b0100, 0b0010,
            ];

            loop {
                for _ in 0..4 {
                    for &m in &BOUNCE {
                        Self::write_mask_4(gpio, m);
                        Self::delay(med);
                    }
                }
                for _ in 0..6 {
                    for &m in &IN_OUT {
                        Self::write_mask_4(gpio, m);
                        Self::delay(fast);
                    }
                }
                Self::write_mask_4(gpio, 0b1111);
                Self::delay(slow);
                Self::write_mask_4(gpio, 0b0000);
                Self::delay(med);
                for _ in 0..6 {
                    for &m in &TWINKLE {
                        Self::write_mask_4(gpio, m);
                        Self::delay(fast);
                    }
                }
                for &m in &CHASE {
                    Self::write_mask_4(gpio, m);
                    Self::delay(fast);
                }
            }
        } else {
            loop {
                cortex_m::asm::nop();
            }
        }
    }
}

static mut BLINK_APP: Option<BlinkApp<'static>> = None;
pub fn init_blink_app() -> Result<(), i32> {
    unsafe {
        BLINK_APP = Some(BlinkApp::new());
        if let Some(ref mut app) = BLINK_APP {
            app.init()?;
        }
    }
    Ok(())
}
pub fn run_blink_app() -> ! {
    unsafe {
        if let Some(ref mut app) = BLINK_APP {
            app.run();
        } else {
            loop {
                cortex_m::asm::nop();
            }
        }
    }
}
pub fn create_simple_blink_app() -> BlinkApp<'static> {
    BlinkApp::new()
}
