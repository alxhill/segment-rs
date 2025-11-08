use arduino_hal::{Peripherals, Pins};
use embedded_hal::i2c::I2c;

const BLINK_CMD: u8 = 0x80;
const BRIGHTNESS_CMD: u8 = 0xE0;
const DISPLAY_ON: u8 = 0x01;
const ENABLE_OSCILLATOR: u8 = 0x21u8;
const MAX_BRIGHTNESS: u8 = 15;


pub struct SevenSeg {
    addr: u8,
    i2c: arduino_hal::I2c,
}


#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Segment {
    Top,
    TopRight,
    BottomRight,
    Bottom,
    BottomLeft,
    TopLeft,
    Middle,
    Dot,
}

impl Segment {
    pub fn bytes(&self) -> u16 {
        1 << *self as u8
    }
}

impl SevenSeg {
    pub fn init(dp: Peripherals, pins: Pins, addr: u8) -> Self {
        let mut i2c = arduino_hal::I2c::new(
            dp.TWI,
            pins.a4.into_pull_up_input(),
            pins.a5.into_pull_up_input(),
            50000,
        );

        i2c.write(addr, &[ENABLE_OSCILLATOR]).unwrap();
        i2c.write(addr, &[0u8; 16]).unwrap();
        i2c.write(addr, &[BLINK_CMD | DISPLAY_ON]).unwrap();
        i2c.write(addr, &[BRIGHTNESS_CMD | MAX_BRIGHTNESS]).unwrap();

        Self {
            addr,
            i2c
        }
    }
}