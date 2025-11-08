#![no_std]
#![no_main]

mod sevseg;

use crate::sevseg::{Digit, Seg, SevenSeg};
use arduino_hal::i2c::{Direction, I2cOps};
use arduino_hal::prelude::*;
use embedded_hal::i2c::I2c;
use panic_halt as _;
use ufmt::{uwrite, uwriteln};

const NUMBERS: &[u8] = &[
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

const BLINK_CMD: u8 = 0x80;
const BRIGHTNESS_CMD: u8 = 0xE0;
const DISPLAY_ON: u8 = 0x01;
const ENABLE_OSCILLATOR: u8 = 0x21u8;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );

    // // turn on oscillator
    // I2c::write(&mut i2c, 0x70, &[ENABLE_OSCILLATOR]).unwrap();
    //
    // let display_on_cmd = BLINK_CMD | DISPLAY_ON;
    // I2c::write(&mut i2c, 0x70, &[display_on_cmd]).unwrap();
    //
    // let brightness_cmd = BRIGHTNESS_CMD | 15; // max brightness = 15
    // I2c::write(&mut i2c, 0x70, &[brightness_cmd]).unwrap();
    //
    // // third digit controls the colon
    // let mut numbers: [u8; 5] = [1, 2, 0x2, 3, 4];
    //
    // let mut display_buf = [0u16; 9];
    //
    // let dot: u16 = 0b1000_0000;

    uwriteln!(serial, "init seven seg").unwrap();

    let mut seg = SevenSeg::init(i2c, 0x70);

    uwriteln!(serial, "starting loop").unwrap();

    let nums = [
        Digit::Zero,
        Digit::One,
        Digit::Two,
        Digit::Three,
        Digit::Four,
        Digit::Five,
        Digit::Six,
        Digit::Seven,
        Digit::Eight,
        Digit::Nine,
    ];

    let mut output = [Digit::Zero; 4];

    let mut colon = true;

    let start_idx = 0;

    loop {
        for i in 0..4 {
            let idx = (start_idx + i) % nums.len();
            output[i] = nums[idx];
        }

        seg.write(output[0], output[1], output[2], output[3], colon);

        colon = !colon;
        arduino_hal::delay_ms(1000);
    }

    //     loop {
    //         let buf: &[u8] = bytemuck::cast_slice(&mut display_buf);
    //         I2c::write(&mut i2c, 0x70, &buf[1..]).unwrap();
    //         // uwriteln!(serial, "wrote {:?}", numbers).unwrap();
    //
    //         for (idx, number) in numbers.iter_mut().enumerate() {
    //             if idx == 2 {
    //                 continue;
    //             }
    //
    //             if *number > 9 {
    //                 *number = 0;
    //             }
    //
    //             display_buf[idx + 1] = NUMBERS[*number as usize] as u16;
    //
    //             if *number == 1 {
    //                 display_buf[idx + 1] |= dot;
    //             }
    //             *number += 1;
    //         }
    // // show the colon
    //         display_buf[3] = 0x2;
    //
    //         arduino_hal::delay_ms(100);
    //     }
}
