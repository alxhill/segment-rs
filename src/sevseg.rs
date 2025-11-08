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

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum Seg {
    Top = 1,
    TopR = 1 << 1,
    BotR = 1 << 2,
    Bot = 1 << 3,
    BotL = 1 << 4,
    TopL = 1 << 5,
    Mid = 1 << 6,
    Dot = 1 << 7,
}

macro_rules! segs {
    ($($s:expr),* $(,)?) => { 0u16 $(| ($s as u16))* };
}

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum Digit {
    Zero = segs!(
        Seg::Top,
        Seg::TopL,
        Seg::TopR,
        Seg::Bot,
        Seg::BotL,
        Seg::BotR
    ),
    One = segs!(Seg::TopR, Seg::BotR),
    Two = segs!(Seg::Top, Seg::TopR, Seg::Mid, Seg::BotL, Seg::Bot),
    Three = segs!(Seg::Top, Seg::TopR, Seg::BotR, Seg::Bot, Seg::Mid),
    Four = segs!(Seg::TopL, Seg::Mid, Seg::TopR, Seg::BotR),
    Five = segs!(Seg::Top, Seg::TopL, Seg::Mid, Seg::BotR, Seg::Bot),
    Six = segs!(
        Seg::Top,
        Seg::TopL,
        Seg::Mid,
        Seg::BotL,
        Seg::BotR,
        Seg::Bot
    ),
    Seven = segs!(Seg::Top, Seg::TopR, Seg::BotR),
    Eight = segs!(
        Seg::Top,
        Seg::TopL,
        Seg::TopR,
        Seg::Bot,
        Seg::BotL,
        Seg::BotR,
        Seg::Mid
    ),
    Nine = segs!(
        Seg::Top,
        Seg::TopL,
        Seg::TopR,
        Seg::Bot,
        Seg::BotR,
        Seg::Mid
    ),
}

pub trait SegDisplay {
    fn seg_display(&self) -> u16;
}

impl SegDisplay for u16 {
    fn seg_display(&self) -> u16 {
        *self
    }
}

impl SegDisplay for Digit {
    fn seg_display(&self) -> u16 {
        *self as u16
    }
}

impl SegDisplay for Seg {
    fn seg_display(&self) -> u16 {
        *self as u16
    }
}

impl<T1: SegDisplay, T2: SegDisplay> SegDisplay for (T1, T2) {
    fn seg_display(&self) -> u16 {
        self.0.seg_display() | self.1.seg_display()
    }
}

impl SevenSeg {
    pub fn init(mut i2c: arduino_hal::I2c, addr: u8) -> Self {
        i2c.write(addr, &[ENABLE_OSCILLATOR]).unwrap();
        i2c.write(addr, &[0u8; 16]).unwrap();
        i2c.write(addr, &[BLINK_CMD | DISPLAY_ON]).unwrap();
        i2c.write(addr, &[BRIGHTNESS_CMD | MAX_BRIGHTNESS]).unwrap();

        Self { addr, i2c }
    }

    pub fn write(&mut self, char1: impl SegDisplay, char2: impl SegDisplay, char3: impl SegDisplay, char4: impl SegDisplay, colon: bool) {
        let colon = if colon { 0x2 } else { 0x0 };
        let display_buf = [
            0u16, // write cmd (u8, first half of u16 is skipped write
            char1.seg_display(),
            char2.seg_display(),
            colon,
            char3.seg_display(),
            char4.seg_display(),
        ];

        let write_buf = bytemuck::cast_slice(&display_buf);

        self.i2c.write(self.addr, &write_buf[1..]).unwrap();
    }
}
