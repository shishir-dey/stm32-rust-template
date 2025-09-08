#![no_std]
#![no_main]

extern crate alloc;
use alloc::boxed::Box;
use alloc_cortex_m::CortexMHeap;
use cortex_m_rt::entry;
use panic_halt as _;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1024; // 1 KB

mod apps;
mod arch;
mod bsp;
mod components;
mod driver;
mod mcu;
mod utils;

use crate::apps::{init_all_apps, init_app_registry, register_app, run_all_loop_steps};

#[entry]
fn main() -> ! {
    // Initialize the allocator
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) };

    // Initialize SysTick for 1ms interrupts (assuming 16 MHz system clock)
    let _ = arch::cortex_m4::systick::systick_init_1ms(16_000_000);

    // Initialize app registry
    init_app_registry();

    // Register apps
    // register_app(Box::new(apps::empty::create_empty_app()));
    register_app(Box::new(apps::blink::create_simple_blink_app()));

    // Initialize all apps
    if let Err(_) = init_all_apps() {
        // Handle initialization error
        loop {
            cortex_m::asm::nop();
        }
    }

    // Run the main loop
    loop {
        run_all_loop_steps();
    }
}
