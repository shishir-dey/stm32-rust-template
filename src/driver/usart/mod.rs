#![allow(dead_code)]

use bitflags::bitflags;
use core::ops::FnMut;

/// Defines the possible modes for USART communication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// UART (Asynchronous)
    Asynchronous,
    /// Synchronous Master (generates clock signal)
    SynchronousMaster,
    /// Synchronous Slave (external clock signal)
    SynchronousSlave,
    /// UART Single-wire (half-duplex)
    SingleWire,
    /// UART IrDA
    IrDA,
    /// UART Smart Card
    SmartCard,
}

/// Defines the number of data bits in a frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataBits {
    Bits5 = 5,
    Bits6 = 6,
    Bits7 = 7,
    /// Default
    Bits8 = 8,
    Bits9 = 9,
}

/// Defines the parity generation and checking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parity {
    /// Default
    None,
    Even,
    Odd,
}

/// Defines the number of stop bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopBits {
    /// Default
    Bits1,
    Bits2,
    Bits1_5,
    Bits0_5,
}

/// Defines the flow control type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowControl {
    /// Default
    None,
    RTS,
    CTS,
    RTS_CTS,
}

/// Defines the clock polarity for synchronous modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockPolarity {
    /// Default
    CPOL0,
    CPOL1,
}

/// Defines the clock phase for synchronous modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockPhase {
    /// Default
    CPHA0,
    CPHA1,
}

/// Defines the modem control lines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModemControl {
    RTSClear,
    RTSSet,
    DTRClear,
    DTRSet,
}

/// Represents the status of the USART peripheral.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Status {
    /// Transmitter busy flag
    pub tx_busy: bool,
    /// Receiver busy flag
    pub rx_busy: bool,
    /// Transmit data underflow detected
    pub tx_underflow: bool,
    /// Receive data overflow detected
    pub rx_overflow: bool,
    /// Break detected on receive
    pub rx_break: bool,
    /// Framing error detected on receive
    pub rx_framing_error: bool,
    /// Parity error detected on receive
    pub rx_parity_error: bool,
}

/// Represents the status of the modem signals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModemStatus {
    /// CTS state: true=Active, false=Inactive
    pub cts: bool,
    /// DSR state: true=Active, false=Inactive
    pub dsr: bool,
    /// DCD state: true=Active, false=Inactive
    pub dcd: bool,
    /// RI state: true=Active, false=Inactive
    pub ri: bool,
}

bitflags! {
    /// Represents USART communication events.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Event: u32 {
        /// Send completed
        const SEND_COMPLETE = (1 << 0);
        /// Receive completed
        const RECEIVE_COMPLETE = (1 << 1);
        /// Transfer completed
        const TRANSFER_COMPLETE = (1 << 2);
        /// Transmit completed
        const TX_COMPLETE = (1 << 3);
        /// Transmit data not available
        const TX_UNDERFLOW = (1 << 4);
        /// Receive data overflow
        const RX_OVERFLOW = (1 << 5);
        /// Receive character timeout
        const RX_TIMEOUT = (1 << 6);
        /// Break detected on receive
        const RX_BREAK = (1 << 7);
        /// Framing error detected on receive
        const RX_FRAMING_ERROR = (1 << 8);
        /// Parity error detected on receive
        const RX_PARITY_ERROR = (1 << 9);
        /// CTS state changed
        const CTS = (1 << 10);
        /// DSR state changed
        const DSR = (1 << 11);
        /// DCD state changed
        const DCD = (1 << 12);
        /// RI state changed
        const RI = (1 << 13);
    }
}

/// A generic error type for the USART driver, using i32 for error codes.
pub type Error = i32;

/// A specialized Result type for USART operations.
pub type Result<T> = core::result::Result<T, Error>;

/// Holds the configuration for a USART peripheral.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub mode: Mode,
    pub baudrate: u32,
    pub data_bits: DataBits,
    pub parity: Parity,
    pub stop_bits: StopBits,
    pub flow_control: FlowControl,
    /// For synchronous modes
    pub clock_polarity: ClockPolarity,
    /// For synchronous modes
    pub clock_phase: ClockPhase,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: Mode::Asynchronous,
            baudrate: 115200,
            data_bits: DataBits::Bits8,
            parity: Parity::None,
            stop_bits: StopBits::Bits1,
            flow_control: FlowControl::None,
            clock_polarity: ClockPolarity::CPOL0,
            clock_phase: ClockPhase::CPHA0,
        }
    }
}

/// A trait that defines a standard interface for a USART driver.
pub trait Usart<'a> {
    /// Initializes the USART peripheral.
    ///
    /// The provided callback will be invoked to signal communication events.
    fn initialize(&mut self, callback: impl FnMut(Event) + 'a) -> Result<()>;

    /// De-initializes the USART peripheral.
    fn uninitialize(&mut self) -> Result<()>;

    /// Configures the USART peripheral.
    fn configure(&mut self, config: &Config) -> Result<()>;

    /// Transmits data over the USART bus.
    fn send(&mut self, data: &[u8]) -> Result<()>;

    /// Receives data from the USART bus.
    fn receive(&mut self, data: &mut [u8]) -> Result<()>;

    /// Simultaneously sends and receives data.
    fn transfer(&mut self, data_out: &[u8], data_in: &mut [u8]) -> Result<()>;

    /// Gets the number of bytes transmitted.
    fn get_tx_count(&self) -> u32;

    /// Gets the number of bytes received.
    fn get_rx_count(&self) -> u32;

    /// Gets the current status of the USART peripheral.
    fn get_status(&self) -> Status;

    /// Controls the modem lines.
    fn set_modem_control(&mut self, control: ModemControl) -> Result<()>;

    /// Gets the status of the modem lines.
    fn get_modem_status(&self) -> ModemStatus;

    /// Enables or disables the transmitter.
    fn tx_enable(&mut self, enable: bool) -> Result<()>;

    /// Enables or disables the receiver.
    fn rx_enable(&mut self, enable: bool) -> Result<()>;

    /// Controls the generation of a break condition.
    fn control_break(&mut self, enable: bool) -> Result<()>;

    /// Aborts an ongoing send operation.
    fn abort_send(&mut self) -> Result<()>;

    /// Aborts an ongoing receive operation.
    fn abort_receive(&mut self) -> Result<()>;

    /// Aborts an ongoing transfer operation.
    fn abort_transfer(&mut self) -> Result<()>;
}

#[cfg(feature = "stm32f407")]
pub mod stm32f407;

#[cfg(feature = "stm32f401")]
pub mod stm32f401;

#[cfg(feature = "stm32f411")]
pub mod stm32f411;

#[cfg(feature = "stm32f103")]
pub mod stm32f103;
