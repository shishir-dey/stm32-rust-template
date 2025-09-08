// SCB (System Control Block) register definitions
// Based on CMSIS Cortex-M4 core_cm4.h

use super::super::super::mcu::stm32f407::PeripheralAccess;

// SCB Base Address
pub const SCB_BASE: u32 = 0xE000ED00;

// SCB Register Block
#[repr(C)]
pub struct Scb {
    pub cpuid: u32,     // CPUID Base Register
    pub icsr: u32,      // Interrupt Control and State Register
    pub vtor: u32,      // Vector Table Offset Register
    pub aircr: u32,     // Application Interrupt and Reset Control Register
    pub scr: u32,       // System Control Register
    pub ccr: u32,       // Configuration Control Register
    pub shp: [u8; 12],  // System Handlers Priority Registers (4-7, 8-11, 12-15)
    pub shcsr: u32,     // System Handler Control and State Register
    pub cfsr: u32,      // Configurable Fault Status Register
    pub hfsr: u32,      // HardFault Status Register
    pub dfsr: u32,      // Debug Fault Status Register
    pub mmfar: u32,     // MemManage Fault Address Register
    pub bfar: u32,      // BusFault Address Register
    pub afsr: u32,      // Auxiliary Fault Status Register
    pub pfr: [u32; 2],  // Processor Feature Register
    pub dfr: u32,       // Debug Feature Register
    pub adr: u32,       // Auxiliary Feature Register
    pub mmfr: [u32; 4], // Memory Model Feature Register
    pub isar: [u32; 5], // Instruction Set Attributes Register
    _reserved0: [u32; 5],
    pub cpacr: u32, // Coprocessor Access Control Register
}

// SCB Peripheral Instance
pub struct SCB;

impl PeripheralAccess for SCB {
    const BASE_ADDRESS: u32 = SCB_BASE;
    type RegisterBlock = Scb;
}

// SCB CPUID Register Definitions
pub const SCB_CPUID_IMPLEMENTER_POS: u32 = 24;
pub const SCB_CPUID_IMPLEMENTER_MSK: u32 = 0xFF << SCB_CPUID_IMPLEMENTER_POS;

pub const SCB_CPUID_VARIANT_POS: u32 = 20;
pub const SCB_CPUID_VARIANT_MSK: u32 = 0xF << SCB_CPUID_VARIANT_POS;

pub const SCB_CPUID_ARCHITECTURE_POS: u32 = 16;
pub const SCB_CPUID_ARCHITECTURE_MSK: u32 = 0xF << SCB_CPUID_ARCHITECTURE_POS;

pub const SCB_CPUID_PARTNO_POS: u32 = 4;
pub const SCB_CPUID_PARTNO_MSK: u32 = 0xFFF << SCB_CPUID_PARTNO_POS;

pub const SCB_CPUID_REVISION_POS: u32 = 0;
pub const SCB_CPUID_REVISION_MSK: u32 = 0xF;

// SCB Interrupt Control State Register Definitions
pub const SCB_ICSR_NMIPENDSET_POS: u32 = 31;
pub const SCB_ICSR_NMIPENDSET_MSK: u32 = 1 << SCB_ICSR_NMIPENDSET_POS;

pub const SCB_ICSR_PENDSVSET_POS: u32 = 28;
pub const SCB_ICSR_PENDSVSET_MSK: u32 = 1 << SCB_ICSR_PENDSVSET_POS;

pub const SCB_ICSR_PENDSVCLR_POS: u32 = 27;
pub const SCB_ICSR_PENDSVCLR_MSK: u32 = 1 << SCB_ICSR_PENDSVCLR_POS;

pub const SCB_ICSR_PENDSTSET_POS: u32 = 26;
pub const SCB_ICSR_PENDSTSET_MSK: u32 = 1 << SCB_ICSR_PENDSTSET_POS;

pub const SCB_ICSR_PENDSTCLR_POS: u32 = 25;
pub const SCB_ICSR_PENDSTCLR_MSK: u32 = 1 << SCB_ICSR_PENDSTCLR_POS;

pub const SCB_ICSR_ISRPREEMPT_POS: u32 = 23;
pub const SCB_ICSR_ISRPREEMPT_MSK: u32 = 1 << SCB_ICSR_ISRPREEMPT_POS;

pub const SCB_ICSR_ISRPENDING_POS: u32 = 22;
pub const SCB_ICSR_ISRPENDING_MSK: u32 = 1 << SCB_ICSR_ISRPENDING_POS;

pub const SCB_ICSR_VECTPENDING_POS: u32 = 12;
pub const SCB_ICSR_VECTPENDING_MSK: u32 = 0x1FF << SCB_ICSR_VECTPENDING_POS;

pub const SCB_ICSR_RETTOBASE_POS: u32 = 11;
pub const SCB_ICSR_RETTOBASE_MSK: u32 = 1 << SCB_ICSR_RETTOBASE_POS;

pub const SCB_ICSR_VECTACTIVE_POS: u32 = 0;
pub const SCB_ICSR_VECTACTIVE_MSK: u32 = 0x1FF;

// SCB Vector Table Offset Register Definitions
pub const SCB_VTOR_TBLOFF_POS: u32 = 7;
pub const SCB_VTOR_TBLOFF_MSK: u32 = 0x1FFFFFF << SCB_VTOR_TBLOFF_POS;

// SCB Application Interrupt and Reset Control Register Definitions
pub const SCB_AIRCR_VECTKEY_POS: u32 = 16;
pub const SCB_AIRCR_VECTKEY_MSK: u32 = 0xFFFF << SCB_AIRCR_VECTKEY_POS;
pub const SCB_AIRCR_VECTKEYSTAT_POS: u32 = 16;
pub const SCB_AIRCR_VECTKEYSTAT_MSK: u32 = 0xFFFF << SCB_AIRCR_VECTKEYSTAT_POS;

pub const SCB_AIRCR_ENDIANESS_POS: u32 = 15;
pub const SCB_AIRCR_ENDIANESS_MSK: u32 = 1 << SCB_AIRCR_ENDIANESS_POS;

pub const SCB_AIRCR_PRIGROUP_POS: u32 = 8;
pub const SCB_AIRCR_PRIGROUP_MSK: u32 = 7 << SCB_AIRCR_PRIGROUP_POS;

pub const SCB_AIRCR_SYSRESETREQ_POS: u32 = 2;
pub const SCB_AIRCR_SYSRESETREQ_MSK: u32 = 1 << SCB_AIRCR_SYSRESETREQ_POS;

pub const SCB_AIRCR_VECTCLRACTIVE_POS: u32 = 1;
pub const SCB_AIRCR_VECTCLRACTIVE_MSK: u32 = 1 << SCB_AIRCR_VECTCLRACTIVE_POS;

pub const SCB_AIRCR_VECTRESET_POS: u32 = 0;
pub const SCB_AIRCR_VECTRESET_MSK: u32 = 1;

// SCB System Control Register Definitions
pub const SCB_SCR_SEVONPEND_POS: u32 = 4;
pub const SCB_SCR_SEVONPEND_MSK: u32 = 1 << SCB_SCR_SEVONPEND_POS;

pub const SCB_SCR_SLEEPDEEP_POS: u32 = 2;
pub const SCB_SCR_SLEEPDEEP_MSK: u32 = 1 << SCB_SCR_SLEEPDEEP_POS;

pub const SCB_SCR_SLEEPONEXIT_POS: u32 = 1;
pub const SCB_SCR_SLEEPONEXIT_MSK: u32 = 1 << SCB_SCR_SLEEPONEXIT_POS;

// SCB Configuration Control Register Definitions
pub const SCB_CCR_STKALIGN_POS: u32 = 9;
pub const SCB_CCR_STKALIGN_MSK: u32 = 1 << SCB_CCR_STKALIGN_POS;

pub const SCB_CCR_BFHFNMIGN_POS: u32 = 8;
pub const SCB_CCR_BFHFNMIGN_MSK: u32 = 1 << SCB_CCR_BFHFNMIGN_POS;

pub const SCB_CCR_DIV_0_TRP_POS: u32 = 4;
pub const SCB_CCR_DIV_0_TRP_MSK: u32 = 1 << SCB_CCR_DIV_0_TRP_POS;

pub const SCB_CCR_UNALIGN_TRP_POS: u32 = 3;
pub const SCB_CCR_UNALIGN_TRP_MSK: u32 = 1 << SCB_CCR_UNALIGN_TRP_POS;

pub const SCB_CCR_USERSETMPEND_POS: u32 = 1;
pub const SCB_CCR_USERSETMPEND_MSK: u32 = 1 << SCB_CCR_USERSETMPEND_POS;

pub const SCB_CCR_NONBASETHRDENA_POS: u32 = 0;
pub const SCB_CCR_NONBASETHRDENA_MSK: u32 = 1;

// SCB System Handler Control and State Register Definitions
pub const SCB_SHCSR_USGFAULTENA_POS: u32 = 18;
pub const SCB_SHCSR_USGFAULTENA_MSK: u32 = 1 << SCB_SHCSR_USGFAULTENA_POS;

pub const SCB_SHCSR_BUSFAULTENA_POS: u32 = 17;
pub const SCB_SHCSR_BUSFAULTENA_MSK: u32 = 1 << SCB_SHCSR_BUSFAULTENA_POS;

pub const SCB_SHCSR_MEMFAULTENA_POS: u32 = 16;
pub const SCB_SHCSR_MEMFAULTENA_MSK: u32 = 1 << SCB_SHCSR_MEMFAULTENA_POS;

pub const SCB_SHCSR_SVCALLPENDED_POS: u32 = 15;
pub const SCB_SHCSR_SVCALLPENDED_MSK: u32 = 1 << SCB_SHCSR_SVCALLPENDED_POS;

pub const SCB_SHCSR_BUSFAULTPENDED_POS: u32 = 14;
pub const SCB_SHCSR_BUSFAULTPENDED_MSK: u32 = 1 << SCB_SHCSR_BUSFAULTPENDED_POS;

pub const SCB_SHCSR_MEMFAULTPENDED_POS: u32 = 13;
pub const SCB_SHCSR_MEMFAULTPENDED_MSK: u32 = 1 << SCB_SHCSR_MEMFAULTPENDED_POS;

pub const SCB_SHCSR_USGFAULTPENDED_POS: u32 = 12;
pub const SCB_SHCSR_USGFAULTPENDED_MSK: u32 = 1 << SCB_SHCSR_USGFAULTPENDED_POS;

pub const SCB_SHCSR_SYSTICKACT_POS: u32 = 11;
pub const SCB_SHCSR_SYSTICKACT_MSK: u32 = 1 << SCB_SHCSR_SYSTICKACT_POS;

pub const SCB_SHCSR_PENDSVACT_POS: u32 = 10;
pub const SCB_SHCSR_PENDSVACT_MSK: u32 = 1 << SCB_SHCSR_PENDSVACT_POS;

pub const SCB_SHCSR_MONITORACT_POS: u32 = 8;
pub const SCB_SHCSR_MONITORACT_MSK: u32 = 1 << SCB_SHCSR_MONITORACT_POS;

pub const SCB_SHCSR_SVCALLACT_POS: u32 = 7;
pub const SCB_SHCSR_SVCALLACT_MSK: u32 = 1 << SCB_SHCSR_SVCALLACT_POS;

pub const SCB_SHCSR_USGFAULTACT_POS: u32 = 3;
pub const SCB_SHCSR_USGFAULTACT_MSK: u32 = 1 << SCB_SHCSR_USGFAULTACT_POS;

pub const SCB_SHCSR_BUSFAULTACT_POS: u32 = 1;
pub const SCB_SHCSR_BUSFAULTACT_MSK: u32 = 1 << SCB_SHCSR_BUSFAULTACT_POS;

pub const SCB_SHCSR_MEMFAULTACT_POS: u32 = 0;
pub const SCB_SHCSR_MEMFAULTACT_MSK: u32 = 1;

// SCB Configurable Fault Status Register Definitions
pub const SCB_CFSR_USGFAULTSR_POS: u32 = 16;
pub const SCB_CFSR_USGFAULTSR_MSK: u32 = 0xFFFF << SCB_CFSR_USGFAULTSR_POS;

pub const SCB_CFSR_BUSFAULTSR_POS: u32 = 8;
pub const SCB_CFSR_BUSFAULTSR_MSK: u32 = 0xFF << SCB_CFSR_BUSFAULTSR_POS;

pub const SCB_CFSR_MEMFAULTSR_POS: u32 = 0;
pub const SCB_CFSR_MEMFAULTSR_MSK: u32 = 0xFF;

// MemManage Fault Status Register (part of SCB Configurable Fault Status Register)
pub const SCB_CFSR_MMARVALID_POS: u32 = SCB_CFSR_MEMFAULTSR_POS + 7;
pub const SCB_CFSR_MMARVALID_MSK: u32 = 1 << SCB_CFSR_MMARVALID_POS;

pub const SCB_CFSR_MLSPERR_POS: u32 = SCB_CFSR_MEMFAULTSR_POS + 5;
pub const SCB_CFSR_MLSPERR_MSK: u32 = 1 << SCB_CFSR_MLSPERR_POS;

pub const SCB_CFSR_MSTKERR_POS: u32 = SCB_CFSR_MEMFAULTSR_POS + 4;
pub const SCB_CFSR_MSTKERR_MSK: u32 = 1 << SCB_CFSR_MSTKERR_POS;

pub const SCB_CFSR_MUNSTKERR_POS: u32 = SCB_CFSR_MEMFAULTSR_POS + 3;
pub const SCB_CFSR_MUNSTKERR_MSK: u32 = 1 << SCB_CFSR_MUNSTKERR_POS;

pub const SCB_CFSR_DACCVIOL_POS: u32 = SCB_CFSR_MEMFAULTSR_POS + 1;
pub const SCB_CFSR_DACCVIOL_MSK: u32 = 1 << SCB_CFSR_DACCVIOL_POS;

pub const SCB_CFSR_IACCVIOL_POS: u32 = SCB_CFSR_MEMFAULTSR_POS + 0;
pub const SCB_CFSR_IACCVIOL_MSK: u32 = 1;

// BusFault Status Register (part of SCB Configurable Fault Status Register)
pub const SCB_CFSR_BFARVALID_POS: u32 = SCB_CFSR_BUSFAULTSR_POS + 7;
pub const SCB_CFSR_BFARVALID_MSK: u32 = 1 << SCB_CFSR_BFARVALID_POS;

pub const SCB_CFSR_LSPERR_POS: u32 = SCB_CFSR_BUSFAULTSR_POS + 5;
pub const SCB_CFSR_LSPERR_MSK: u32 = 1 << SCB_CFSR_LSPERR_POS;

pub const SCB_CFSR_STKERR_POS: u32 = SCB_CFSR_BUSFAULTSR_POS + 4;
pub const SCB_CFSR_STKERR_MSK: u32 = 1 << SCB_CFSR_STKERR_POS;

pub const SCB_CFSR_UNSTKERR_POS: u32 = SCB_CFSR_BUSFAULTSR_POS + 3;
pub const SCB_CFSR_UNSTKERR_MSK: u32 = 1 << SCB_CFSR_UNSTKERR_POS;

pub const SCB_CFSR_IMPRECISERR_POS: u32 = SCB_CFSR_BUSFAULTSR_POS + 2;
pub const SCB_CFSR_IMPRECISERR_MSK: u32 = 1 << SCB_CFSR_IMPRECISERR_POS;

pub const SCB_CFSR_PRECISERR_POS: u32 = SCB_CFSR_BUSFAULTSR_POS + 1;
pub const SCB_CFSR_PRECISERR_MSK: u32 = 1 << SCB_CFSR_PRECISERR_POS;

pub const SCB_CFSR_IBUSERR_POS: u32 = SCB_CFSR_BUSFAULTSR_POS + 0;
pub const SCB_CFSR_IBUSERR_MSK: u32 = 1 << SCB_CFSR_IBUSERR_POS;

// UsageFault Status Register (part of SCB Configurable Fault Status Register)
pub const SCB_CFSR_DIVBYZERO_POS: u32 = SCB_CFSR_USGFAULTSR_POS + 9;
pub const SCB_CFSR_DIVBYZERO_MSK: u32 = 1 << SCB_CFSR_DIVBYZERO_POS;

pub const SCB_CFSR_UNALIGNED_POS: u32 = SCB_CFSR_USGFAULTSR_POS + 8;
pub const SCB_CFSR_UNALIGNED_MSK: u32 = 1 << SCB_CFSR_UNALIGNED_POS;

pub const SCB_CFSR_NOCP_POS: u32 = SCB_CFSR_USGFAULTSR_POS + 3;
pub const SCB_CFSR_NOCP_MSK: u32 = 1 << SCB_CFSR_NOCP_POS;

pub const SCB_CFSR_INVPC_POS: u32 = SCB_CFSR_USGFAULTSR_POS + 2;
pub const SCB_CFSR_INVPC_MSK: u32 = 1 << SCB_CFSR_INVPC_POS;

pub const SCB_CFSR_INVSTATE_POS: u32 = SCB_CFSR_USGFAULTSR_POS + 1;
pub const SCB_CFSR_INVSTATE_MSK: u32 = 1 << SCB_CFSR_INVSTATE_POS;

pub const SCB_CFSR_UNDEFINSTR_POS: u32 = SCB_CFSR_USGFAULTSR_POS + 0;
pub const SCB_CFSR_UNDEFINSTR_MSK: u32 = 1 << SCB_CFSR_UNDEFINSTR_POS;

// SCB Hard Fault Status Register Definitions
pub const SCB_HFSR_DEBUGEVT_POS: u32 = 31;
pub const SCB_HFSR_DEBUGEVT_MSK: u32 = 1 << SCB_HFSR_DEBUGEVT_POS;

pub const SCB_HFSR_FORCED_POS: u32 = 30;
pub const SCB_HFSR_FORCED_MSK: u32 = 1 << SCB_HFSR_FORCED_POS;

pub const SCB_HFSR_VECTTBL_POS: u32 = 1;
pub const SCB_HFSR_VECTTBL_MSK: u32 = 1 << SCB_HFSR_VECTTBL_POS;

// SCB Debug Fault Status Register Definitions
pub const SCB_DFSR_EXTERNAL_POS: u32 = 4;
pub const SCB_DFSR_EXTERNAL_MSK: u32 = 1 << SCB_DFSR_EXTERNAL_POS;

pub const SCB_DFSR_VCATCH_POS: u32 = 3;
pub const SCB_DFSR_VCATCH_MSK: u32 = 1 << SCB_DFSR_VCATCH_POS;

pub const SCB_DFSR_DWTTRAP_POS: u32 = 2;
pub const SCB_DFSR_DWTTRAP_MSK: u32 = 1 << SCB_DFSR_DWTTRAP_POS;

pub const SCB_DFSR_BKPT_POS: u32 = 1;
pub const SCB_DFSR_BKPT_MSK: u32 = 1 << SCB_DFSR_BKPT_POS;

pub const SCB_DFSR_HALTED_POS: u32 = 0;
pub const SCB_DFSR_HALTED_MSK: u32 = 1;

// SCB Coprocessor Access Control Register Definitions
pub const SCB_CPACR_CP11_POS: u32 = 22;
pub const SCB_CPACR_CP11_MSK: u32 = 3 << SCB_CPACR_CP11_POS;

pub const SCB_CPACR_CP10_POS: u32 = 20;
pub const SCB_CPACR_CP10_MSK: u32 = 3 << SCB_CPACR_CP10_POS;

pub const SCB_CPACR_CP7_POS: u32 = 14;
pub const SCB_CPACR_CP7_MSK: u32 = 3 << SCB_CPACR_CP7_POS;

pub const SCB_CPACR_CP6_POS: u32 = 12;
pub const SCB_CPACR_CP6_MSK: u32 = 3 << SCB_CPACR_CP6_POS;

pub const SCB_CPACR_CP5_POS: u32 = 10;
pub const SCB_CPACR_CP5_MSK: u32 = 3 << SCB_CPACR_CP5_POS;

pub const SCB_CPACR_CP4_POS: u32 = 8;
pub const SCB_CPACR_CP4_MSK: u32 = 3 << SCB_CPACR_CP4_POS;

pub const SCB_CPACR_CP3_POS: u32 = 6;
pub const SCB_CPACR_CP3_MSK: u32 = 3 << SCB_CPACR_CP3_POS;

pub const SCB_CPACR_CP2_POS: u32 = 4;
pub const SCB_CPACR_CP2_MSK: u32 = 3 << SCB_CPACR_CP2_POS;

pub const SCB_CPACR_CP1_POS: u32 = 2;
pub const SCB_CPACR_CP1_MSK: u32 = 3 << SCB_CPACR_CP1_POS;

pub const SCB_CPACR_CP0_POS: u32 = 0;
pub const SCB_CPACR_CP0_MSK: u32 = 3 << SCB_CPACR_CP0_POS;
