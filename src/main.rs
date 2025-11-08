#![no_std]
#![no_main]

mod sevseg;

use crate::sevseg::{Digit, Seg, SegDisplay, SevenSeg};
use arduino_hal::hal::port::{PD0, PD1};
use arduino_hal::hal::{Atmega, Usart};
use arduino_hal::pac::USART0;
use arduino_hal::port::mode::{Input, Output};
use arduino_hal::port::Pin;
use arduino_hal::DefaultClock;
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

    scroll_numbers(seg, &mut serial);

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

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let potentiometer = pins.a3.into_analog_input(&mut adc);

    let mut pot_val = 0;

    loop {
        for i in 0..4 {
            let idx = (start_idx + i) % nums.len();
            output[i] = nums[idx].seg_display();
            if idx == 1 {
                output[i] |= Seg::Dot as u16;
            }
        }

        pot_val = potentiometer.analog_read(&mut adc);
        uwriteln!(&mut serial, "Pot Value: {}", pot_val).unwrap();

        seg.write(output[0], output[1], output[2], output[3], true);

        start_idx += 1;
        arduino_hal::delay_ms(100);
    }
}

fn scroll_numbers(mut seg: SevenSeg) {
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
