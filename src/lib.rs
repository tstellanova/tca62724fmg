/*
Copyright (c) 2020 Todd Stellanova
LICENSE: BSD3 (see LICENSE file)
*/

#![no_std]

use embedded_hal as hal;

/// Errors in this crate
#[derive(Debug)]
pub enum Error<CommE> {
    /// Communication error
    Comm(CommE),
}


pub struct TCA62724FMG<I2C> {
    i2c_port: I2C,
    address: u8,
}

impl<I2C, CommE> TCA62724FMG<I2C>
    where
        I2C: hal::blocking::i2c::Write<Error = CommE>
        + hal::blocking::i2c::Read<Error = CommE>
        + hal::blocking::i2c::WriteRead<Error = CommE>,
{
    pub fn default(i2c_port: I2C) ->  Result<Self, Error<CommE>> {
        Self::new(i2c_port, DEFAULT_I2C_ADDRESS)
    }

    pub fn new(i2c_port: I2C, address: u8) -> Result<Self, Error<CommE>> {
        let mut inst = Self {
            i2c_port,
            address
        };

        inst.reset()?;
        Ok(inst)
    }

    pub fn reset(&mut self) -> Result<(), Error<CommE>> {
        self.get_enabled()?;
        self.set_enable(false)?;
        self.set_white_brightness(BRIGHTNESS_OFF)
    }

    /// Set white light brightness
    /// You can use BRIGHTNESS_xxx constants here
    pub fn set_white_brightness(&mut self, brightness: u8) -> Result<(), Error<CommE>> {
        let bright = BRIGHTNESS_MAX & brightness;
        self.set_color_brightness(bright, bright, bright)
    }

    pub fn set_enable(&mut self, enable: bool) -> Result<(), Error<CommE>>  {
        let settings =
            if enable {
                BIT_SHDN | BIT_ENABLE
            }
            else {
                0
            };

        self.write_sub_address(SUBADDR_NAA_SETTINGS, settings)
    }

    pub fn write_sub_address(&mut self, subaddr: u8, val: u8)  -> Result<(), Error<CommE>> {
        let write_buf = [subaddr, val];
        self.i2c_port
            .write(self.address, &write_buf)
            .map_err(Error::Comm)?;
        Ok(())
    }

    /// Set the color and brightness values all at once
    /// You can use BRIGHTNESS_xxx constants here
    pub fn set_color_brightness(&mut self, red: u8, green: u8, blue: u8)
        -> Result<(), Error<CommE>> {

        let write_buf = [
            SUBADDR_AA_PWM0,
            blue & BRIGHTNESS_MAX,
            green & BRIGHTNESS_MAX,
            red & BRIGHTNESS_MAX,
        ];

        self.i2c_port
            .write(self.address, &write_buf)
            .map_err(Error::Comm)?;
        Ok(())
    }

    pub fn get_enabled(&mut self) -> Result<bool, Error<CommE>> {
        let mut read_buf = [0u8; 2];
        self.i2c_port
            .read(self.address, &mut read_buf)
            .map_err(Error::Comm)?;

        let enabled =  (read_buf[0] &  (BIT_ENABLE | BIT_SHDN)) == (BIT_ENABLE | BIT_SHDN);
        Ok(enabled)
    }
}

const DEFAULT_I2C_ADDRESS: u8 = 0x55;


/// Blue PWM auto-increment subaddress
const SUBADDR_AA_PWM0: u8 = 0x01;
/// Green PWM auto-increment subaddress
const SUBADDR_AA_PWM1: u8 = 0x02;
/// Red PWM auto-increment subaddress
const SUBADDR_AA_PWM2: u8 = 0x03;

const AUTO_INCREMENT_OFF: u8 = 0x80;
// Note that these sub-addresses have 0x80 added to them, which turns off auto-increment
// When the AI flag is set high, auto-increment is OFF; when it is set low, auto-increment is ON.
/// Blue PWM
const SUBADDR_PWM0 : u8 = (SUBADDR_AA_PWM0 | AUTO_INCREMENT_OFF);
// Green PWM
//const SUBADDR_PWM1 : u8 = (SUBADDR_AA_PWM1 | AUTO_INCREMENT_OFF);
// Red PWM
//const SUBADDR_PWM2 : u8 = (SUBADDR_AA_PWM2 | AUTO_INCREMENT_OFF);

/// Settings configuration: ENABLE/SHDN
const SUBADDR_NAA_SETTINGS : u8 = (0x04 | AUTO_INCREMENT_OFF);


/// Constants for brightness
pub const BRIGHTNESS_MAX: u8 = 0x0f;
pub const BRIGHTNESS_HALF: u8 = 0x07;
pub const BRIGHTNESS_LOW: u8 = 0x0f;
pub const BRIGHTNESS_OFF: u8 = 0x00;

const SET_POWERSAVE_OFF: u8 = 0x01;
const SET_ENABLE: u8 = 0x02;

/// SHDN setting: H: Output blinks at PWM0, PWM1, and PWM2 rate L: Power-saving mode
const BIT_SHDN: u8 = (1 << 0);

/// ENABLE setting: H: Output blinks at PWM0, PWM1, and PWM2 rate L: Output is OFF
const BIT_ENABLE: u8 = (1 << 1);
