use lazy_static::lazy_static;
use rdev::Key;
use serde::{Deserialize, Serialize};
use std::{ascii::escape_default, collections::HashMap, fs, process::exit};

pub const KEY_TOPIC: &str = "remote_button/key";

const APP_NAME: &str = "remote_button";
const CONFIG_FILE_NAME: &str = "remote-button-config.toml";

const DEFAULT_CONFIG_FILE: &str = include_str!("../.config.default.toml");

#[derive(Serialize, Deserialize, Clone)]
pub struct KeyMap {
    pub send_map: HashMap<Key, Key>,
    pub recv_map: HashMap<Key, Key>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub key_map: KeyMap,
    pub mqtt_server_host: String,
    pub mqtt_server_port: u16,
}

pub fn get_config_or_exit() -> Config {
    let proj_dir =
        directories::ProjectDirs::from("rs", "", APP_NAME).expect("Valid UTF-8 Config Path");

    let config_dir = proj_dir.config_dir();

    let config_file = proj_dir.config_dir().with_file_name(CONFIG_FILE_NAME);

    if !config_file.exists() {
        println!(
            "No config file yet at \"{}\"",
            config_file.to_str().expect("Able to write path as str")
        );

        fs::create_dir_all(config_dir).expect("Create all dirs for config");

        fs::write(config_file, DEFAULT_CONFIG_FILE).expect("Able to write to config file");

        println!("Wrote default config file out, please change");

        exit(0);
    }

    let config_str =
        fs::read_to_string(config_file.clone()).expect("Able to read from config file");

    let cfg: Config = toml::from_str(&config_str).expect("Able to deserialize file to Config");

    println!(
        "\nConfig file exists at \"{}\"",
        config_file.to_str().expect("Able to write path as str")
    );
    println!("Please Change if there are errors!");
    println!("****************************************");

    cfg
}

lazy_static! {
    static ref ALL_KEYS: Vec<rdev::Key> = vec![
        Key::DownArrow,
        Key::LeftArrow,
        Key::RightArrow,
        Key::UpArrow,
        Key::Minus,
        Key::Equal,
        Key::Space,
        Key::Tab,
        Key::KeyA,
        Key::KeyB,
        Key::KeyC,
        Key::KeyD,
        Key::KeyE,
        Key::KeyF,
        Key::KeyG,
        Key::KeyH,
        Key::KeyI,
        Key::KeyJ,
        Key::KeyK,
        Key::KeyL,
        Key::KeyM,
        Key::KeyN,
        Key::KeyO,
        Key::KeyP,
        Key::KeyQ,
        Key::KeyR,
        Key::KeyS,
        Key::KeyT,
        Key::KeyU,
        Key::KeyV,
        Key::KeyW,
        Key::KeyX,
        Key::KeyY,
        Key::KeyZ,
        Key::Alt,
        Key::AltGr,
        Key::ControlLeft,
        Key::ControlRight,
        Key::MetaLeft,
        Key::MetaRight,
        Key::ShiftLeft,
        Key::ShiftRight,
        Key::Num1,
        Key::Num2,
        Key::Num3,
        Key::Num4,
        Key::Num5,
        Key::Num6,
        Key::Num7,
        Key::Num8,
        Key::Num9,
        Key::Num0,
        Key::F1,
        Key::F2,
        Key::F3,
        Key::F4,
        Key::F5,
        Key::F6,
        Key::F7,
        Key::F8,
        Key::F9,
        Key::F10,
        Key::F11,
        Key::F12,
        Key::CapsLock,
        Key::Backspace,
        Key::Delete,
        Key::End,
        Key::Escape,
        Key::Home,
        Key::PageDown,
        Key::PageUp,
        Key::Return,
        Key::Insert,
        Key::Slash,
        Key::BackSlash,
        Key::Quote,
        Key::SemiColon,
        Key::Comma,
        Key::Dot,
        Key::LeftBracket,
        Key::RightBracket,
        Key::PrintScreen,
        Key::ScrollLock,
        Key::Pause,
        Key::NumLock,
        Key::BackQuote,
    ];
}

pub fn show_bytes(bs: &[u8]) -> String {
    let mut visible = String::new();
    for &b in bs {
        let part: Vec<u8> = escape_default(b).collect();
        visible.push_str(std::str::from_utf8(&part).unwrap());
    }
    visible
}
