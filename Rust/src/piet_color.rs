use phf::phf_map;
use std::fmt;
use image::Rgb;

pub const COLORS: [[&str; 6]; 3] = [
    ["❤", "🧡", "💛", "💚", "💙", "💜"],
    ["🔴", "🟠", "🟡", "🟢", "🔵", "🟣"],
    ["🟥", "🟧", "🟨", "🟩", "🟦", "🟪"],
];

pub const REV_MAP: phf::Map<&str, (usize, usize)> = phf_map! {
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

impl From<ValidColor> for Rgb<u8> {
    fn from(rgb: ValidColor) -> Self {
        match rgb {
            ValidColor::White => Rgb([255, 255, 255]),
            ValidColor::Black => Rgb([0, 0, 0]),

            ValidColor::VeryPaleRed => Rgb([255, 192, 192]),
            ValidColor::VeryPaleYellow => Rgb([255, 255, 192]),
            ValidColor::VeryPaleGreen => Rgb([192, 255, 192]),

            ValidColor::VeryPaleCyan => Rgb([192, 255, 255]),
            ValidColor::VeryPaleBlue => Rgb([192, 192, 255]),
            ValidColor::VeryPaleMagenta => Rgb([255, 192, 255]),

            ValidColor::Red => Rgb([255, 0, 0]),
            ValidColor::Yellow => Rgb([255, 255, 0]),
            ValidColor::Green => Rgb([0, 255, 0]),

            ValidColor::Cyan => Rgb([0, 255, 255]),
            ValidColor::Blue => Rgb([0, 0, 255]),
            ValidColor::Magenta => Rgb([255, 0, 255]),

            ValidColor::StrongRed => Rgb([192, 0, 0]),
            ValidColor::StrongYellow => Rgb([192, 192, 0]),
            ValidColor::StrongGreen => Rgb([0, 192, 0]),

            ValidColor::StrongCyan => Rgb([0, 192, 192]),
            ValidColor::StrongBlue => Rgb([0, 0, 192]),
            ValidColor::StrongMagenta => Rgb([192, 0, 192]),
        }.into()
    }
}

impl From<Rgb<u8>> for ValidColor {
    fn from(rgb: Rgb<u8>) -> Self {
        match rgb {
            Rgb([255, 192, 192]) => "❤",
            Rgb([255, 255, 192]) => "🧡",
            Rgb([192, 255, 192]) => "💛",
            Rgb([192, 255, 255]) => "💚",
            Rgb([192, 192, 255]) => "💙",
            Rgb([255, 192, 255]) => "💜",
            Rgb([255, 0, 0]) => "🔴",
            Rgb([255, 255, 0]) => "🟠",
            Rgb([0, 255, 0]) => "🟡",
            Rgb([0, 255, 255]) => "🟢",
            Rgb([0, 0, 255]) => "🔵",
            Rgb([255, 0, 255]) => "🟣",
            Rgb([192, 0, 0]) => "🟥",
            Rgb([192, 192, 0]) => "🟧",
            Rgb([0, 192, 0]) => "🟨",
            Rgb([0, 192, 192]) => "🟩",
            Rgb([0, 0, 192]) => "🟦",
            Rgb([192, 0, 192]) => "🟪",
            Rgb([0, 0, 0]) => "⚫",
            _ => "⚪", // Whitespace if not exact color
        }.into()
    }
}

impl fmt::Debug for ValidColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <ValidColor as Into<&str>>::into(*self))
    }
}
