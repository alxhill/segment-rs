#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use embedded_hal::i2c::I2c;
use panic_halt as _;
use ufmt::uwrite;

static NUMBERS: &[u8] = &[
    0b00111111, // 0
    0b00000110, // 1
    0b01011011, // 2
    0b01001111, // 3
    0b01100110, // 4
    0b01101101, // 5
    0b01111101, // 6
    0b00000111, // 7
    0b01111111, // 8
    0b01101111, // 9
];

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

    // third digit controls the colon
    let mut numbers: [u8; 5] = [1, 2, 0x2, 3, 4];

    let mut display_buf = [0u16; 6];

    let mut dot: u16 = 1 << 7;

    loop {
        let buf: &[u8] = bytemuck::cast_slice(&mut display_buf);
        I2c::write(&mut i2c, 0x70, &buf[1..]).unwrap();

        for (idx, number) in numbers.iter_mut().enumerate() {
            if idx == 2 {
                continue;
            }

            if *number > 9 {
                *number = 0;
            }
            dot ^= 1 << 7;
            display_buf[idx + 1] = NUMBERS[*number as usize] as u16 | dot;
            *number += 1;
        }

        // flash the colon
        display_buf[3] ^= 0x2;

        arduino_hal::delay_ms(500);
    }
}
