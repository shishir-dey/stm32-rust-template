extern crate alloc;

use super::{
    ClockPhase, ClockPolarity, Config, DataBits, Event, FlowControl, Mode, ModemControl,
    ModemStatus, Parity, Result, Status, Usart,
};
use crate::mcu::stm32f407::{self, usart::*};
use crate::utils;
use alloc::boxed::Box;
use core::ops::FnMut;
use core::ptr;

/// Default APB bus clock frequencies in Hz.
/// Update these to match your clock configuration.
const PCLK1_HZ: u32 = 16_000_000; // APB1 (USART2/3)
const PCLK2_HZ: u32 = 16_000_000; // APB2 (USART1)

/// A polling-based USART driver for STM32F407.
pub struct UsartDriver<'a> {
    regs: *mut RegisterBlock,
    _callback: Option<Box<dyn FnMut(Event) + 'a>>,
    config: Config,
    tx_count: u32,
    rx_count: u32,
}

impl<'a> UsartDriver<'a> {
    pub fn new(usart_base_addr: u32, config: Config) -> Self {
        Self {
            regs: usart_base_addr as *mut RegisterBlock,
            _callback: None,
            config,
            tx_count: 0,
            rx_count: 0,
        }
    }

    /// Create a new USART1 driver instance (APB2 clock)
    pub fn new_usart1(config: Config) -> Self {
        Self::new(stm32f407::USART1_BASEADDR, config)
    }

    /// Create a new USART2 driver instance (APB1 clock)
    pub fn new_usart2(config: Config) -> Self {
        Self::new(stm32f407::USART2_BASEADDR, config)
    }

    /// Create a new USART3 driver instance (APB1 clock)
    pub fn new_usart3(config: Config) -> Self {
        Self::new(stm32f407::USART3_BASEADDR, config)
    }

    fn regs(&self) -> &mut RegisterBlock {
        unsafe { &mut *self.regs }
    }

    fn is_on_apb2(&self) -> bool {
        let base = self.regs as u32;
        base == stm32f407::USART1_BASEADDR
    }

    fn compute_brr(&self, baudrate: u32) -> u32 {
        // Oversampling by 16 (OVER8 = 0)
        let pclk = if self.is_on_apb2() {
            PCLK2_HZ
        } else {
            PCLK1_HZ
        };
        // usartdiv * 16 = pclk / baud (rounded)
        let usartdiv_times_16 = (pclk + (baudrate / 2)) / baudrate;
        let mantissa = usartdiv_times_16 / 16;
        let fraction = usartdiv_times_16 & 0xF;
        (mantissa << 4) | fraction
    }

    fn write_cr1(&mut self, f: impl FnOnce(u32) -> u32) {
        let mut v = unsafe { ptr::read_volatile(&self.regs().cr1) };
        v = f(v);
        unsafe { ptr::write_volatile(&mut self.regs().cr1, v) };
    }

    fn write_cr2(&mut self, f: impl FnOnce(u32) -> u32) {
        let mut v = unsafe { ptr::read_volatile(&self.regs().cr2) };
        v = f(v);
        unsafe { ptr::write_volatile(&mut self.regs().cr2, v) };
    }

    fn write_cr3(&mut self, f: impl FnOnce(u32) -> u32) {
        let mut v = unsafe { ptr::read_volatile(&self.regs().cr3) };
        v = f(v);
        unsafe { ptr::write_volatile(&mut self.regs().cr3, v) };
    }

    fn wait_txe(&self) {
        while unsafe { ptr::read_volatile(&self.regs().sr) } & SR_TXE_MASK == 0 {}
    }

    fn wait_rxne(&self) {
        while unsafe { ptr::read_volatile(&self.regs().sr) } & SR_RXNE_MASK == 0 {}
    }

    fn wait_tc(&self) {
        while unsafe { ptr::read_volatile(&self.regs().sr) } & SR_TC_MASK == 0 {}
    }
}

impl<'a> Usart<'a> for UsartDriver<'a> {
    fn initialize(&mut self, callback: impl FnMut(Event) + 'a) -> Result<()> {
        self._callback = Some(Box::new(callback));

        // Disable USART before configuration
        self.write_cr1(|v| utils::set_bit(v, CR1_UE_POS, false));

        // Apply configuration
        let cfg = self.config.clone();
        self.configure(&cfg)?;

        // Enable USART
        self.write_cr1(|v| utils::set_bit(v, CR1_UE_POS, true));
        Ok(())
    }

    fn uninitialize(&mut self) -> Result<()> {
        self.write_cr1(|v| utils::set_bit(v, CR1_UE_POS, false));
        self._callback = None;
        Ok(())
    }

    fn configure(&mut self, config: &Config) -> Result<()> {
        self.config = config.clone();

        // Disable before reconfig
        self.write_cr1(|v| utils::set_bit(v, CR1_UE_POS, false));

        // CR1: word length, parity, RX/TX enable
        self.write_cr1(|mut v| {
            // Clear fields we manage
            v &= !(CR1_M_MASK
                | CR1_PCE_MASK
                | CR1_PS_MASK
                | CR1_TE_MASK
                | CR1_RE_MASK
                | CR1_OVER8_MASK
                | CR1_RWU_MASK
                | CR1_SBK_MASK
                | CR1_IDLEIE_MASK
                | CR1_RXNEIE_MASK
                | CR1_TCIE_MASK
                | CR1_TXEIE_MASK
                | CR1_PEIE_MASK);

            // Word length
            v = match config.data_bits {
                DataBits::Bits9 => utils::set_bit(v, CR1_M_POS, true),
                DataBits::Bits8 => utils::set_bit(v, CR1_M_POS, false),
                _ => return v, // unsupported values will be handled below by returning Err
            };

            // Parity
            match config.parity {
                Parity::None => {
                    v = utils::set_bit(v, CR1_PCE_POS, false);
                }
                Parity::Even => {
                    v = utils::set_bit(v, CR1_PCE_POS, true);
                    v = utils::set_bit(v, CR1_PS_POS, false);
                }
                Parity::Odd => {
                    v = utils::set_bit(v, CR1_PCE_POS, true);
                    v = utils::set_bit(v, CR1_PS_POS, true);
                }
            }

            // TX/RX enable based on mode
            let enable_tx = matches!(
                config.mode,
                Mode::Asynchronous
                    | Mode::SynchronousMaster
                    | Mode::SynchronousSlave
                    | Mode::SingleWire
                    | Mode::IrDA
                    | Mode::SmartCard
            );
            let enable_rx = matches!(
                config.mode,
                Mode::Asynchronous
                    | Mode::SynchronousMaster
                    | Mode::SynchronousSlave
                    | Mode::SingleWire
                    | Mode::IrDA
                    | Mode::SmartCard
            );
            v = utils::set_bit(v, CR1_TE_POS, enable_tx);
            v = utils::set_bit(v, CR1_RE_POS, enable_rx);

            v
        });

        // Reject unsupported data bits (anything other than 8 or 9)
        match config.data_bits {
            DataBits::Bits8 | DataBits::Bits9 => {}
            _ => return Err(-1),
        }

        // CR2: STOP bits, clock settings for synchronous mode
        self.write_cr2(|mut v| {
            // Clear STOP, CLKEN, CPOL, CPHA, LBCL
            v &= !(CR2_STOP_MASK | CR2_CLKEN_MASK | CR2_CPOL_MASK | CR2_CPHA_MASK | CR2_LBCL_MASK);

            // STOP bits
            let stop_bits_val = match config.stop_bits {
                super::StopBits::Bits1 => 0,
                super::StopBits::Bits0_5 => 1,
                super::StopBits::Bits2 => 2,
                super::StopBits::Bits1_5 => 3,
            } as u32;
            v = utils::set_bits(v, stop_bits_val, CR2_STOP_POS, CR2_STOP_WIDTH);

            // Synchronous clock settings when required
            let sync_mode = matches!(
                config.mode,
                Mode::SynchronousMaster | Mode::SynchronousSlave
            );
            v = utils::set_bit(v, CR2_CLKEN_POS, sync_mode);
            if sync_mode {
                v = utils::set_bit(
                    v,
                    CR2_CPOL_POS,
                    matches!(config.clock_polarity, ClockPolarity::CPOL1),
                );
                v = utils::set_bit(
                    v,
                    CR2_CPHA_POS,
                    matches!(config.clock_phase, ClockPhase::CPHA1),
                );
            }

            v
        });

        // CR3: hardware flow control and special modes
        self.write_cr3(|mut v| {
            // Clear RTSE/CTSE/HDSEL/IREN/SCEN
            v &= !(CR3_RTSE_MASK | CR3_CTSE_MASK | CR3_HDSEL_MASK | CR3_IREN_MASK | CR3_SCEN_MASK);

            // Flow control
            match config.flow_control {
                FlowControl::None => {}
                FlowControl::RTS => {
                    v |= CR3_RTSE_MASK;
                }
                FlowControl::CTS => {
                    v |= CR3_CTSE_MASK;
                }
                FlowControl::RTS_CTS => {
                    v |= CR3_RTSE_MASK | CR3_CTSE_MASK;
                }
            }

            // Special modes
            match config.mode {
                Mode::SingleWire => {
                    v |= CR3_HDSEL_MASK; // half duplex
                }
                Mode::IrDA => {
                    v |= CR3_IREN_MASK;
                }
                Mode::SmartCard => {
                    v |= CR3_SCEN_MASK;
                }
                _ => {}
            }

            v
        });

        // Baud rate
        let brr = self.compute_brr(config.baudrate);
        unsafe { ptr::write_volatile(&mut self.regs().brr, brr) };

        // Re-enable USART
        self.write_cr1(|v| utils::set_bit(v, CR1_UE_POS, true));
        Ok(())
    }

    fn send(&mut self, data: &[u8]) -> Result<()> {
        self.tx_count = 0;
        for &byte in data {
            self.wait_txe();
            unsafe { ptr::write_volatile(&mut self.regs().dr, byte as u32) };
            self.tx_count += 1;
        }
        self.wait_tc();
        if let Some(cb) = &mut self._callback {
            cb(Event::SEND_COMPLETE | Event::TX_COMPLETE);
        }
        Ok(())
    }

    fn receive(&mut self, data: &mut [u8]) -> Result<()> {
        self.rx_count = 0;
        for b in data.iter_mut() {
            self.wait_rxne();
            *b = unsafe { ptr::read_volatile(&self.regs().dr) as u8 };
            self.rx_count += 1;
        }
        if let Some(cb) = &mut self._callback {
            cb(Event::RECEIVE_COMPLETE);
        }
        Ok(())
    }

    fn transfer(&mut self, data_out: &[u8], data_in: &mut [u8]) -> Result<()> {
        if data_in.len() != data_out.len() {
            return Err(-1);
        }

        self.tx_count = 0;
        self.rx_count = 0;

        for (i, &byte) in data_out.iter().enumerate() {
            self.wait_txe();
            unsafe { ptr::write_volatile(&mut self.regs().dr, byte as u32) };
            self.tx_count += 1;

            self.wait_rxne();
            data_in[i] = unsafe { ptr::read_volatile(&self.regs().dr) as u8 };
            self.rx_count += 1;
        }
        self.wait_tc();
        if let Some(cb) = &mut self._callback {
            cb(Event::TRANSFER_COMPLETE);
        }
        Ok(())
    }

    fn get_tx_count(&self) -> u32 {
        self.tx_count
    }

    fn get_rx_count(&self) -> u32 {
        self.rx_count
    }

    fn get_status(&self) -> Status {
        let sr = unsafe { ptr::read_volatile(&self.regs().sr) };
        Status {
            tx_busy: (sr & SR_TC_MASK) == 0,
            rx_busy: (sr & SR_RXNE_MASK) != 0,
            tx_underflow: false,
            rx_overflow: (sr & SR_ORE_MASK) != 0,
            rx_break: (sr & SR_LBD_MASK) != 0,
            rx_framing_error: (sr & SR_FE_MASK) != 0,
            rx_parity_error: (sr & SR_PE_MASK) != 0,
        }
    }

    fn set_modem_control(&mut self, control: ModemControl) -> Result<()> {
        match control {
            // Direct manual control of RTS/DTR lines is not supported by this peripheral
            // (RTS/CTS are managed by hardware when enabled). Return unsupported.
            ModemControl::RTSClear
            | ModemControl::RTSSet
            | ModemControl::DTRClear
            | ModemControl::DTRSet => Err(-2),
        }
    }

    fn get_modem_status(&self) -> ModemStatus {
        let sr = unsafe { ptr::read_volatile(&self.regs().sr) };
        ModemStatus {
            cts: (sr & SR_CTS_MASK) != 0,
            dsr: false,
            dcd: false,
            ri: false,
        }
    }

    fn tx_enable(&mut self, enable: bool) -> Result<()> {
        self.write_cr1(|v| utils::set_bit(v, CR1_TE_POS, enable));
        Ok(())
    }

    fn rx_enable(&mut self, enable: bool) -> Result<()> {
        self.write_cr1(|v| utils::set_bit(v, CR1_RE_POS, enable));
        Ok(())
    }

    fn control_break(&mut self, enable: bool) -> Result<()> {
        // Setting SBK sends a break; it is cleared by hardware.
        if enable {
            self.write_cr1(|v| utils::set_bit(v, CR1_SBK_POS, true));
        }
        Ok(())
    }

    fn abort_send(&mut self) -> Result<()> {
        // Disable transmitter
        self.write_cr1(|v| utils::set_bit(v, CR1_TE_POS, false));
        Ok(())
    }

    fn abort_receive(&mut self) -> Result<()> {
        // Disable receiver
        self.write_cr1(|v| utils::set_bit(v, CR1_RE_POS, false));
        Ok(())
    }

    fn abort_transfer(&mut self) -> Result<()> {
        self.write_cr1(|mut v| {
            v = utils::set_bit(v, CR1_TE_POS, false);
            v = utils::set_bit(v, CR1_RE_POS, false);
            v
        });
        Ok(())
    }
}
