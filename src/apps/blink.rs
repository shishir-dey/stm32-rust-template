use crate::apps::App;
use crate::arch::cortex_m4::systick::get_ticks;
use crate::driver::gpio::stm32f407::{GpioDriver, pins};
use crate::driver::gpio::{Direction, Gpio};

const BOUNCE: [u8; 6] = [0b0001, 0b0010, 0b0100, 0b1000, 0b0100, 0b0010];
const IN_OUT: [u8; 6] = [0b1001, 0b0110, 0b1111, 0b0110, 0b1001, 0b0000];
const TWINKLE: [u8; 8] = [
    0b0001, 0b1000, 0b0010, 0b0100, 0b1000, 0b0001, 0b0100, 0b0010,
];
const CHASE: [u8; 4] = [0b0001, 0b0010, 0b0100, 0b1000];
const FAST_MS: u32 = 120;
const MED_MS: u32 = 220;
const SLOW_MS: u32 = 320;
const SMOKE_DELAY_MS: u32 = 700;
const SMOKE_OFF_DELAY_MS: u32 = 200;

#[derive(Clone, Copy)]
enum State {
    Smoke(u8),
    SmokeDelay(u8),
    SmokeOffDelay,
    Bounce(u8, u8),
    BounceDelay(u8, u8),
    InOut(u8, u8),
    InOutDelay(u8, u8),
    AllOn,
    AllOnDelay,
    AllOff,
    AllOffDelay,
    Twinkle(u8, u8),
    TwinkleDelay(u8, u8),
    Chase(u8),
    ChaseDelay(u8),
}

pub struct BlinkApp {
    gpio_driver: Option<GpioDriver<'static>>,
    initialized: bool,
    state: State,
    last_tick: u32,
}

impl BlinkApp {
    pub fn new() -> Self {
        Self {
            gpio_driver: None,
            initialized: false,
            state: State::Smoke(0),
            last_tick: 0,
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

    pub fn tick(&mut self) {
        if !self.initialized {
            return;
        }
        if let Some(ref mut gpio) = self.gpio_driver {
            match self.state {
                State::Smoke(step) => {
                    if step < 4 {
                        for j in 0..4 {
                            Self::write_mask_4(gpio, if j == step { 1 << j } else { 0 });
                        }
                        self.last_tick = get_ticks();
                        self.state = State::SmokeDelay(step);
                    } else {
                        Self::write_mask_4(gpio, 0);
                        self.last_tick = get_ticks();
                        self.state = State::SmokeOffDelay;
                    }
                }
                State::SmokeDelay(step) => {
                    if get_ticks().wrapping_sub(self.last_tick) >= SMOKE_DELAY_MS {
                        self.state = State::Smoke(step + 1);
                    }
                }
                State::SmokeOffDelay => {
                    if get_ticks().wrapping_sub(self.last_tick) >= SMOKE_OFF_DELAY_MS {
                        self.state = State::Bounce(0, 0);
                    }
                }
                State::Bounce(count, step) => {
                    let m = BOUNCE[step as usize];
                    Self::write_mask_4(gpio, m);
                    self.last_tick = get_ticks();
                    self.state = State::BounceDelay(count, step);
                }
                State::BounceDelay(count, step) => {
                    if get_ticks().wrapping_sub(self.last_tick) >= MED_MS {
                        let next_step = step + 1;
                        if next_step >= BOUNCE.len() as u8 {
                            let next_count = count + 1;
                            if next_count >= 4 {
                                self.state = State::InOut(0, 0);
                            } else {
                                self.state = State::Bounce(next_count, 0);
                            }
                        } else {
                            self.state = State::Bounce(count, next_step);
                        }
                    }
                }
                State::InOut(count, step) => {
                    let m = IN_OUT[step as usize];
                    Self::write_mask_4(gpio, m);
                    self.last_tick = get_ticks();
                    self.state = State::InOutDelay(count, step);
                }
                State::InOutDelay(count, step) => {
                    if get_ticks().wrapping_sub(self.last_tick) >= FAST_MS {
                        let next_step = step + 1;
                        if next_step >= IN_OUT.len() as u8 {
                            let next_count = count + 1;
                            if next_count >= 6 {
                                self.state = State::AllOn;
                            } else {
                                self.state = State::InOut(next_count, 0);
                            }
                        } else {
                            self.state = State::InOut(count, next_step);
                        }
                    }
                }
                State::AllOn => {
                    Self::write_mask_4(gpio, 0b1111);
                    self.last_tick = get_ticks();
                    self.state = State::AllOnDelay;
                }
                State::AllOnDelay => {
                    if get_ticks().wrapping_sub(self.last_tick) >= SLOW_MS {
                        self.state = State::AllOff;
                    }
                }
                State::AllOff => {
                    Self::write_mask_4(gpio, 0b0000);
                    self.last_tick = get_ticks();
                    self.state = State::AllOffDelay;
                }
                State::AllOffDelay => {
                    if get_ticks().wrapping_sub(self.last_tick) >= MED_MS {
                        self.state = State::Twinkle(0, 0);
                    }
                }
                State::Twinkle(count, step) => {
                    let m = TWINKLE[step as usize];
                    Self::write_mask_4(gpio, m);
                    self.last_tick = get_ticks();
                    self.state = State::TwinkleDelay(count, step);
                }
                State::TwinkleDelay(count, step) => {
                    if get_ticks().wrapping_sub(self.last_tick) >= FAST_MS {
                        let next_step = step + 1;
                        if next_step >= TWINKLE.len() as u8 {
                            let next_count = count + 1;
                            if next_count >= 6 {
                                self.state = State::Chase(0);
                            } else {
                                self.state = State::Twinkle(next_count, 0);
                            }
                        } else {
                            self.state = State::Twinkle(count, next_step);
                        }
                    }
                }
                State::Chase(step) => {
                    let m = CHASE[step as usize];
                    Self::write_mask_4(gpio, m);
                    self.last_tick = get_ticks();
                    self.state = State::ChaseDelay(step);
                }
                State::ChaseDelay(step) => {
                    if get_ticks().wrapping_sub(self.last_tick) >= FAST_MS {
                        let next_step = step + 1;
                        if next_step >= CHASE.len() as u8 {
                            self.state = State::Bounce(0, 0);
                        } else {
                            self.state = State::Chase(next_step);
                        }
                    }
                }
            }
        }
    }
}

impl App for BlinkApp {
    fn init(&mut self) -> Result<(), i32> {
        self.init()
    }
    fn loop_step(&mut self) {
        self.tick();
    }
}

pub fn create_simple_blink_app() -> BlinkApp {
    BlinkApp::new()
}
