use int_enum::IntEnum;
use enum_iterator::{all, Sequence};

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9
}

pub fn color_to_value(color: ResistorColor) -> u8 {
    color.int_value()
}

pub fn value_to_color_string(value: u8) -> String {
    let color = ResistorColor::from_int(value);

    if color.is_err() {
        return String::from("value out of range");
    }

    format!("{:?}", color.unwrap())
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
}
