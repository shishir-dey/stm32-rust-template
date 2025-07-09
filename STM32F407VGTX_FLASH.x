/* Define memory regions for STM32F407VG */
MEMORY
{
    FLASH (rx)    : ORIGIN = 0x08000000, LENGTH = 1M
    RAM (rwx)     : ORIGIN = 0x20000000, LENGTH = 112K
    CCMRAM (rwx)  : ORIGIN = 0x10000000, LENGTH = 64K
    /* Optional: Backup SRAM (if needed)
    BSRAM (rwx)   : ORIGIN = 0x40024000, LENGTH = 4K */
}

_start_of_stack = ORIGIN(RAM) + LENGTH(RAM);