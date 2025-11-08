use arduino_hal::{port, Peripherals, Pins};
use arduino_hal::pac::TWI;
use arduino_hal::port::mode::{Floating, Input, PullUp};
use arduino_hal::port::Pin;
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
pub enum Seg {
    Top,
    TopR,
    BotR,
    Bot,
    BotL,
    TopL,
    Mid,
    Dot,
}

impl Seg {
    pub fn bytes(&self) -> u16 {
        1 << *self as u8
    }
}

pub enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}


impl Digit {
    pub fn bytes(&self) -> u16 {
        match self {
            Digit::One => Seg::TopR.bytes() | Seg::BotR.bytes(),
            Digit::Two => Seg::Top.bytes() | Seg::TopR.bytes() | Seg::Mid.bytes() | Seg::BotL.bytes() | Seg::Bot.bytes(),
            Digit::Three => Seg::Top.bytes() | Seg::Mid.bytes() | Seg::Bot.bytes() | Seg::BotR.bytes() | Seg::TopR.bytes(),
            // Digit::Four => {}
            // Digit::Five => {}
            // Digit::Six => {}
            // Digit::Seven => {}
            // Digit::Eight => {}
            // Digit::Nine => {}
            _ => panic!()
        }
    }
}


impl SevenSeg {
    pub fn init(mut i2c: arduino_hal::I2c, addr: u8) -> Self {
        i2c.write(addr, &[ENABLE_OSCILLATOR]).unwrap();
        i2c.write(addr, &[0u8; 16]).unwrap();
        i2c.write(addr, &[BLINK_CMD | DISPLAY_ON]).unwrap();
        i2c.write(addr, &[BRIGHTNESS_CMD | MAX_BRIGHTNESS]).unwrap();

        Self {
            addr,
            i2c
        }
    }

    pub fn write(
        &mut self,
        char1: &Digit,
        char2: &Digit,
        char3: &Digit,
        char4: &Digit,
        colon: bool,
    ) {
        let colon = if colon { 0x2 } else { 0x0 };
        let display_buf = [0u16, char1.bytes(), char2.bytes(), colon, char3.bytes(), char4.bytes()];

        let write_buf = bytemuck::cast_slice(&display_buf);

        self.i2c.write(self.addr, &write_buf[1..]).unwrap();
    }
}