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

/// Constants for brightness
pub const BRIGHTNESS_MAX: u8 = 0x0f;
pub const BRIGHTNESS_HALF: u8 = 0x07;
pub const BRIGHTNESS_LOW: u8 = 0x0f;
pub const BRIGHTNESS_OFF: u8 = 0x00;

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
    pub fn default(i2c_port: I2C) -> Result<Self, Error<CommE>> {
        Self::new(i2c_port, DEFAULT_I2C_ADDRESS)
    }

    pub fn new(i2c_port: I2C, address: u8) -> Result<Self, Error<CommE>> {
        let mut inst = Self { i2c_port, address };
        inst.reset()?;
        Ok(inst)
    }

    /// Reset the device to an off state
    pub fn reset(&mut self) -> Result<(), Error<CommE>> {
        // probe the device
        self.get_enabled()?;
        // completely turn off light output
        self.set_enabled(false)?;
        self.set_white_brightness(BRIGHTNESS_OFF)
    }

    /// Set white light brightness, where each color has the same brightness.
    /// You can use BRIGHTNESS_xxx constants here
    pub fn set_white_brightness(&mut self, brightness: u8) -> Result<(), Error<CommE>> {
        let bright = BRIGHTNESS_MAX & brightness;
        self.set_color_brightness(bright, bright, bright)
    }

    /// Set the color brightness values independently.
    /// You can use BRIGHTNESS_xxx constants here
    pub fn set_color_brightness(
        &mut self,
        red: u8,
        green: u8,
        blue: u8,
    ) -> Result<(), Error<CommE>> {
        // write to the PWM0 address with auto-increment turned on,
        // which allows us to write to PWM0,1,2 at the same time
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

    /// Check whether the device has light output enabled
    pub fn get_enabled(&mut self) -> Result<bool, Error<CommE>> {
        let mut read_buf = [0u8; 2];
        self.i2c_port
            .read(self.address, &mut read_buf)
            .map_err(Error::Comm)?;

        // on = ((result[0] >> 4) & SETTING_ENABLE);

        let enabled = BIT_ENABLE == ((read_buf[0] >> 4) & BIT_ENABLE);
        Ok(enabled)
    }

    /// Enable or disable light output
    pub fn set_enabled(&mut self, enable: bool) -> Result<(), Error<CommE>> {
        let settings = if enable {
            BIT_SHDN | BIT_ENABLE
        } else {
            BIT_SHDN
        };
        self.write_sub_address(SUBADDR_NAA_SETTINGS, settings)
    }

    /// Toggle enabled/disabled
    /// This can be used to quickly blink or flash the light output.
    pub fn toggle(&mut self) -> Result<(), Error<CommE>> {
        let enabled = self.get_enabled()?;
        self.set_enabled(!enabled)
    }

    /// Write a value to a sub-address on the device
    fn write_sub_address(&mut self, subaddr: u8, val: u8) -> Result<(), Error<CommE>> {
        let write_buf = [subaddr, val];
        self.i2c_port
            .write(self.address, &write_buf)
            .map_err(Error::Comm)?;
        Ok(())
    }
}

const DEFAULT_I2C_ADDRESS: u8 = 0x55;

/// Blue PWM auto-increment subaddress
const SUBADDR_AA_PWM0: u8 = 0x01;
// /// Green PWM auto-increment subaddress
// const SUBADDR_AA_PWM1: u8 = 0x02;
// /// Red PWM auto-increment subaddress
// const SUBADDR_AA_PWM2: u8 = 0x03;

const AUTO_INCREMENT_OFF: u8 = 0x80;
// Note that these sub-addresses have 0x80 added to them, which turns off auto-increment
// When the AI flag is set high, auto-increment is OFF; when it is set low, auto-increment is ON.
// /// Blue PWM
//const SUBADDR_PWM0 : u8 = (SUBADDR_AA_PWM0 | AUTO_INCREMENT_OFF);
// /// Green PWM
//const SUBADDR_PWM1 : u8 = (SUBADDR_AA_PWM1 | AUTO_INCREMENT_OFF);
// /// Red PWM
//const SUBADDR_PWM2 : u8 = (SUBADDR_AA_PWM2 | AUTO_INCREMENT_OFF);

/// Settings configuration: ENABLE/SHDN
const SUBADDR_NAA_SETTINGS: u8 = (0x04 | AUTO_INCREMENT_OFF);

/// SHDN setting: H: Output blinks at PWM0, PWM1, and PWM2 rate L: Power-saving mode
const BIT_SHDN: u8 = (1 << 0);

/// ENABLE setting: H: Output blinks at PWM0, PWM1, and PWM2 rate L: Output is OFF
const BIT_ENABLE: u8 = (1 << 1);
