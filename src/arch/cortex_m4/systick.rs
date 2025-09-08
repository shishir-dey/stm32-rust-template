// SysTick (System Timer) register definitions
// Based on CMSIS Cortex-M4 core_cm4.h

use super::super::super::mcu::stm32f407::PeripheralAccess;
use core::sync::atomic::{AtomicU32, Ordering};

// SysTick Base Address
pub const SYSTICK_BASE: u32 = 0xE000E010;

// SysTick Register Block
#[repr(C)]
pub struct SysTick {
    pub ctrl: u32,  // SysTick Control and Status Register
    pub load: u32,  // SysTick Reload Value Register
    pub val: u32,   // SysTick Current Value Register
    pub calib: u32, // SysTick Calibration Register
}

// SysTick Peripheral Instance
pub struct SYSTICK;

impl PeripheralAccess for SYSTICK {
    const BASE_ADDRESS: u32 = SYSTICK_BASE;
    type RegisterBlock = SysTick;
}

// Global tick counter
static TICK_COUNT: AtomicU32 = AtomicU32::new(0);

// Get current tick count
pub fn get_ticks() -> u32 {
    TICK_COUNT.load(Ordering::Relaxed)
}

// Increment tick count (called from interrupt handler)
pub fn increment_ticks() {
    TICK_COUNT.fetch_add(1, Ordering::Relaxed);
}

// SysTick Control / Status Register Definitions
pub const SYSTICK_CTRL_COUNTFLAG_POS: u32 = 16;
pub const SYSTICK_CTRL_COUNTFLAG_MSK: u32 = 1 << SYSTICK_CTRL_COUNTFLAG_POS;

pub const SYSTICK_CTRL_CLKSOURCE_POS: u32 = 2;
pub const SYSTICK_CTRL_CLKSOURCE_MSK: u32 = 1 << SYSTICK_CTRL_CLKSOURCE_POS;

pub const SYSTICK_CTRL_TICKINT_POS: u32 = 1;
pub const SYSTICK_CTRL_TICKINT_MSK: u32 = 1 << SYSTICK_CTRL_TICKINT_POS;

pub const SYSTICK_CTRL_ENABLE_POS: u32 = 0;
pub const SYSTICK_CTRL_ENABLE_MSK: u32 = 1;

// SysTick Reload Register Definitions
pub const SYSTICK_LOAD_RELOAD_POS: u32 = 0;
pub const SYSTICK_LOAD_RELOAD_MSK: u32 = 0xFFFFFF;

// SysTick Current Register Definitions
pub const SYSTICK_VAL_CURRENT_POS: u32 = 0;
pub const SYSTICK_VAL_CURRENT_MSK: u32 = 0xFFFFFF;

// SysTick Calibration Register Definitions
pub const SYSTICK_CALIB_NOREF_POS: u32 = 31;
pub const SYSTICK_CALIB_NOREF_MSK: u32 = 1 << SYSTICK_CALIB_NOREF_POS;

pub const SYSTICK_CALIB_SKEW_POS: u32 = 30;
pub const SYSTICK_CALIB_SKEW_MSK: u32 = 1 << SYSTICK_CALIB_SKEW_POS;

pub const SYSTICK_CALIB_TENMS_POS: u32 = 0;
pub const SYSTICK_CALIB_TENMS_MSK: u32 = 0xFFFFFF;

// Helper functions for SysTick
impl SysTick {
    /// Enable SysTick
    pub fn enable(&mut self) {
        self.ctrl |= SYSTICK_CTRL_ENABLE_MSK;
    }

    /// Disable SysTick
    pub fn disable(&mut self) {
        self.ctrl &= !SYSTICK_CTRL_ENABLE_MSK;
    }

    /// Enable SysTick interrupt
    pub fn enable_interrupt(&mut self) {
        self.ctrl |= SYSTICK_CTRL_TICKINT_MSK;
    }

    /// Disable SysTick interrupt
    pub fn disable_interrupt(&mut self) {
        self.ctrl &= !SYSTICK_CTRL_TICKINT_MSK;
    }

    /// Set clock source to processor clock
    pub fn set_clock_source_processor(&mut self) {
        self.ctrl |= SYSTICK_CTRL_CLKSOURCE_MSK;
    }

    /// Set clock source to external clock
    pub fn set_clock_source_external(&mut self) {
        self.ctrl &= !SYSTICK_CTRL_CLKSOURCE_MSK;
    }

    /// Set reload value
    pub fn set_reload(&mut self, value: u32) {
        self.load = value & SYSTICK_LOAD_RELOAD_MSK;
    }

    /// Get current value
    pub fn get_current(&self) -> u32 {
        self.val & SYSTICK_VAL_CURRENT_MSK
    }

    /// Clear current value (write any value to clear)
    pub fn clear_current(&mut self) {
        self.val = 0;
    }

    /// Check if count flag is set
    pub fn is_count_flag_set(&self) -> bool {
        (self.ctrl & SYSTICK_CTRL_COUNTFLAG_MSK) != 0
    }

    /// Get calibration value for 10ms
    pub fn get_tenms(&self) -> u32 {
        self.calib & SYSTICK_CALIB_TENMS_MSK
    }

    /// Check if calibration has no reference clock
    pub fn has_no_reference(&self) -> bool {
        (self.calib & SYSTICK_CALIB_NOREF_MSK) != 0
    }

    /// Check if calibration is skewed
    pub fn is_skewed(&self) -> bool {
        (self.calib & SYSTICK_CALIB_SKEW_MSK) != 0
    }
}

/// SysTick Configuration
/// Initializes the System Timer and its interrupt, and starts the System Tick Timer.
/// Counter is in free running mode to generate periodic interrupts.
/// Returns 0 on success, 1 on failure.
pub fn systick_config(ticks: u32) -> u32 {
    if (ticks - 1) > SYSTICK_LOAD_RELOAD_MSK {
        return 1; // Reload value impossible
    }

    unsafe {
        let systick = &mut *(SYSTICK_BASE as *mut SysTick);
        systick.set_reload(ticks - 1);
        systick.clear_current();
        systick.set_clock_source_processor();
        systick.enable_interrupt();
        systick.enable();
    }

    0
}

/// Configure SysTick for 1ms interrupts based on system clock frequency
/// freq_hz: System clock frequency in Hz (e.g., 168000000 for 168MHz)
pub fn systick_init_1ms(freq_hz: u32) -> u32 {
    let ticks = freq_hz / 1000; // 1ms ticks
    systick_config(ticks)
}
