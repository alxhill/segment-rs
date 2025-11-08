#![no_std]
#![no_main]

mod sevseg;

use crate::sevseg::{Digit, Seg, SegDisplay, SevenSeg};
use panic_halt as _;
use ufmt::uwriteln;

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

    let mut seg = SevenSeg::init(i2c, 0x70, 15);

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

    let mut output = [Digit::Zero as u16; 4];

    let mut start_idx = 0;

    uwriteln!(&mut serial, "Starting Write Loop").unwrap();

    loop {
        for i in 0..4 {
            let idx = (start_idx + i) % nums.len();
            output[i] = nums[idx].seg_display();
            if idx == 1 {
                output[i] |= Seg::Dot as u16;
            }
        }

        seg.write(output[0], output[1], output[2], output[3], true);

        start_idx += 1;
        arduino_hal::delay_ms(100);
    }
}
