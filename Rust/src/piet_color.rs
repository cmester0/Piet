use phf::phf_map;

use std::fmt;

// const COLORS: [[&str; 6]; 3] = [
//     ["❤", "🧡", "💛", "💚", "💙", "💜"],
//     ["🔴", "🟠", "🟡", "🟢", "🔵", "🟣"],
//     ["🟥", "🟧", "🟨", "🟩", "🟦", "🟪"],
// ];

pub const REV_MAP: phf::Map<&str, (u8, u8)> = phf_map! {
    "❤" => (0,0),
    "🔴" => (0,1),
    "🟥" => (0,2),
    "🧡" => (1,0),
    "🟠" => (1,1),
    "🟧" => (1,2),
    "💛" => (2,0),
    "🟡" => (2,1),
    "🟨" => (2,2),
    "💚" => (3,0),
    "🟢" => (3,1),
    "🟩" => (3,2),
    "💙" => (4,0),
    "🔵" => (4,1),
    "🟦" => (4,2),
    "💜" => (5,0),
    "🟣" => (5,1),
    "🟪" => (5,2)
};

// Color name guide: https://www.colorhexa.com/
#[derive(Copy, Clone, Eq, Hash, PartialEq)]
#[repr(usize)]
pub enum ValidColor {
    White,           // "⚪",
    Black,           // "⚫",
    VeryPaleRed,     // "❤",
    VeryPaleYellow,  // "🧡",
    VeryPaleGreen,   // "💛",
    VeryPaleCyan,    // "💚",
    VeryPaleBlue,    // "💙",
    VeryPaleMagenta, // "💜",
    Red,             // "🔴",
    Yellow,          // "🟠",
    Green,           // "🟡",
    Cyan,            // "🟢",
    Blue,            // "🔵",
    Magenta,         // "🟣",
    StrongRed,       // "🟥",
    StrongYellow,    // "🟧",
    StrongGreen,     // "🟨",
    StrongCyan,      // "🟩",
    StrongBlue,      // "🟦",
    StrongMagenta,   // "🟪",
}

pub const ALL_COLORS: [ValidColor; 20] = [
    White,
    Black,
    VeryPaleRed,
    VeryPaleYellow,
    VeryPaleGreen,
    VeryPaleCyan,
    VeryPaleBlue,
    VeryPaleMagenta,
    Red,
    Yellow,
    Green,
    Cyan,
    Blue,
    Magenta,
    StrongRed,
    StrongYellow,
    StrongGreen,
    StrongCyan,
    StrongBlue,
    StrongMagenta,
];

use ValidColor::*;

impl<'a> Into<&'a str> for ValidColor {
    fn into(self) -> &'a str {
        match self {
            Black => "⚫",
            VeryPaleRed => "❤",
            VeryPaleYellow => "🧡",
            VeryPaleGreen => "💛",
            VeryPaleCyan => "💚",
            VeryPaleBlue => "💙",
            VeryPaleMagenta => "💜",
            Red => "🔴",
            Yellow => "🟠",
            Green => "🟡",
            Cyan => "🟢",
            Blue => "🔵",
            Magenta => "🟣",
            StrongRed => "🟥",
            StrongYellow => "🟧",
            StrongGreen => "🟨",
            StrongCyan => "🟩",
            StrongBlue => "🟦",
            StrongMagenta => "🟪",
            White => "⚪",
        }
    }
}

impl From<&str> for ValidColor {
    fn from(s: &str) -> Self {
        match s {
            "⚫" => Black,
            "❤" => VeryPaleRed,
            "🧡" => VeryPaleYellow,
            "💛" => VeryPaleGreen,
            "💚" => VeryPaleCyan,
            "💙" => VeryPaleBlue,
            "💜" => VeryPaleMagenta,
            "🔴" => Red,
            "🟠" => Yellow,
            "🟡" => Green,
            "🟢" => Cyan,
            "🔵" => Blue,
            "🟣" => Magenta,
            "🟥" => StrongRed,
            "🟧" => StrongYellow,
            "🟨" => StrongGreen,
            "🟩" => StrongCyan,
            "🟦" => StrongBlue,
            "🟪" => StrongMagenta,
            "⚪" | _ => White,
        }
    }
}

impl fmt::Debug for ValidColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <ValidColor as Into<&str>>::into(*self))
    }
}
