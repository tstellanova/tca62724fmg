# tca62724fmg

The Toshiba TCA62724FMG is a three output RGB LED with an I2C interface. 
It features high brightness and three independent PWM controllers.
It's commonly used for status indicator lights.

![rgbled demo gif](https://i.imgur.com/NzL362X.gif)

## Resources
- [Datasheet](https://datasheet.ciiva.com/12276/21212-21811-12276938.pdf)

## License
BSD-3 : see LICENSE file. 
 
## Status

- [x] Builds 
- [x] Tested with hardware device
- [x] Supports setting color channels individually
- [x] Supports setting white values (all color channels equal brightness)
- [x] Supports toggling on and off light output

## Examples

This example has been tested with stm32h743 in a 
[Holybro Durandal](http://www.holybro.com/product/durandalbeta/) flight controller,
with an externally connected 
[M8N GPS module](http://www.holybro.com/product/pixhawk-4-gps-module/)
containing a tca62724fmg rgbled.

```
cargo run --example sweep --target thumbv7em-none-eabihf
```

The `memory.x` ,`.cargo/config`, and `dronecode.gdb` files included with this crate are
configured to run this example by connecting to the Durandal via a dronecode
probe (or similar, such as a Black Magic Probe)
