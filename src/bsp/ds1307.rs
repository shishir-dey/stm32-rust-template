use crate::driver::i2c::{I2c, Result};

/// DS1307 RTC chip I2C address
const DS1307_I2C_ADDRESS: u32 = 0x68;

/// DS1307 Register addresses
const DS1307_ADDR_SEC: u8 = 0x00;
const DS1307_ADDR_MIN: u8 = 0x01;
const DS1307_ADDR_HRS: u8 = 0x02;
const DS1307_ADDR_DAY: u8 = 0x03;
const DS1307_ADDR_DATE: u8 = 0x04;
const DS1307_ADDR_MONTH: u8 = 0x05;
const DS1307_ADDR_YEAR: u8 = 0x06;

/// Time format constants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeFormat {
    /// 12-hour format AM
    TwelveHoursAM = 0,
    /// 12-hour format PM
    TwelveHoursPM = 1,
    /// 24-hour format
    TwentyFourHours = 2,
}

/// Days of the week
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayOfWeek {
    Sunday = 1,
    Monday = 2,
    Tuesday = 3,
    Wednesday = 4,
    Thursday = 5,
    Friday = 6,
    Saturday = 7,
}

/// RTC Date structure
#[derive(Debug, Clone, Copy)]
pub struct RtcDate {
    pub date: u8,
    pub month: u8,
    pub year: u8,
    pub day: DayOfWeek,
}

/// RTC Time structure
#[derive(Debug, Clone, Copy)]
pub struct RtcTime {
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8,
    pub time_format: TimeFormat,
}

/// DS1307 RTC Driver
pub struct DS1307<I2C> {
    i2c: I2C,
}

impl<I2C> DS1307<I2C>
where
    I2C: I2c<'static>,
{
    /// Create a new DS1307 driver instance
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    /// Initialize the DS1307 RTC
    /// Returns Ok(()) if initialization successful, Err if clock halt bit is set
    pub fn init(&mut self) -> Result<()> {
        // Clear the clock halt bit (bit 7 of seconds register)
        self.write_register(DS1307_ADDR_SEC, 0x00)?;

        // Read back the clock halt bit to verify initialization
        let clock_state = self.read_register(DS1307_ADDR_SEC)?;

        // Check if clock halt bit (bit 7) is still set
        if (clock_state >> 7) & 0x1 != 0 {
            return Err(-1); // Initialization failed
        }

        Ok(())
    }

    /// Set the current time on the DS1307
    pub fn set_current_time(&mut self, rtc_time: &RtcTime) -> Result<()> {
        // Convert seconds to BCD and clear clock halt bit
        let mut seconds = Self::binary_to_bcd(rtc_time.seconds);
        seconds &= !(1 << 7); // Clear bit 7 (clock halt)
        self.write_register(DS1307_ADDR_SEC, seconds)?;

        // Convert and write minutes
        let minutes = Self::binary_to_bcd(rtc_time.minutes);
        self.write_register(DS1307_ADDR_MIN, minutes)?;

        // Handle hours with time format
        let mut hours = Self::binary_to_bcd(rtc_time.hours);
        match rtc_time.time_format {
            TimeFormat::TwentyFourHours => {
                hours &= !(1 << 6); // Clear 12/24 hour bit for 24-hour format
            }
            TimeFormat::TwelveHoursAM => {
                hours |= 1 << 6; // Set 12/24 hour bit for 12-hour format
                hours &= !(1 << 5); // Clear AM/PM bit for AM
            }
            TimeFormat::TwelveHoursPM => {
                hours |= 1 << 6; // Set 12/24 hour bit for 12-hour format
                hours |= 1 << 5; // Set AM/PM bit for PM
            }
        }
        self.write_register(DS1307_ADDR_HRS, hours)?;

        Ok(())
    }

    /// Set the current date on the DS1307
    pub fn set_current_date(&mut self, rtc_date: &RtcDate) -> Result<()> {
        self.write_register(DS1307_ADDR_DATE, Self::binary_to_bcd(rtc_date.date))?;
        self.write_register(DS1307_ADDR_MONTH, Self::binary_to_bcd(rtc_date.month))?;
        self.write_register(DS1307_ADDR_YEAR, Self::binary_to_bcd(rtc_date.year))?;
        self.write_register(DS1307_ADDR_DAY, Self::binary_to_bcd(rtc_date.day as u8))?;
        Ok(())
    }

    /// Get the current time from the DS1307
    pub fn get_current_time(&mut self) -> Result<RtcTime> {
        // Read seconds and clear clock halt bit
        let mut seconds = self.read_register(DS1307_ADDR_SEC)?;
        seconds &= !(1 << 7); // Clear clock halt bit for BCD conversion

        let minutes = self.read_register(DS1307_ADDR_MIN)?;
        let mut hours = self.read_register(DS1307_ADDR_HRS)?;

        // Determine time format and extract hours
        let time_format = if (hours & (1 << 6)) != 0 {
            // 12-hour format
            let is_pm = (hours & (1 << 5)) != 0;
            hours &= !(0x3 << 5); // Clear bits 6 and 5
            if is_pm {
                TimeFormat::TwelveHoursPM
            } else {
                TimeFormat::TwelveHoursAM
            }
        } else {
            // 24-hour format
            TimeFormat::TwentyFourHours
        };

        Ok(RtcTime {
            seconds: Self::bcd_to_binary(seconds),
            minutes: Self::bcd_to_binary(minutes),
            hours: Self::bcd_to_binary(hours),
            time_format,
        })
    }

    /// Get the current date from the DS1307
    pub fn get_current_date(&mut self) -> Result<RtcDate> {
        let day_raw = self.read_register(DS1307_ADDR_DAY)?;
        let date = self.read_register(DS1307_ADDR_DATE)?;
        let month = self.read_register(DS1307_ADDR_MONTH)?;
        let year = self.read_register(DS1307_ADDR_YEAR)?;

        let day = match Self::bcd_to_binary(day_raw) {
            1 => DayOfWeek::Sunday,
            2 => DayOfWeek::Monday,
            3 => DayOfWeek::Tuesday,
            4 => DayOfWeek::Wednesday,
            5 => DayOfWeek::Thursday,
            6 => DayOfWeek::Friday,
            7 => DayOfWeek::Saturday,
            _ => DayOfWeek::Sunday, // Default fallback
        };

        Ok(RtcDate {
            day,
            date: Self::bcd_to_binary(date),
            month: Self::bcd_to_binary(month),
            year: Self::bcd_to_binary(year),
        })
    }

    /// Write a value to a DS1307 register
    fn write_register(&mut self, reg_addr: u8, value: u8) -> Result<()> {
        let tx_data = [reg_addr, value];
        self.i2c
            .master_transmit(DS1307_I2C_ADDRESS, &tx_data, false)
    }

    /// Read a value from a DS1307 register
    fn read_register(&mut self, reg_addr: u8) -> Result<u8> {
        // Send register address
        self.i2c
            .master_transmit(DS1307_I2C_ADDRESS, &[reg_addr], true)?;

        // Read the register value
        let mut data = [0u8; 1];
        self.i2c
            .master_receive(DS1307_I2C_ADDRESS, &mut data, false)?;

        Ok(data[0])
    }

    /// Convert binary value to BCD (Binary Coded Decimal)
    fn binary_to_bcd(value: u8) -> u8 {
        if value >= 10 {
            let tens = value / 10;
            let ones = value % 10;
            (tens << 4) | ones
        } else {
            value
        }
    }

    /// Convert BCD (Binary Coded Decimal) to binary value
    fn bcd_to_binary(value: u8) -> u8 {
        let tens = (value >> 4) * 10;
        let ones = value & 0x0F;
        tens + ones
    }
}

impl<I2C> DS1307<I2C> {
    /// Release the I2C peripheral and consume the driver
    pub fn release(self) -> I2C {
        self.i2c
    }
}

// Helper functions for creating date/time structures
impl RtcDate {
    /// Create a new RtcDate
    pub fn new(date: u8, month: u8, year: u8, day: DayOfWeek) -> Self {
        Self {
            date,
            month,
            year,
            day,
        }
    }
}

impl RtcTime {
    /// Create a new RtcTime
    pub fn new(seconds: u8, minutes: u8, hours: u8, time_format: TimeFormat) -> Self {
        Self {
            seconds,
            minutes,
            hours,
            time_format,
        }
    }
}
