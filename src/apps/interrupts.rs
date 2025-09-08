use crate::arch::cortex_m4::systick;
use cortex_m_rt::exception;

#[exception]
fn SysTick() {
    systick::increment_ticks();
}
