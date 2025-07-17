#![allow(dead_code)]

use bitflags::bitflags;

/// USB Role
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbRole {
    None = 0,
    Host = 1,
    Device = 2,
}

/// USB Signal Pins
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct UsbPin: u32 {
        const DP    = 1 << 0;  // USB D+
        const DM    = 1 << 1;  // USB D-
        const VBUS  = 1 << 2;  // USB VBUS
        const OC    = 1 << 3;  // OverCurrent
        const ID    = 1 << 4;  // USB ID
    }
}

/// USB Speeds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbSpeed {
    Low = 0,
    Full = 1,
    High = 2,
}

/// USB PID Types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum UsbPid {
    Reserved = 0,
    Out = 1,
    Ack = 2,
    Data0 = 3,
    Ping = 4,
    Sof = 5,
    Nyet = 6,
    Data2 = 7,
    Split = 8,
    In = 9,
    Nak = 10,
    Data1 = 11,
    Err = 12,
    Setup = 13,
    Stall = 14,
    Mdata = 15,
}

/// Endpoint address helpers
pub mod endpoint {
    pub const NUMBER_MASK: u8 = 0x0F;
    pub const DIRECTION_MASK: u8 = 0x80;

    #[inline]
    pub fn address(number: u8, direction_in: bool) -> u8 {
        (number & NUMBER_MASK) | if direction_in { DIRECTION_MASK } else { 0 }
    }

    #[inline]
    pub fn number(address: u8) -> u8 {
        address & NUMBER_MASK
    }

    #[inline]
    pub fn is_in(address: u8) -> bool {
        address & DIRECTION_MASK != 0
    }
}

/// Endpoint types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndpointType {
    Control = 0,
    Isochronous = 1,
    Bulk = 2,
    Interrupt = 3,
}

/// Endpoint packet size helpers
pub mod packet_size {
    pub const MAX_PACKET_SIZE_MASK: u16 = 0x07FF;

    pub const MICROFRAME_TRANSACTIONS_MASK: u16 = 0x1800;
    pub const MICROFRAME_TRANSACTIONS_1: u16 = 0x0000;
    pub const MICROFRAME_TRANSACTIONS_2: u16 = 0x0800;
    pub const MICROFRAME_TRANSACTIONS_3: u16 = 0x1000;
}

pub mod device;
pub mod host;
