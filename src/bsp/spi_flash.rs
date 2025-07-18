//! SPI Flash HAL Driver

use crate::driver::spi::{Result, Spi};
use core::marker::PhantomData;

pub const SPIF_PAGE_SIZE: usize = 0x100;
pub const SPIF_SECTOR_SIZE: usize = 0x1000;
pub const SPIF_BLOCK_SIZE: usize = 0x10000;

/// Manufacturer and size codes, matching C header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Manufacturer {
    Error = 0,
    Winbond = 0xEF,
    ISSI = 0x9D,
    Micron = 0x20,
    GigaDevice = 0xC8,
    Macronix = 0xC2,
    Spansion = 0x01,
    Amic = 0x37,
    Sst = 0xBF,
    Hyundai = 0xAD,
    Atmel = 0x1F,
    Fudan = 0xA1,
    Esmt = 0x8C,
    Intel = 0x89,
    Sanyo = 0x62,
    Fujitsu = 0x04,
    Eon = 0x1C,
    Puya = 0x85,
    Unknown = 0xFF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlashSize {
    Error = 0,
    M1bit = 0x11,
    M2bit = 0x12,
    M4bit = 0x13,
    M8bit = 0x14,
    M16bit = 0x15,
    M32bit = 0x16,
    M64bit = 0x17,
    M128bit = 0x18,
    M256bit = 0x19,
    M512bit = 0x20,
    Unknown = 0xFF,
}

/// Flash device configuration/handle.
pub struct SpiFlash<'a, SPI: Spi<'a>> {
    spi: SPI,
    cs: fn(active: bool), // software control of CS
    pub manufacturer: Manufacturer,
    pub size: FlashSize,
    pub page_count: u32,
    pub sector_count: u32,
    pub block_count: u32,
    four_byte_addr: bool,
    busy: bool,
    _phantom: PhantomData<&'a ()>,
}

impl<'a, SPI: Spi<'a>> SpiFlash<'a, SPI> {
    pub fn new(spi: SPI, cs: fn(active: bool)) -> Self {
        Self {
            spi,
            cs,
            manufacturer: Manufacturer::Unknown,
            size: FlashSize::Unknown,
            page_count: 0,
            sector_count: 0,
            block_count: 0,
            four_byte_addr: false,
            busy: false,
            _phantom: PhantomData,
        }
    }

    /// Pull CS low
    fn select(&self) {
        (self.cs)(false);
    }
    /// Pull CS high
    fn deselect(&self) {
        (self.cs)(true);
    }

    /// Send command and optional address/data; optionally read.
    fn cmd(
        &mut self,
        cmd: u8,
        addr: Option<u32>,
        tx_data: Option<&[u8]>,
        rx_data: Option<&mut [u8]>,
    ) -> Result<()> {
        let mut buf = [0u8; 5];
        let mut n = 1;
        buf[0] = cmd;
        if let Some(a) = addr {
            if self.four_byte_addr {
                buf[1] = ((a >> 24) & 0xFF) as u8;
                buf[2] = ((a >> 16) & 0xFF) as u8;
                buf[3] = ((a >> 8) & 0xFF) as u8;
                buf[4] = (a & 0xFF) as u8;
                n += 4;
            } else {
                buf[1] = ((a >> 16) & 0xFF) as u8;
                buf[2] = ((a >> 8) & 0xFF) as u8;
                buf[3] = (a & 0xFF) as u8;
                n += 3;
            }
        }
        self.select();
        self.spi.send(&buf[0..n])?;
        if let Some(tx) = tx_data {
            self.spi.send(tx)?;
        }
        if let Some(rx) = rx_data {
            self.spi.receive(rx)?;
        }
        self.deselect();
        Ok(())
    }

    pub fn find_chip(&mut self) -> Result<()> {
        let mut rx = [0xFFu8; 4];
        self.select();
        self.spi.send(&[0x9F])?;
        self.spi.receive(&mut rx[1..4])?;
        self.deselect();

        self.manufacturer = match rx[1] {
            0xEF => Manufacturer::Winbond,
            0x9D => Manufacturer::ISSI,
            0x20 => Manufacturer::Micron,
            0xC8 => Manufacturer::GigaDevice,
            0xC2 => Manufacturer::Macronix,
            0x01 => Manufacturer::Spansion,
            0x37 => Manufacturer::Amic,
            0xBF => Manufacturer::Sst,
            0xAD => Manufacturer::Hyundai,
            0x1F => Manufacturer::Atmel,
            0xA1 => Manufacturer::Fudan,
            0x8C => Manufacturer::Esmt,
            0x89 => Manufacturer::Intel,
            0x62 => Manufacturer::Sanyo,
            0x04 => Manufacturer::Fujitsu,
            0x1C => Manufacturer::Eon,
            0x85 => Manufacturer::Puya,
            _ => Manufacturer::Error,
        };
        self.size = match rx[3] {
            0x11 => FlashSize::M1bit,
            0x12 => FlashSize::M2bit,
            0x13 => FlashSize::M4bit,
            0x14 => FlashSize::M8bit,
            0x15 => FlashSize::M16bit,
            0x16 => FlashSize::M32bit,
            0x17 => FlashSize::M64bit,
            0x18 => FlashSize::M128bit,
            0x19 => FlashSize::M256bit,
            0x20 => FlashSize::M512bit,
            _ => FlashSize::Error,
        };
        self.block_count = match self.size {
            FlashSize::M1bit => 2,
            FlashSize::M2bit => 4,
            FlashSize::M4bit => 8,
            FlashSize::M8bit => 16,
            FlashSize::M16bit => 32,
            FlashSize::M32bit => 64,
            FlashSize::M64bit => 128,
            FlashSize::M128bit => 256,
            FlashSize::M256bit => 512,
            FlashSize::M512bit => 1024,
            _ => 0,
        };
        self.sector_count = self.block_count * 16;
        self.page_count = self.sector_count * (SPIF_SECTOR_SIZE as u32 / SPIF_PAGE_SIZE as u32);
        self.four_byte_addr = self.block_count >= 512;
        Ok(())
    }

    fn write_enable(&mut self) -> Result<()> {
        self.cmd(0x06, None, None, None)
    }

    fn write_disable(&mut self) -> Result<()> {
        self.cmd(0x04, None, None, None)
    }

    fn read_status(&mut self) -> Result<u8> {
        let mut tx = [0x05, 0xA5];
        let mut rx = [0u8; 2];
        self.select();
        self.spi.transfer(&tx, &mut rx)?;
        self.deselect();
        Ok(rx[1])
    }

    pub fn wait_ready(&mut self, timeout: u32, mut delay: impl FnMut(u32)) -> Result<()> {
        let mut t = 0;
        while t < timeout {
            if self.read_status()? & 0x01 == 0 {
                return Ok(());
            }
            delay(1);
            t += 1;
        }
        Err(-1)
    }

    pub fn erase_chip(&mut self, mut delay: impl FnMut(u32)) -> Result<()> {
        self.busy = true;
        self.write_enable()?;
        self.cmd(0x60, None, None, None)?; // erase command
        self.wait_ready(self.block_count * 1000, &mut delay)?;
        self.write_disable()?;
        self.busy = false;
        Ok(())
    }

    pub fn erase_sector(&mut self, sector: u32, mut delay: impl FnMut(u32)) -> Result<()> {
        if sector >= self.sector_count {
            return Err(-2);
        }
        self.busy = true;
        let address = sector * SPIF_SECTOR_SIZE as u32;
        self.write_enable()?;
        self.cmd(
            if self.four_byte_addr { 0x21 } else { 0x20 },
            Some(address),
            None,
            None,
        )?;
        self.wait_ready(1000, &mut delay)?;
        self.write_disable()?;
        self.busy = false;
        Ok(())
    }

    pub fn erase_block(&mut self, block: u32, mut delay: impl FnMut(u32)) -> Result<()> {
        if block >= self.block_count {
            return Err(-2);
        }
        self.busy = true;
        let address = block * SPIF_BLOCK_SIZE as u32;
        self.write_enable()?;
        self.cmd(
            if self.four_byte_addr { 0xDC } else { 0xD8 },
            Some(address),
            None,
            None,
        )?;
        self.wait_ready(3000, &mut delay)?;
        self.write_disable()?;
        self.busy = false;
        Ok(())
    }

    /// Write array to address, splits across pages as needed.
    pub fn write_address(
        &mut self,
        mut address: u32,
        mut data: &[u8],
        mut delay: impl FnMut(u32),
    ) -> Result<()> {
        self.busy = true;
        while !data.is_empty() {
            let page = (address / SPIF_PAGE_SIZE as u32) as u32;
            let offset = (address % SPIF_PAGE_SIZE as u32) as usize;
            let max = SPIF_PAGE_SIZE - offset;
            let to_write = core::cmp::min(data.len(), max);
            self.write_page(page, &data[..to_write], offset, &mut delay)?;
            address += to_write as u32;
            data = &data[to_write..];
        }
        self.busy = false;
        Ok(())
    }

    /// Write a partial page (up to 256-offset).
    pub fn write_page(
        &mut self,
        page: u32,
        data: &[u8],
        offset: usize,
        delay: &mut impl FnMut(u32),
    ) -> Result<()> {
        if offset >= SPIF_PAGE_SIZE {
            return Err(-3);
        }
        let n = core::cmp::min(data.len(), SPIF_PAGE_SIZE - offset);
        let address = page * SPIF_PAGE_SIZE as u32 + offset as u32;
        self.write_enable()?;
        self.cmd(
            if self.four_byte_addr { 0x12 } else { 0x02 },
            Some(address),
            Some(&data[..n]),
            None,
        )?;
        self.wait_ready(100, delay)?;
        self.write_disable()?;
        Ok(())
    }

    /// Read array from arbitrary address, splits if needed.
    pub fn read_address(&mut self, mut address: u32, mut data: &mut [u8]) -> Result<()> {
        self.busy = true;
        let max_len = 256; // can tune based on stack
        let mut off = 0;
        while off < data.len() {
            let sz = core::cmp::min(max_len, data.len() - off);
            self.cmd(
                if self.four_byte_addr { 0x13 } else { 0x03 },
                Some(address + off as u32),
                None,
                Some(&mut data[off..off + sz]),
            )?;
            off += sz;
        }
        self.busy = false;
        Ok(())
    }

    /// Read portion of a page
    pub fn read_page(&mut self, page: u32, data: &mut [u8], offset: usize) -> Result<()> {
        let max = SPIF_PAGE_SIZE - offset;
        let sz = core::cmp::min(data.len(), max);
        let address = page * SPIF_PAGE_SIZE as u32 + offset as u32;
        self.read_address(address, &mut data[..sz])
    }

    pub fn write_sector(
        &mut self,
        sector: u32,
        data: &[u8],
        offset: usize,
        mut delay: impl FnMut(u32),
    ) -> Result<()> {
        if offset >= SPIF_SECTOR_SIZE {
            return Err(-3);
        }
        let mut written = 0;
        let mut page = sector * (SPIF_SECTOR_SIZE as u32 / SPIF_PAGE_SIZE as u32);
        page += (offset as u32) / SPIF_PAGE_SIZE as u32;
        let mut rem = core::cmp::min(data.len(), SPIF_SECTOR_SIZE - offset);
        let mut page_offset = offset % SPIF_PAGE_SIZE;
        while rem > 0 && page < ((sector + 1) * (SPIF_SECTOR_SIZE as u32 / SPIF_PAGE_SIZE as u32)) {
            let n = core::cmp::min(SPIF_PAGE_SIZE - page_offset, rem);
            self.write_page(page, &data[written..written + n], page_offset, &mut delay)?;
            written += n;
            rem -= n;
            page += 1;
            page_offset = 0;
        }
        Ok(())
    }

    pub fn read_sector(&mut self, sector: u32, data: &mut [u8], offset: usize) -> Result<()> {
        if offset >= SPIF_SECTOR_SIZE {
            return Err(-3);
        }
        let n = core::cmp::min(data.len(), SPIF_SECTOR_SIZE - offset);
        let address = sector * SPIF_SECTOR_SIZE as u32 + offset as u32;
        self.read_address(address, &mut data[..n])
    }

    pub fn write_block(
        &mut self,
        block: u32,
        data: &[u8],
        offset: usize,
        mut delay: impl FnMut(u32),
    ) -> Result<()> {
        if offset >= SPIF_BLOCK_SIZE {
            return Err(-3);
        }
        let mut written = 0;
        let mut page = block * (SPIF_BLOCK_SIZE as u32 / SPIF_PAGE_SIZE as u32);
        page += (offset as u32) / SPIF_PAGE_SIZE as u32;
        let mut rem = core::cmp::min(data.len(), SPIF_BLOCK_SIZE - offset);
        let mut page_offset = offset % SPIF_PAGE_SIZE;
        while rem > 0 && page < ((block + 1) * (SPIF_BLOCK_SIZE as u32 / SPIF_PAGE_SIZE as u32)) {
            let n = core::cmp::min(SPIF_PAGE_SIZE - page_offset, rem);
            self.write_page(page, &data[written..written + n], page_offset, &mut delay)?;
            written += n;
            rem -= n;
            page += 1;
            page_offset = 0;
        }
        Ok(())
    }

    pub fn read_block(&mut self, block: u32, data: &mut [u8], offset: usize) -> Result<()> {
        if offset >= SPIF_BLOCK_SIZE {
            return Err(-3);
        }
        let n = core::cmp::min(data.len(), SPIF_BLOCK_SIZE - offset);
        let address = block * SPIF_BLOCK_SIZE as u32 + offset as u32;
        self.read_address(address, &mut data[..n])
    }

    pub fn is_busy(&self) -> bool {
        self.busy
    }
}
