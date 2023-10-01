use std::ascii::escape_default;

use device_query::Keycode;
use serde::{Serialize, Deserialize};

pub const STARFIELD_TOPIC: &str = "starfield/key";


#[derive(Serialize, Deserialize, Debug)]
pub enum SendableKeys {
    Num(u8),
    Letter(char),
    ESC,
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,
}

impl TryFrom<&Keycode> for SendableKeys{
    type Error = &'static str;

    fn try_from(value: &Keycode) -> Result<Self, Self::Error> {
        match value {
            Keycode::Key0 => Ok(Self::Num(0)),
            Keycode::Key1 => Ok(Self::Num(1)),
            Keycode::Key2 => Ok(Self::Num(2)),
            Keycode::Key3 => Ok(Self::Num(3)),
            Keycode::Key4 => Ok(Self::Num(4)),
            Keycode::Key5 => Ok(Self::Num(5)),
            Keycode::Key6 => Ok(Self::Num(6)),
            Keycode::Key7 => Ok(Self::Num(7)),
            Keycode::Key8 => Ok(Self::Num(8)),
            Keycode::Key9 => Ok(Self::Num(9)),
            Keycode::G => Ok(Self::Letter('G')),
            Keycode::R => Ok(Self::Letter('R')),
            Keycode::Z => Ok(Self::Letter('Z')),
            Keycode::Escape => Ok(Self::ESC),
            Keycode::A => Ok(Self::LeftArrow),
            Keycode::D => Ok(Self::RightArrow),
            Keycode::W => Ok(Self::UpArrow),
            Keycode::S => Ok(Self::DownArrow),
            _ => Err("Not a Sendable Key")
        }
    }
}

pub fn show_bytes(bs: &[u8]) -> String {
    let mut visible = String::new();
    for &b in bs {
        let part: Vec<u8> = escape_default(b).collect();
        visible.push_str(std::str::from_utf8(&part).unwrap());
    }
    visible
}