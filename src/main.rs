#![no_std]
#![no_main]

extern crate alloc;
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

#[entry]
fn main() -> ! {
    // Initialize the allocator
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) };

    // Initialize the blink app
    if let Err(_) = apps::blink::init_blink_app() {
        // Handle initialization error
        loop {
            cortex_m::asm::nop();
        }
    }

    // Run the blink app
    // This function never returns
    apps::blink::run_blink_app();
}
