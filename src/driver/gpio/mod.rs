#![allow(dead_code)]

use bitflags::bitflags;
use core::ops::FnMut;

/// Represents a GPIO pin identifier.
/// The interpretation of this identifier is implementation-specific.
/// For example, it could be a simple number (0-15) for a single port,
/// or a value like `(port << 4) | pin` for multiple ports.
pub type Pin = u32;

/// Defines the direction of a GPIO pin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Input (default)
    Input,
    /// Output
    Output,
}

/// Defines the output mode of a GPIO pin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputMode {
    /// Push-pull (default)
    PushPull,
    /// Open-drain
    OpenDrain,
}

/// Defines the internal pull-up or pull-down resistor configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PullResistor {
    /// None (default)
    None,
    /// Pull-up
    PullUp,
    /// Pull-down
    PullDown,
}

/// Defines the trigger condition for an external interrupt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventTrigger {
    /// None (default)
    None,
    /// Rising-edge
    RisingEdge,
    /// Falling-edge
    FallingEdge,
    /// Either edge (rising and falling)
    EitherEdge,
}

bitflags! {
    /// Represents GPIO interrupt events.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct EventType: u32 {
        /// Rising-edge detected
        const RISING_EDGE = (1 << 0);
        /// Falling-edge detected
        const FALLING_EDGE = (1 << 1);
        /// Either edge detected (if supported)
        const EITHER_EDGE = (1 << 2);
    }
}

/// A generic error type for the GPIO driver, using i32 for error codes.
pub type Error = i32;

/// A specialized Result type for GPIO operations.
pub type Result<T> = core::result::Result<T, Error>;

/// A trait that defines a standard interface for a GPIO driver.
/// This trait manages a collection of GPIO pins.
pub trait Gpio<'a> {
    /// Initializes a GPIO pin and registers a callback for events.
    fn setup(&mut self, pin: Pin, callback: impl FnMut(Pin, EventType) + 'a) -> Result<()>;

    /// Sets the direction of a GPIO pin.
    fn set_direction(&mut self, pin: Pin, direction: Direction) -> Result<()>;

    /// Sets the output mode of a GPIO pin.
    fn set_output_mode(&mut self, pin: Pin, mode: OutputMode) -> Result<()>;

    /// Sets the internal pull-up or pull-down resistor for a GPIO pin.
    fn set_pull_resistor(&mut self, pin: Pin, resistor: PullResistor) -> Result<()>;

    /// Sets the event trigger for a GPIO pin.
    fn set_event_trigger(&mut self, pin: Pin, trigger: EventTrigger) -> Result<()>;

    /// Sets the output level of a GPIO pin.
    /// `value` is true for high, false for low.
    fn set_output(&mut self, pin: Pin, value: bool);

    /// Gets the input level of a GPIO pin.
    /// Returns true for high, false for low.
    fn get_input(&self, pin: Pin) -> bool;
}

#[cfg(feature = "stm32f407")]
pub mod stm32f407;

#[cfg(feature = "stm32f401")]
pub mod stm32f401;

#[cfg(feature = "stm32f411")]
pub mod stm32f411;

#[cfg(feature = "stm32f103")]
pub mod stm32f103;
