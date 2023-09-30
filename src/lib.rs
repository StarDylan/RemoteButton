use std::{ascii::escape_default, fmt::Display};

use device_query::Keycode;
use enigo::Key;
use serde::{Serialize, Deserialize};

pub const STARFIELD_TOPIC: &str = "starfield/key";
pub const NEW_PERSON_TOPIC: &str = "starfield/new";


#[derive(Serialize, Deserialize)]
pub struct NewClient {
    pub is_reciever: bool,
    pub hostname: String,
    pub is_ack: bool
}

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
            Keycode::Left => Ok(Self::LeftArrow),
            Keycode::Right => Ok(Self::RightArrow),
            Keycode::Up => Ok(Self::UpArrow),
            Keycode::Down => Ok(Self::DownArrow),
            _ => Err("Not a Sendable Key")
        }
    }
}

impl TryInto<Key> for SendableKeys {
    type Error = &'static str;

    fn try_into(self) -> Result<Key, Self::Error> {
        match self {
            SendableKeys::Num(number) => match number {
                0 => Ok(Key::Num0),
                1 => Ok(Key::Num1),
                2 => Ok(Key::Num2),
                3 => Ok(Key::Num3),
                4 => Ok(Key::Num4),
                5 => Ok(Key::Num5),
                6 => Ok(Key::Num6),
                7 => Ok(Key::Num7),
                8 => Ok(Key::Num8),
                9 => Ok(Key::Num9),
                _ => Err("Cannot Convert this Number to Key"),
            },
            SendableKeys::Letter(letter) => match letter {
                'R' => Ok(Key::R),
                'Z' => Ok(Key::Z),
                'G' => Ok(Key::G),
                _ => Err("Cannot Convert this Letter to Key"),
            },
            SendableKeys::ESC => Ok(Key::Escape),
            SendableKeys::LeftArrow => Ok(Key::LeftArrow),
            SendableKeys::RightArrow => Ok(Key::RightArrow),
            SendableKeys::UpArrow => Ok(Key::UpArrow),
            SendableKeys::DownArrow => Ok(Key::DownArrow),
            
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