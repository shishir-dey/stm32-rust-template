//! Simple blink application using direct register manipulation
//!
//! This app demonstrates direct register manipulation to blink an LED on PD12
//! following the exact same sequence as the working C code.

use crate::mcu::stm32f407;

/// Simple blink application that directly manipulates registers
pub struct BlinkApp {
    initialized: bool,
}

impl BlinkApp {
    /// Create a new blink app instance
    pub fn new() -> Self {
        Self { initialized: false }
    }

    /// Initialize the blink app - follows the C code initialization steps
    pub fn init(&mut self) -> Result<(), i32> {
        if self.initialized {
            return Ok(());
        }

        // Step 1: Enable the clock for GPIOD peripheral in the AHB1ENR (SET the 3rd bit position)
        // Address: 0x40023830 (RCC_AHB1ENR register)
        let rcc_ahb1enr = stm32f407::RCC_BASEADDR + 0x30; // RCC_AHB1ENR offset
        unsafe {
            let p_clk_ctrl_reg = rcc_ahb1enr as *mut u32;
            *p_clk_ctrl_reg |= 1 << 3; // Enable GPIOD clock
        }

        // Step 2: Configure the mode of the IO pin as output (PD12)
        // Address: 0x40020C00 (GPIOD_MODER register)
        let gpiod_moder = stm32f407::GPIOD_BASEADDR + 0x00; // GPIOD_MODER offset
        unsafe {
            let p_port_d_mode_reg = gpiod_moder as *mut u32;
            // a. Clear the 24th and 25th bit positions (CLEAR)
            *p_port_d_mode_reg &= !(3 << 24);
            // b. Make 24th bit position as 1 (SET) - configures PD12 as output
            *p_port_d_mode_reg |= 1 << 24;
        }

        self.initialized = true;
        Ok(())
    }

    /// Run the blink sequence - follows the C code main loop
    pub fn run(&mut self) -> ! {
        if !self.initialized {
            // Try to initialize if not already done
            if self.init().is_err() {
                // If initialization fails, just loop forever
                loop {
                    cortex_m::asm::nop();
                }
            }
        }

        // Address: 0x40020C14 (GPIOD_ODR register)
        let gpiod_odr = stm32f407::GPIOD_BASEADDR + 0x14; // GPIOD_ODR offset
        let p_port_d_out_reg = gpiod_odr as *mut u32;

        loop {
            // Step 3: SET 12th bit of the output data register to make I/O pin-12 as HIGH
            unsafe {
                *p_port_d_out_reg |= 1 << 12;
            }

            // Introduce small human observable delay
            // This loop executes for 300K times (same as C code)
            for _ in 0..300000 {
                cortex_m::asm::nop();
            }

            // Turn OFF the LED
            unsafe {
                *p_port_d_out_reg &= !(1 << 12);
            }

            // Same delay
            for _ in 0..300000 {
                cortex_m::asm::nop();
            }
        }
    }
}

/// Global blink app instance
static mut BLINK_APP: Option<BlinkApp> = None;

/// Initialize the global blink app
pub fn init_blink_app() -> Result<(), i32> {
    unsafe {
        BLINK_APP = Some(BlinkApp::new());
        if let Some(ref mut app) = BLINK_APP {
            app.init()?;
        }
    }
    Ok(())
}

/// Run the global blink app (blocking - matches C code behavior)
pub fn run_blink_app() -> ! {
    unsafe {
        if let Some(ref mut app) = BLINK_APP {
            app.run();
        } else {
            // If app is not initialized, just loop forever
            loop {
                cortex_m::asm::nop();
            }
        }
    }
}

/// Create a simple blink app (for compatibility)
pub fn create_simple_blink_app() -> BlinkApp {
    BlinkApp::new()
}
