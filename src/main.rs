#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

// use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f4::stm32f411;

#[entry]
fn main() -> ! {
    let peripherals = stm32f411::Peripherals::take().unwrap();
    let rcc = &peripherals.RCC;
    rcc.ahb1enr.write(|w| w.gpioden().set_bit());

    let gpiod = &peripherals.GPIOD;
    gpiod.moder.write(|w| w.moder13().bits(0b01));
    gpiod.odr.modify(|_, w| w.odr13().set_bit());

    loop {
        // your code goes here
    }
}
