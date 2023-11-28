use std::fmt::Debug;

use crate::ansi_to_image::color::{Color, ColorType};

#[allow(dead_code)]
#[derive(Debug)]
pub(super) enum Palette {
    Custom,
    Test,
}

#[derive(Debug)]
pub struct PaletteData {
    pub primary_foreground: [u8; 3],
    pub primary_background: [u8; 3],

    pub black: [u8; 3],
    pub red: [u8; 3],
    pub green: [u8; 3],
    pub yellow: [u8; 3],
    pub blue: [u8; 3],
    pub magenta: [u8; 3],
    pub cyan: [u8; 3],
    pub white: [u8; 3],

    pub bright_black: [u8; 3],
    pub bright_red: [u8; 3],
    pub bright_green: [u8; 3],
    pub bright_yellow: [u8; 3],
    pub bright_blue: [u8; 3],
    pub bright_magenta: [u8; 3],
    pub bright_cyan: [u8; 3],
    pub bright_white: [u8; 3],
}

impl Palette {
    fn pallete(&self) -> PaletteData {
        match self {
            Palette::Custom => pallete_custom(),
            Palette::Test => pallete_test(),
        }
    }

    pub fn get_color(&self, color: ColorType) -> [u8; 3] {
        let pallete = self.pallete();

        match color {
            ColorType::PrimaryForeground => pallete.primary_foreground,
            ColorType::PrimaryBackground => pallete.primary_background,
            ColorType::Rgb(rgb) => [rgb.0, rgb.1, rgb.2],

            ColorType::Normal(color) => match color {
                Color::Black => pallete.black,
                Color::Red => pallete.red,
                Color::Green => pallete.green,
                Color::Yellow => pallete.yellow,
                Color::Blue => pallete.blue,
                Color::Magenta => pallete.magenta,
                Color::Cyan => pallete.cyan,
                Color::White => pallete.white,
            },

            ColorType::Bright(color) => match color {
                Color::Black => pallete.bright_black,
                Color::Red => pallete.bright_red,
                Color::Green => pallete.bright_green,
                Color::Yellow => pallete.bright_yellow,
                Color::Blue => pallete.bright_blue,
                Color::Magenta => pallete.bright_magenta,
                Color::Cyan => pallete.bright_cyan,
                Color::White => pallete.bright_white,
            },
        }
    }
}

fn pallete_custom() -> PaletteData {
    PaletteData {
        // primary_background: "0x161616".parse().unwrap()
        // primary_foreground: "0xf2f2f2".parse().unwrap()
        primary_foreground: [242, 242, 242],
        primary_background: [22, 22, 22],

        black: [44, 44, 44],
        red: [198, 40, 40],
        green: [85, 139, 46],
        yellow: [249, 168, 37],
        blue: [21, 101, 193],
        magenta: [168, 37, 191],
        cyan: [0, 131, 143],
        white: [255, 255, 255],

        bright_black: [44, 44, 44],
        bright_red: [198, 40, 40],
        bright_green: [85, 139, 46],
        bright_yellow: [249, 168, 37],
        bright_blue: [21, 101, 193],
        bright_magenta: [168, 37, 191],
        bright_cyan: [0, 131, 143],
        bright_white: [255, 255, 255],
    }
}

fn pallete_test() -> PaletteData {
    PaletteData {
        // primary_background: "0x161616".parse().unwrap()
        // primary_foreground: "0xf2f2f2".parse().unwrap()
        primary_foreground: [0, 0, 0],
        primary_background: [255, 255, 255],

        black: [0, 0, 0],
        red: [255, 0, 0],
        green: [0, 255, 0],
        yellow: [249, 168, 37],
        blue: [0, 0, 255],
        magenta: [168, 37, 191],
        cyan: [0, 131, 143],
        white: [255, 255, 255],

        bright_black: [44, 44, 44],
        bright_red: [198, 40, 40],
        bright_green: [85, 139, 46],
        bright_yellow: [249, 168, 37],
        bright_blue: [21, 101, 193],
        bright_magenta: [168, 37, 191],
        bright_cyan: [0, 131, 143],
        bright_white: [255, 255, 255],
    }
}
