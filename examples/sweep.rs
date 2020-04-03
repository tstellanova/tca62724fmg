#![no_main]
#![no_std]

extern crate panic_semihosting;

use p_hal::{pac, prelude::*};
use stm32h7xx_hal as p_hal;

use cortex_m_rt::entry;

use tca62724fmg::TCA62724FMG;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let vos = pwr.freeze();

    // Constrain and Freeze clock
    let rcc = dp.RCC.constrain();
    let mut ccdr = rcc.sys_ck(100.mhz()).freeze(vos, &dp.SYSCFG);

    let clocks = ccdr.clocks;
    let mut delay_source = p_hal::delay::Delay::new(cp.SYST, clocks);

    // Grab the only GPIO we need for this example
    let gpiob = dp.GPIOB.split(&mut ccdr.ahb4);

    // Configure SCL and SDA pins for I2C1
    let scl = gpiob.pb8.into_alternate_af4().set_open_drain();
    let sda = gpiob.pb9.into_alternate_af4().set_open_drain();

    // Note the bandwidth selected here (100 kHz): this is for an external
    // i2c device with about a 30 cm  cable connecting it to the stm32h743
    let i2c1_port = dp.I2C1.i2c((scl, sda), 100.khz(), &ccdr);

    // Create an instance of the rgbled
    let mut rgbled = TCA62724FMG::default(i2c1_port).unwrap();
    rgbled
        .set_white_brightness(tca62724fmg::BRIGHTNESS_HALF)
        .unwrap();
    rgbled.set_enabled(true).unwrap();

    const MAX_BRIGHT: u8 = tca62724fmg::BRIGHTNESS_MAX;

    loop {
        // Monochrome / white loop:
        // Sweep through every white brightness value this device supports
        for brightness in 0..MAX_BRIGHT {
            // White / monochrome loop:
            // Sweep through all the white values that this device supports
            let _result = rgbled.set_white_brightness(brightness);
            delay_source.delay_ms(250u8);
        }

        // Color loop:
        // Sweep through every color and brightness  this device supports
        for blue in 0..MAX_BRIGHT {
            for red in 0..MAX_BRIGHT {
                for green in 0..MAX_BRIGHT {
                    let _result = rgbled.set_color_brightness(red, green, blue);
                    delay_source.delay_ms(50u8);
                }
            }
        }
    }
}
