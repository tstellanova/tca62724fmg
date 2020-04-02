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
        // Turn off the LED current
        self.set_enable(false)
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
                SET_ENABLE | SET_POWERSAVE_OFF
            }
            else {
                SET_POWERSAVE_OFF
            };

        self.write_sub_address(SUBADDR_SETTINGS, settings)
    }

    fn write_sub_address(&mut self, subaddr: u8, val: u8)  -> Result<(), Error<CommE>> {
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
            SUBADDR_PWM0, blue & BRIGHTNESS_MAX,
            SUBADDR_PWM1, green & BRIGHTNESS_MAX,
            SUBADDR_PWM2, red & BRIGHTNESS_MAX,
        ];

        self.i2c_port
            .write(self.address, &write_buf)
            .map_err(Error::Comm)?;
        Ok(())
    }
}

const DEFAULT_I2C_ADDRESS: u8 = 0x55;

/// Blue PWM
const SUBADDR_PWM0 : u8 = 0x81;
/// Green PWM
const SUBADDR_PWM1 : u8 = 0x82;
/// Red PWM
const SUBADDR_PWM2 : u8 = 0x83;
/// Settings configuration
const SUBADDR_SETTINGS : u8 = 0x84;


const SET_POWERSAVE_OFF: u8 = 0x01;
const SET_ENABLE: u8 = 0x02;

/// Constants for brightness
pub const BRIGHTNESS_MAX: u8 = 0x0f;
pub const BRIGHTNESS_HALF: u8 = 0x07;
pub const BRIGHTNESS_LOW: u8 = 0x0f;
pub const BRIGHTNESS_OFF: u8 = 0x00;


