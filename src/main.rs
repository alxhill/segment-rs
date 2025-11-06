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

    let mut display_buf = [0u16; 9];

    loop {
        let buf: &[u8] = bytemuck::cast_slice(&mut display_buf);
        I2c::write(&mut i2c, 0x70, &buf[1..]).unwrap();
        for b in display_buf.iter_mut().skip(1) {
            *b = *b ^ 1;
        }
        arduino_hal::delay_ms(1000);
    }
}
