#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use embedded_hal::i2c::I2c;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );

    let mut buf = [0u8; 8];

    loop {
        I2c::write(&mut i2c, 0x70, &buf).unwrap();
        for b in buf.iter_mut() {
            *b = *b ^ 1;
        }
        arduino_hal::delay_ms(1000);
    }
}
