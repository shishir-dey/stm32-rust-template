#![allow(dead_code)]

use bitflags::bitflags;
use core::ops::FnMut;

/// Defines the audio protocols supported by the SAI peripheral.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    UserDefined,
    I2S,
    MsbJustified,
    LsbJustified,
    PcmShort,
    PcmLong,
    Ac97,
}

/// Defines the data companding options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Companding {
    None,
    ALaw,
    ULaw,
}

/// Defines the bit order for audio data transmission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitOrder {
    MsbFirst,
    LsbFirst,
}

/// Defines the polarity of the clock signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockPolarity {
    FallingEdgeDrive,
    RisingEdgeDrive,
}

/// Defines the direction/mode of the SAI block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Master,
    Slave,
}

/// Defines synchronization mode between transmitter and receiver.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Synchronization {
    Asynchronous,
    Synchronous,
}

/// Defines power states for the SAI peripheral.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    Off,
    Low,
    Full,
}

/// Represents the status of the SAI peripheral.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Status {
    pub tx_busy: bool,
    pub rx_busy: bool,
    pub tx_underflow: bool,
    pub rx_overflow: bool,
    pub frame_error: bool,
}

/// Represents the driver capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capabilities {
    pub asynchronous: bool,
    pub synchronous: bool,
    pub protocol_user: bool,
    pub protocol_i2s: bool,
    pub protocol_justified: bool,
    pub protocol_pcm: bool,
    pub protocol_ac97: bool,
    pub mono_mode: bool,
    pub companding: bool,
    pub mclk_pin: bool,
    pub event_frame_error: bool,
}

bitflags! {
    /// Represents SAI communication events.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Event: u32 {
        const SEND_COMPLETE       = 1 << 0;
        const RECEIVE_COMPLETE    = 1 << 1;
        const TX_UNDERFLOW        = 1 << 2;
        const RX_OVERFLOW         = 1 << 3;
        const FRAME_ERROR         = 1 << 4;
    }
}

/// A generic error type for the SAI driver, using i32 for compatibility.
pub type Error = i32;

/// A specialized Result type for SAI operations.
pub type Result<T> = core::result::Result<T, Error>;

/// Trait defining the standard interface for a SAI driver.
pub trait Sai<'a> {
    /// Initializes the SAI peripheral with a callback for event signaling.
    fn initialize(&mut self, callback: impl FnMut(Event) + 'a) -> Result<()>;

    /// De-initializes the SAI peripheral.
    fn uninitialize(&mut self) -> Result<()>;

    /// Controls the power state of the SAI block.
    fn power_control(&mut self, state: PowerState) -> Result<()>;

    /// Sends audio data over SAI.
    fn send(&mut self, data: &[u8]) -> Result<()>;

    /// Receives audio data over SAI.
    fn receive(&mut self, buffer: &mut [u8]) -> Result<()>;

    /// Gets the number of bytes sent so far.
    fn get_tx_count(&self) -> u32;

    /// Gets the number of bytes received so far.
    fn get_rx_count(&self) -> u32;

    /// Configures the SAI peripheral with user-defined parameters.
    ///
    /// Arguments are typically encoded bitfields (frame length, slot count, etc.).
    fn control(&mut self, control: u32, arg1: u32, arg2: u32) -> Result<()>;

    /// Returns the current status of the SAI block.
    fn get_status(&self) -> Status;
}
