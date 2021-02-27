use core::fmt;
use rltk::{Rltk, VirtualKeyCode};
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::{collections::HashMap, fmt::Display};

#[derive(Hash, Eq, PartialEq, Serialize, Deserialize, Copy, Clone, Debug)]
pub enum InputModifier {
    Shift,
    Control,
}

impl Display for InputModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Shift => "Shift",
                Self::Control => "Control",
            }
        )
    }
}

#[derive(Hash, Eq, PartialEq, Serialize, Deserialize, Copy, Clone, Debug)]
pub enum Input {
    Key(VirtualKeyCode),
    LeftClick,
    RightClick,
}

impl Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Key(VirtualKeyCode::A) => "A",
                Self::Key(VirtualKeyCode::B) => "B",
                Self::Key(VirtualKeyCode::C) => "C",
                Self::Key(VirtualKeyCode::D) => "D",
                Self::Key(VirtualKeyCode::E) => "E",
                Self::Key(VirtualKeyCode::F) => "F",
                Self::Key(VirtualKeyCode::G) => "G",
                Self::Key(VirtualKeyCode::H) => "H",
                Self::Key(VirtualKeyCode::I) => "I",
                Self::Key(VirtualKeyCode::J) => "J",
                Self::Key(VirtualKeyCode::K) => "K",
                Self::Key(VirtualKeyCode::L) => "L",
                Self::Key(VirtualKeyCode::M) => "M",
                Self::Key(VirtualKeyCode::N) => "N",
                Self::Key(VirtualKeyCode::O) => "O",
                Self::Key(VirtualKeyCode::P) => "P",
                Self::Key(VirtualKeyCode::Q) => "Q",
                Self::Key(VirtualKeyCode::R) => "R",
                Self::Key(VirtualKeyCode::S) => "S",
                Self::Key(VirtualKeyCode::T) => "T",
                Self::Key(VirtualKeyCode::U) => "U",
                Self::Key(VirtualKeyCode::V) => "V",
                Self::Key(VirtualKeyCode::W) => "W",
                Self::Key(VirtualKeyCode::X) => "X",
                Self::Key(VirtualKeyCode::Y) => "Y",
                Self::Key(VirtualKeyCode::Z) => "Z",
                Self::Key(VirtualKeyCode::Key0) => "0",
                Self::Key(VirtualKeyCode::Key1) => "1",
                Self::Key(VirtualKeyCode::Key2) => "2",
                Self::Key(VirtualKeyCode::Key3) => "3",
                Self::Key(VirtualKeyCode::Key4) => "4",
                Self::Key(VirtualKeyCode::Key5) => "5",
                Self::Key(VirtualKeyCode::Key6) => "6",
                Self::Key(VirtualKeyCode::Key7) => "7",
                Self::Key(VirtualKeyCode::Key8) => "8",
                Self::Key(VirtualKeyCode::Key9) => "9",
                Self::Key(VirtualKeyCode::Escape) => "Escape",
                Self::Key(VirtualKeyCode::F1) => "F1",
                Self::Key(VirtualKeyCode::F2) => "F2",
                Self::Key(VirtualKeyCode::F3) => "F3",
                Self::Key(VirtualKeyCode::F4) => "F4",
                Self::Key(VirtualKeyCode::F5) => "F5",
                Self::Key(VirtualKeyCode::F6) => "F6",
                Self::Key(VirtualKeyCode::F7) => "F7",
                Self::Key(VirtualKeyCode::F8) => "F8",
                Self::Key(VirtualKeyCode::F9) => "F9",
                Self::Key(VirtualKeyCode::F10) => "F10",
                Self::Key(VirtualKeyCode::F11) => "F11",
                Self::Key(VirtualKeyCode::F12) => "F12",
                Self::Key(VirtualKeyCode::F13) => "F13",
                Self::Key(VirtualKeyCode::F14) => "F14",
                Self::Key(VirtualKeyCode::F15) => "F15",
                Self::Key(VirtualKeyCode::F16) => "F16",
                Self::Key(VirtualKeyCode::F17) => "F17",
                Self::Key(VirtualKeyCode::F18) => "F18",
                Self::Key(VirtualKeyCode::F19) => "F19",
                Self::Key(VirtualKeyCode::F20) => "F20",
                Self::Key(VirtualKeyCode::F21) => "F21",
                Self::Key(VirtualKeyCode::F22) => "F22",
                Self::Key(VirtualKeyCode::F23) => "F23",
                Self::Key(VirtualKeyCode::F24) => "F24",
                Self::Key(VirtualKeyCode::Snapshot) => "Snapshot",
                Self::Key(VirtualKeyCode::Scroll) => "Scroll",
                Self::Key(VirtualKeyCode::Pause) => "Pause",
                Self::Key(VirtualKeyCode::Insert) => "Insert",
                Self::Key(VirtualKeyCode::Home) => "Home",
                Self::Key(VirtualKeyCode::Delete) => "Delete",
                Self::Key(VirtualKeyCode::End) => "End",
                Self::Key(VirtualKeyCode::PageDown) => "PageDown",
                Self::Key(VirtualKeyCode::PageUp) => "PageUp",
                Self::Key(VirtualKeyCode::Left) => "Left",
                Self::Key(VirtualKeyCode::Up) => "Up",
                Self::Key(VirtualKeyCode::Right) => "Right",
                Self::Key(VirtualKeyCode::Down) => "Down",
                Self::Key(VirtualKeyCode::Back) => "Backspace",
                Self::Key(VirtualKeyCode::Return) => "Enter",
                Self::Key(VirtualKeyCode::Space) => "Space",
                Self::Key(VirtualKeyCode::Compose) => "Compose",
                Self::Key(VirtualKeyCode::Caret) => "Caret",
                Self::Key(VirtualKeyCode::Numlock) => "Numlock",
                Self::Key(VirtualKeyCode::Numpad0) => "Numpad0",
                Self::Key(VirtualKeyCode::Numpad1) => "Numpad1",
                Self::Key(VirtualKeyCode::Numpad2) => "Numpad2",
                Self::Key(VirtualKeyCode::Numpad3) => "Numpad3",
                Self::Key(VirtualKeyCode::Numpad4) => "Numpad4",
                Self::Key(VirtualKeyCode::Numpad5) => "Numpad5",
                Self::Key(VirtualKeyCode::Numpad6) => "Numpad6",
                Self::Key(VirtualKeyCode::Numpad7) => "Numpad7",
                Self::Key(VirtualKeyCode::Numpad8) => "Numpad8",
                Self::Key(VirtualKeyCode::Numpad9) => "Numpad9",
                Self::Key(VirtualKeyCode::AbntC1) => "AbntC1",
                Self::Key(VirtualKeyCode::AbntC2) => "AbntC2",
                Self::Key(VirtualKeyCode::Add) => "Add",
                Self::Key(VirtualKeyCode::Apostrophe) => "Apostrophe",
                Self::Key(VirtualKeyCode::Apps) => "Apps",
                Self::Key(VirtualKeyCode::At) => "At",
                Self::Key(VirtualKeyCode::Ax) => "Ax",
                Self::Key(VirtualKeyCode::Backslash) => "\\",
                Self::Key(VirtualKeyCode::Calculator) => "Calculator",
                Self::Key(VirtualKeyCode::Capital) => "Capitol",
                Self::Key(VirtualKeyCode::Colon) => "Colon",
                Self::Key(VirtualKeyCode::Comma) => "Comma",
                Self::Key(VirtualKeyCode::Convert) => "Convert",
                Self::Key(VirtualKeyCode::Decimal) => "Decimal",
                Self::Key(VirtualKeyCode::Divide) => "Divide",
                Self::Key(VirtualKeyCode::Equals) => "Equals",
                Self::Key(VirtualKeyCode::Grave) => "Grave",
                Self::Key(VirtualKeyCode::Kana) => "Kana",
                Self::Key(VirtualKeyCode::Kanji) => "Kanji",
                Self::Key(VirtualKeyCode::LAlt) => "Left Alt",
                Self::Key(VirtualKeyCode::LBracket) => "Left Bracket",
                Self::Key(VirtualKeyCode::LControl) => "Left Control",
                Self::Key(VirtualKeyCode::LShift) => "Left Shift",
                Self::Key(VirtualKeyCode::LWin) => "Left Windows",
                Self::Key(VirtualKeyCode::Mail) => "Mail",
                Self::Key(VirtualKeyCode::MediaSelect) => "Media Select",
                Self::Key(VirtualKeyCode::MediaStop) => "Media Stop",
                Self::Key(VirtualKeyCode::Minus) => "Minus",
                Self::Key(VirtualKeyCode::Multiply) => "Multiply",
                Self::Key(VirtualKeyCode::Mute) => "Mute",
                Self::Key(VirtualKeyCode::MyComputer) => "My Computer",
                Self::Key(VirtualKeyCode::NavigateForward) => "Navigate Forward",
                Self::Key(VirtualKeyCode::NavigateBackward) => "Navigate Backward",
                Self::Key(VirtualKeyCode::NextTrack) => "Next Track",
                Self::Key(VirtualKeyCode::NoConvert) => "No Convert",
                Self::Key(VirtualKeyCode::NumpadComma) => "Numpad Comma",
                Self::Key(VirtualKeyCode::NumpadEnter) => "Numpad Enter",
                Self::Key(VirtualKeyCode::NumpadEquals) => "Numpad Equals",
                Self::Key(VirtualKeyCode::OEM102) => "~",
                Self::Key(VirtualKeyCode::Period) => ".",
                Self::Key(VirtualKeyCode::PlayPause) => "Play/Pause",
                Self::Key(VirtualKeyCode::Power) => "Power",
                Self::Key(VirtualKeyCode::PrevTrack) => "Previous Track",
                Self::Key(VirtualKeyCode::RAlt) => "Right Alt",
                Self::Key(VirtualKeyCode::RBracket) => "Right Bracket",
                Self::Key(VirtualKeyCode::RControl) => "Right Control",
                Self::Key(VirtualKeyCode::RShift) => "Right Shift",
                Self::Key(VirtualKeyCode::RWin) => "Right Windows",
                Self::Key(VirtualKeyCode::Semicolon) => "Semicolon",
                Self::Key(VirtualKeyCode::Slash) => "/",
                Self::Key(VirtualKeyCode::Sleep) => "Sleep",
                Self::Key(VirtualKeyCode::Stop) => "Stop",
                Self::Key(VirtualKeyCode::Subtract) => "Subtract",
                Self::Key(VirtualKeyCode::Sysrq) => "SysReq",
                Self::Key(VirtualKeyCode::Tab) => "Tab",
                Self::Key(VirtualKeyCode::Underline) => "Underline",
                Self::Key(VirtualKeyCode::Unlabeled) => "Unlabeled",
                Self::Key(VirtualKeyCode::VolumeDown) => "Volume Down",
                Self::Key(VirtualKeyCode::VolumeUp) => "Volume Up",
                Self::Key(VirtualKeyCode::Wake) => "Wake",
                Self::Key(VirtualKeyCode::WebBack) => "Web Back",
                Self::Key(VirtualKeyCode::WebFavorites) => "Web Favorites",
                Self::Key(VirtualKeyCode::WebForward) => "Web Forward",
                Self::Key(VirtualKeyCode::WebHome) => "Web Home",
                Self::Key(VirtualKeyCode::WebRefresh) => "Web Refresh",
                Self::Key(VirtualKeyCode::WebSearch) => "Web Search",
                Self::Key(VirtualKeyCode::WebStop) => "Web Stop",
                Self::Key(VirtualKeyCode::Yen) => "Yen",
                Self::Key(VirtualKeyCode::Copy) => "Copy",
                Self::Key(VirtualKeyCode::Paste) => "Paste",
                Self::Key(VirtualKeyCode::Cut) => "Cut",
                Self::LeftClick => "Mouse Left",
                Self::RightClick => "Mouse Right",
            }
        )
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct Control {
    modifier: Option<InputModifier>,
    input: Input,
}

impl Control {
    fn modifier_from_context(context: &Rltk) -> Option<InputModifier> {
        if context.shift {
            Some(InputModifier::Shift)
        } else if context.control {
            Some(InputModifier::Control)
        } else {
            None
        }
    }
    fn input_from_context(context: &Rltk) -> Option<Input> {
        match context.key {
            Some(k) => Some(Input::Key(k)),
            None => {
                if context.left_click {
                    Some(Input::LeftClick)
                } else {
                    None
                }
            }
        }
    }
    pub fn from_context(context: &Rltk) -> Option<Self> {
        match Self::input_from_context(context) {
            Some(input) => Some(Self {
                input,
                modifier: Self::modifier_from_context(context),
            }),
            None => None,
        }
    }
}
impl Display for Control {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.modifier {
                Some(modifier) => {
                    format!("{} + {}", &modifier, self.input)
                }
                None => self.input.to_string(),
            }
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControlMap<T: Eq> {
    pub map: HashMap<Control, T>,
}

impl<T: Eq> ControlMap<T> {
    pub fn get_value_with_context(&self, context: &Rltk) -> Option<&T> {
        match Control::from_context(context) {
            Some(c) => self.map.get(&c),
            None => None,
        }
    }

    pub fn get_control_for_value(&self, value: &T) -> Option<&Control> {
        self.map.iter().find_map(|(k, v)| match v == value {
            true => Some(k),
            false => None,
        })
    }

    pub fn insert_with_context(&mut self, context: &Rltk, value: T) {
        match Control::from_context(context) {
            Some(c) => self.map.insert(c, value),
            None => None,
        };
    }

    pub fn remove(&mut self, control: &Control) {
        self.map.remove(control);
    }

    pub fn remove_by_value(&mut self, value: &T) {
        let control = {
            self.map.iter().find_map(|(k, v)| match v == value {
                true => Some(k.clone()),
                false => None,
            })
        };
        if let Some(control) = control {
            self.remove(&control);
        }
    }
}
