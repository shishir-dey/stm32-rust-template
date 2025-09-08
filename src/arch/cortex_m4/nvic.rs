// NVIC (Nested Vectored Interrupt Controller) register definitions
// Based on CMSIS Cortex-M4 core_cm4.h

use super::super::super::mcu::stm32f407::PeripheralAccess;

// NVIC Base Address
pub const NVIC_BASE: u32 = 0xE000E100;

// NVIC Register Block
#[repr(C)]
pub struct Nvic {
    pub iser: [u32; 8], // Interrupt Set Enable Register
    _reserved0: [u32; 24],
    pub icer: [u32; 8], // Interrupt Clear Enable Register
    _reserved1: [u32; 24],
    pub ispr: [u32; 8], // Interrupt Set Pending Register
    _reserved2: [u32; 24],
    pub icpr: [u32; 8], // Interrupt Clear Pending Register
    _reserved3: [u32; 24],
    pub iabr: [u32; 8], // Interrupt Active bit Register
    _reserved4: [u32; 56],
    pub ip: [u8; 240], // Interrupt Priority Register (8Bit wide)
    _reserved5: [u32; 644],
    pub stir: u32, // Software Trigger Interrupt Register
}

// NVIC Peripheral Instance
pub struct NVIC;

impl PeripheralAccess for NVIC {
    const BASE_ADDRESS: u32 = NVIC_BASE;
    type RegisterBlock = Nvic;
}

// Software Triggered Interrupt Register Definitions
pub const NVIC_STIR_INTID_POS: u32 = 0;
pub const NVIC_STIR_INTID_MSK: u32 = 0x1FF;

// Helper functions for NVIC
impl Nvic {
    /// Enable Interrupt
    pub fn enable_irq(&mut self, irqn: u32) {
        if irqn < 240 {
            let index = (irqn >> 5) as usize;
            let bit = irqn & 0x1F;
            self.iser[index] |= 1 << bit;
        }
    }

    /// Disable Interrupt
    pub fn disable_irq(&mut self, irqn: u32) {
        if irqn < 240 {
            let index = (irqn >> 5) as usize;
            let bit = irqn & 0x1F;
            self.icer[index] |= 1 << bit;
        }
    }

    /// Get Interrupt Enable status
    pub fn get_enable_irq(&self, irqn: u32) -> bool {
        if irqn < 240 {
            let index = (irqn >> 5) as usize;
            let bit = irqn & 0x1F;
            (self.iser[index] & (1 << bit)) != 0
        } else {
            false
        }
    }

    /// Set Pending Interrupt
    pub fn set_pending_irq(&mut self, irqn: u32) {
        if irqn < 240 {
            let index = (irqn >> 5) as usize;
            let bit = irqn & 0x1F;
            self.ispr[index] |= 1 << bit;
        }
    }

    /// Clear Pending Interrupt
    pub fn clear_pending_irq(&mut self, irqn: u32) {
        if irqn < 240 {
            let index = (irqn >> 5) as usize;
            let bit = irqn & 0x1F;
            self.icpr[index] |= 1 << bit;
        }
    }

    /// Get Pending Interrupt
    pub fn get_pending_irq(&self, irqn: u32) -> bool {
        if irqn < 240 {
            let index = (irqn >> 5) as usize;
            let bit = irqn & 0x1F;
            (self.ispr[index] & (1 << bit)) != 0
        } else {
            false
        }
    }

    /// Get Active Interrupt
    pub fn get_active(&self, irqn: u32) -> bool {
        if irqn < 240 {
            let index = (irqn >> 5) as usize;
            let bit = irqn & 0x1F;
            (self.iabr[index] & (1 << bit)) != 0
        } else {
            false
        }
    }

    /// Set Interrupt Priority
    pub fn set_priority(&mut self, irqn: u32, priority: u8) {
        if irqn < 240 {
            self.ip[irqn as usize] = priority;
        }
    }

    /// Get Interrupt Priority
    pub fn get_priority(&self, irqn: u32) -> u8 {
        if irqn < 240 {
            self.ip[irqn as usize]
        } else {
            0
        }
    }

    /// Trigger Software Interrupt
    pub fn trigger_interrupt(&mut self, irqn: u32) {
        if irqn < 240 {
            self.stir = irqn & 0x1FF;
        }
    }
}

// Global functions similar to CMSIS
pub fn nvic_enable_irq(irqn: u32) {
    unsafe {
        let nvic = &mut *(NVIC_BASE as *mut Nvic);
        nvic.enable_irq(irqn);
    }
}

pub fn nvic_disable_irq(irqn: u32) {
    unsafe {
        let nvic = &mut *(NVIC_BASE as *mut Nvic);
        nvic.disable_irq(irqn);
    }
}

pub fn nvic_set_pending_irq(irqn: u32) {
    unsafe {
        let nvic = &mut *(NVIC_BASE as *mut Nvic);
        nvic.set_pending_irq(irqn);
    }
}

pub fn nvic_clear_pending_irq(irqn: u32) {
    unsafe {
        let nvic = &mut *(NVIC_BASE as *mut Nvic);
        nvic.clear_pending_irq(irqn);
    }
}

pub fn nvic_get_pending_irq(irqn: u32) -> bool {
    unsafe {
        let nvic = &*(NVIC_BASE as *const Nvic);
        nvic.get_pending_irq(irqn)
    }
}

pub fn nvic_get_active(irqn: u32) -> bool {
    unsafe {
        let nvic = &*(NVIC_BASE as *const Nvic);
        nvic.get_active(irqn)
    }
}

pub fn nvic_set_priority(irqn: u32, priority: u32) {
    unsafe {
        let nvic = &mut *(NVIC_BASE as *mut Nvic);
        nvic.set_priority(irqn, (priority as u8) << 4); // Assuming 4 bits for priority
    }
}

pub fn nvic_get_priority(irqn: u32) -> u32 {
    unsafe {
        let nvic = &*(NVIC_BASE as *const Nvic);
        (nvic.get_priority(irqn) >> 4) as u32
    }
}
