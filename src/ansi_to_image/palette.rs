use std::fmt::Debug;

use crate::ansi_to_image::color::{Color, ColorType};
type ColorOption = Option<[u8; 3]>;
#[allow(dead_code)]
#[derive(Debug)]
pub enum Palette {
    Vscode,
    Xterm,
    Eclipse,
    Ubuntu,
    MIRC,
    Putty,
    WinXp,
    WinTerminal,
    Win10,
    WinPs,
    Env,
    Custom(PaletteData),
    Test,
}

impl Default for Palette {
    fn default() -> Self {
        if let Ok(_) = std::env::var("NU_PLUGIN_IMAGE_FG") {
            return Palette::Env;
        }
        return Palette::Vscode;
    }
}

#[derive(Debug, Clone, Copy)]
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

    pub fixed: [[u8; 3]; 256],
}
impl PaletteData {
    pub fn copy_with(
        &self,
        primary_foreground: ColorOption,
        primary_background: ColorOption,

        black: ColorOption,
        red: ColorOption,
        green: ColorOption,
        yellow: ColorOption,
        blue: ColorOption,
        magenta: ColorOption,
        cyan: ColorOption,
        white: ColorOption,

        bright_black: ColorOption,
        bright_red: ColorOption,
        bright_green: ColorOption,
        bright_yellow: ColorOption,
        bright_blue: ColorOption,
        bright_magenta: ColorOption,
        bright_cyan: ColorOption,
        bright_white: ColorOption,
    ) -> PaletteData {
        let result = &mut self.clone();
        if let Some(fg) = primary_foreground {
            result.primary_foreground = fg;
        }

        if let Some(bg) = primary_background {
            result.primary_background = bg;
        }

        if let Some(color) = black {
            result.black = color;
        }

        if let Some(color) = red {
            result.red = color;
        }

        if let Some(color) = green {
            result.green = color;
        }

        if let Some(color) = yellow {
            result.yellow = color;
        }

        if let Some(color) = blue {
            result.blue = color;
        }

        if let Some(color) = magenta {
            result.magenta = color;
        }

        if let Some(color) = cyan {
            result.cyan = color;
        }

        if let Some(color) = white {
            result.white = color;
        }

        if let Some(color) = bright_black {
            result.bright_black = color;
        }

        if let Some(color) = bright_red {
            result.bright_red = color;
        }

        if let Some(color) = bright_green {
            result.bright_green = color;
        }

        if let Some(color) = bright_yellow {
            result.bright_yellow = color;
        }

        if let Some(color) = bright_blue {
            result.bright_blue = color;
        }

        if let Some(color) = bright_magenta {
            result.bright_magenta = color;
        }

        if let Some(color) = bright_cyan {
            result.bright_cyan = color;
        }

        if let Some(color) = bright_white {
            result.bright_white = color;
        }

        result.to_owned()
    }
}

impl Palette {
    pub(super) fn palette(&self) -> PaletteData {
        match self {
            Palette::Vscode => palette_vscode(),
            Palette::Xterm => palette_xterm(),
            Palette::Ubuntu => palette_ubuntu(),
            Palette::Eclipse => palette_eclipse(),
            Palette::MIRC => palette_mirc(),
            Palette::Putty => palette_putty(),
            Palette::WinXp => palette_win_xp(),
            Palette::WinTerminal => palette_terminal_app(),
            Palette::Win10 => palette_win_10(),
            Palette::WinPs => palette_win_power_shell(),
            Palette::Test => palette_test(),
            Palette::Env => palette_env(),
            Palette::Custom(p) => *p,
        }
    }
    pub(super) fn from_name(name: String) -> Option<Palette> {
        match name.to_lowercase().as_str() {
            "vscode" => Some(Palette::Vscode),
            "xterm" => Some(Palette::Xterm),
            "eclipse" => Some(Palette::Eclipse),
            "ubuntu" => Some(Palette::Ubuntu),
            "mirc" => Some(Palette::MIRC),
            "putty" => Some(Palette::Putty),
            "winxp" => Some(Palette::WinXp),
            "terminal" => Some(Palette::WinTerminal),
            "winterm" => Some(Palette::WinTerminal),
            "win10" => Some(Palette::Win10),
            "win_power-shell" => Some(Palette::WinPs),
            "win_ps" => Some(Palette::WinPs),
            _ => None,
        }
    }
    pub fn list() -> Vec<String> {
        vec![
            "vscode".to_string(),
            "xterm".to_string(),
            "ubuntu".to_string(),
            "eclipse".to_string(),
            "mirc".to_string(),
            "putty".to_string(),
            "winxp".to_string(),
            "terminal".to_string(),
            "win10".to_string(),
            "win_power-shell".to_string(),
            "win_ps".to_string(),
        ]
    }

    pub(super) fn get_color(&self, color: ColorType) -> [u8; 3] {
        let palette = self.palette();

        match color {
            ColorType::PrimaryForeground => palette.primary_foreground,
            ColorType::PrimaryBackground => palette.primary_background,
            ColorType::Rgb { field1: rgb } => [rgb.0, rgb.1, rgb.2],

            ColorType::Normal(color) => match color {
                Color::Black => palette.black,
                Color::Red => palette.red,
                Color::Green => palette.green,
                Color::Yellow => palette.yellow,
                Color::Blue => palette.blue,
                Color::Magenta => palette.magenta,
                Color::Cyan => palette.cyan,
                Color::White => palette.white,
            },

            ColorType::Bright(color) => match color {
                Color::Black => palette.bright_black,
                Color::Red => palette.bright_red,
                Color::Green => palette.bright_green,
                Color::Yellow => palette.bright_yellow,
                Color::Blue => palette.bright_blue,
                Color::Magenta => palette.bright_magenta,
                Color::Cyan => palette.bright_cyan,
                Color::White => palette.bright_white,
            },

            ColorType::Fixed(num) => palette.fixed[num as usize],
        }
    }
}
fn palette_env() -> PaletteData {
    PaletteData {
        primary_foreground: hex_from_env("NU_PLUGIN_IMAGE_FG"),
        primary_background: hex_from_env("NU_PLUGIN_IMAGE_BG"),

        black: hex_from_env("NU_PLUGIN_IMAGE_BLACK"),
        red: hex_from_env("NU_PLUGIN_IMAGE_RED"),
        green: hex_from_env("NU_PLUGIN_IMAGE_GREEN"),
        yellow: hex_from_env("NU_PLUGIN_IMAGE_YELLOW"),
        blue: hex_from_env("NU_PLUGIN_IMAGE_BLUE"),
        magenta: hex_from_env("NU_PLUGIN_IMAGE_MAGENTA"),
        cyan: hex_from_env("NU_PLUGIN_IMAGE_CYAN"),
        white: hex_from_env("NU_PLUGIN_IMAGE_WHITE"),

        bright_black: hex_from_env("NU_PLUGIN_IMAGE_BRIGHT_BLACK"),
        bright_red: hex_from_env("NU_PLUGIN_IMAGE_BRIGHT_RED"),
        bright_green: hex_from_env("NU_PLUGIN_IMAGE_BRIGHT_GREEN"),
        bright_yellow: hex_from_env("NU_PLUGIN_IMAGE_BRIGHT_YELLOW"),
        bright_blue: hex_from_env("NU_PLUGIN_IMAGE_BRIGHT_BLUE"),
        bright_magenta: hex_from_env("NU_PLUGIN_IMAGE_BRIGHT_MAGENTA"),
        bright_cyan: hex_from_env("NU_PLUGIN_IMAGE_BRIGHT_CYAN"),
        bright_white: hex_from_env("NU_PLUGIN_IMAGE_BRIGHT_WHITE"),

        fixed: fixed_colors(),
    }
}
fn palette_vscode() -> PaletteData {
    PaletteData {
        // primary_background: "0x161616".parse().unwrap()
        // primary_foreground: "0xf2f2f2".parse().unwrap()
        primary_foreground: [229, 229, 229],
        primary_background: [24, 24, 24],

        black: [0, 0, 0],
        red: [205, 49, 49],
        green: [13, 188, 121],
        yellow: [229, 229, 16],
        blue: [36, 114, 200],
        magenta: [188, 63, 188],
        cyan: [17, 168, 205],
        white: [229, 229, 229],

        bright_black: [102, 102, 102],
        bright_red: [241, 76, 76],
        bright_green: [35, 209, 139],
        bright_yellow: [245, 245, 67],
        bright_blue: [59, 142, 234],
        bright_magenta: [214, 112, 214],
        bright_cyan: [41, 184, 219],
        bright_white: [229, 229, 229],

        fixed: fixed_colors(),
    }
}
fn palette_xterm() -> PaletteData {
    PaletteData {
        // primary_background: "0x161616".parse().unwrap()
        // primary_foreground: "0xf2f2f2".parse().unwrap()
        primary_foreground: [229, 229, 229],
        primary_background: [0, 0, 0],

        black: [0, 0, 0],
        red: [205, 0, 0],
        green: [0, 205, 0],
        yellow: [205, 205, 0],
        blue: [0, 0, 238],
        magenta: [205, 0, 205],
        cyan: [0, 205, 205],
        white: [229, 229, 229],

        bright_black: [127, 127, 127],
        bright_red: [255, 0, 0],
        bright_green: [0, 255, 0],
        bright_yellow: [255, 255, 0],
        bright_blue: [0, 0, 252],
        bright_magenta: [255, 0, 255],
        bright_cyan: [0, 255, 255],
        bright_white: [255, 255, 255],

        fixed: fixed_colors(),
    }
}
fn palette_eclipse() -> PaletteData {
    PaletteData {
        // primary_background: "0x161616".parse().unwrap()
        // primary_foreground: "0xf2f2f2".parse().unwrap()
        primary_foreground: [229, 229, 229],
        primary_background: [0, 0, 0],

        black: [0, 0, 0],
        red: [205, 0, 0],
        green: [0, 205, 0],
        yellow: [205, 205, 0],
        blue: [0, 0, 238],
        magenta: [205, 0, 205],
        cyan: [0, 205, 205],
        white: [229, 229, 229],

        bright_black: [0, 0, 0],
        bright_red: [255, 0, 0],
        bright_green: [0, 255, 0],
        bright_yellow: [255, 255, 0],
        bright_blue: [0, 0, 252],
        bright_magenta: [255, 0, 255],
        bright_cyan: [0, 255, 255],
        bright_white: [255, 255, 255],

        fixed: fixed_colors(),
    }
}
fn palette_ubuntu() -> PaletteData {
    PaletteData {
        // primary_background: "0x161616".parse().unwrap()
        // primary_foreground: "0xf2f2f2".parse().unwrap()
        primary_foreground: [204, 204, 204],
        primary_background: [1, 1, 1],

        black: [1, 1, 1],
        red: [222, 56, 43],
        green: [57, 181, 74],
        yellow: [255, 199, 6],
        blue: [0, 111, 184],
        magenta: [118, 38, 113],
        cyan: [44, 181, 233],
        white: [204, 204, 204],

        bright_black: [128, 128, 128],
        bright_red: [255, 0, 0],
        bright_green: [0, 255, 0],
        bright_yellow: [255, 255, 0],
        bright_blue: [0, 0, 255],
        bright_magenta: [255, 0, 255],
        bright_cyan: [0, 255, 255],
        bright_white: [255, 255, 255],

        fixed: fixed_colors(),
    }
}
fn palette_mirc() -> PaletteData {
    PaletteData {
        // primary_background: "0x161616".parse().unwrap()
        // primary_foreground: "0xf2f2f2".parse().unwrap()
        primary_foreground: [210, 210, 210],
        primary_background: [0, 0, 0],

        black: [0, 0, 0],
        red: [127, 0, 0],
        green: [0, 147, 0],
        yellow: [252, 127, 0],
        blue: [0, 0, 127],
        magenta: [156, 0, 156],
        cyan: [0, 147, 147],
        white: [210, 210, 210],

        bright_black: [127, 127, 127],
        bright_red: [255, 0, 0],
        bright_green: [0, 252, 0],
        bright_yellow: [255, 255, 0],
        bright_blue: [0, 0, 252],
        bright_magenta: [255, 0, 255],
        bright_cyan: [0, 255, 255],
        bright_white: [255, 255, 255],

        fixed: fixed_colors(),
    }
}
fn palette_putty() -> PaletteData {
    PaletteData {
        // primary_background: "0x161616".parse().unwrap()
        // primary_foreground: "0xf2f2f2".parse().unwrap()
        primary_foreground: [187, 187, 187],
        primary_background: [0, 0, 0],

        black: [0, 0, 0],
        red: [187, 0, 0],
        green: [0, 187, 0],
        yellow: [187, 187, 0],
        blue: [0, 0, 187],
        magenta: [187, 0, 187],
        cyan: [0, 187, 187],
        white: [187, 187, 187],

        bright_black: [85, 85, 85],
        bright_red: [255, 85, 85],
        bright_green: [85, 255, 85],
        bright_yellow: [255, 255, 85],
        bright_blue: [85, 85, 255],
        bright_magenta: [255, 85, 255],
        bright_cyan: [85, 255, 255],
        bright_white: [255, 255, 255],

        fixed: fixed_colors(),
    }
}
fn palette_terminal_app() -> PaletteData {
    PaletteData {
        // primary_background: "0x161616".parse().unwrap()
        // primary_foreground: "0xf2f2f2".parse().unwrap()
        primary_foreground: [203, 204, 205],
        primary_background: [0, 0, 0],

        black: [0, 0, 0],
        red: [194, 54, 33],
        green: [37, 188, 36],
        yellow: [173, 173, 39],
        blue: [73, 46, 225],
        magenta: [211, 56, 211],
        cyan: [51, 187, 200],
        white: [203, 204, 205],

        bright_black: [129, 131, 131],
        bright_red: [252, 57, 31],
        bright_green: [49, 231, 34],
        bright_yellow: [234, 236, 35],
        bright_blue: [88, 51, 255],
        bright_magenta: [249, 53, 248],
        bright_cyan: [20, 240, 240],
        bright_white: [233, 235, 235],

        fixed: fixed_colors(),
    }
}
fn palette_win_10() -> PaletteData {
    PaletteData {
        // primary_background: "0x161616".parse().unwrap()
        // primary_foreground: "0xf2f2f2".parse().unwrap()
        primary_foreground: [204, 204, 204],
        primary_background: [12, 12, 12],

        black: [12, 12, 12],
        red: [197, 15, 31],
        green: [19, 161, 14],
        yellow: [193, 156, 0],
        blue: [0, 55, 218],
        magenta: [136, 23, 152],
        cyan: [58, 150, 221],
        white: [204, 204, 204],

        bright_black: [118, 118, 118],
        bright_red: [231, 72, 86],
        bright_green: [22, 198, 12],
        bright_yellow: [249, 241, 165],
        bright_blue: [59, 120, 255],
        bright_magenta: [180, 0, 158],
        bright_cyan: [97, 214, 214],
        bright_white: [242, 242, 242],

        fixed: fixed_colors(),
    }
}
fn palette_win_xp() -> PaletteData {
    PaletteData {
        // primary_background: "0x161616".parse().unwrap()
        // primary_foreground: "0xf2f2f2".parse().unwrap()
        primary_foreground: [192, 192, 192],
        primary_background: [0, 0, 0],

        black: [0, 0, 0],
        red: [128, 0, 0],
        green: [0, 128, 0],
        yellow: [128, 128, 0],
        blue: [0, 0, 128],
        magenta: [128, 0, 128],
        cyan: [0, 128, 128],
        white: [192, 192, 192],

        bright_black: [128, 128, 128],
        bright_red: [255, 0, 0],
        bright_green: [0, 255, 0],
        bright_yellow: [255, 255, 0],
        bright_blue: [0, 0, 255],
        bright_magenta: [255, 0, 255],
        bright_cyan: [0, 255, 255],
        bright_white: [255, 255, 255],

        fixed: fixed_colors(),
    }
}
fn palette_win_power_shell() -> PaletteData {
    PaletteData {
        // primary_background: "0x161616".parse().unwrap()
        // primary_foreground: "0xf2f2f2".parse().unwrap()
        primary_foreground: [192, 192, 192],
        primary_background: [1, 36, 86],

        black: [0, 0, 0],
        red: [128, 0, 0],
        green: [0, 128, 0],
        yellow: [238, 237, 240],
        blue: [0, 0, 128],
        magenta: [1, 36, 86],
        cyan: [0, 128, 128],
        white: [192, 192, 192],

        bright_black: [128, 128, 128],
        bright_red: [255, 0, 0],
        bright_green: [0, 255, 0],
        bright_yellow: [255, 255, 0],
        bright_blue: [0, 0, 255],
        bright_magenta: [255, 0, 255],
        bright_cyan: [0, 255, 255],
        bright_white: [255, 255, 255],

        fixed: fixed_colors(),
    }
}

fn palette_test() -> PaletteData {
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

        fixed: fixed_colors(),
    }
}

fn fixed_colors() -> [[u8; 3]; 256] {
    [
        [0, 0, 0],
        [128, 0, 0],
        [0, 128, 0],
        [128, 128, 0],
        [0, 0, 128],
        [128, 0, 128],
        [0, 128, 128],
        [192, 192, 192],
        [128, 128, 128],
        [255, 0, 0],
        [0, 255, 0],
        [255, 255, 0],
        [0, 0, 255],
        [255, 0, 255],
        [0, 255, 255],
        [255, 255, 255],
        [0, 0, 0],
        [0, 0, 95],
        [0, 0, 135],
        [0, 0, 175],
        [0, 0, 215],
        [0, 0, 255],
        [0, 95, 0],
        [0, 95, 95],
        [0, 95, 135],
        [0, 95, 175],
        [0, 95, 215],
        [0, 95, 255],
        [0, 135, 0],
        [0, 135, 95],
        [0, 135, 135],
        [0, 135, 175],
        [0, 135, 215],
        [0, 135, 255],
        [0, 175, 0],
        [0, 175, 95],
        [0, 175, 135],
        [0, 175, 175],
        [0, 175, 215],
        [0, 175, 255],
        [0, 215, 0],
        [0, 215, 95],
        [0, 215, 135],
        [0, 215, 175],
        [0, 215, 215],
        [0, 215, 255],
        [0, 255, 0],
        [0, 255, 95],
        [0, 255, 135],
        [0, 255, 175],
        [0, 255, 215],
        [0, 255, 255],
        [95, 0, 0],
        [95, 0, 95],
        [95, 0, 135],
        [95, 0, 175],
        [95, 0, 215],
        [95, 0, 255],
        [95, 95, 0],
        [95, 95, 95],
        [95, 95, 135],
        [95, 95, 175],
        [95, 95, 215],
        [95, 95, 255],
        [95, 135, 0],
        [95, 135, 95],
        [95, 135, 135],
        [95, 135, 175],
        [95, 135, 215],
        [95, 135, 255],
        [95, 175, 0],
        [95, 175, 95],
        [95, 175, 135],
        [95, 175, 175],
        [95, 175, 215],
        [95, 175, 255],
        [95, 215, 0],
        [95, 215, 95],
        [95, 215, 135],
        [95, 215, 175],
        [95, 215, 215],
        [95, 215, 255],
        [95, 255, 0],
        [95, 255, 95],
        [95, 255, 135],
        [95, 255, 175],
        [95, 255, 215],
        [95, 255, 255],
        [135, 0, 0],
        [135, 0, 95],
        [135, 0, 135],
        [135, 0, 175],
        [135, 0, 215],
        [135, 0, 255],
        [135, 95, 0],
        [135, 95, 95],
        [135, 95, 135],
        [135, 95, 175],
        [135, 95, 215],
        [135, 95, 255],
        [135, 135, 0],
        [135, 135, 95],
        [135, 135, 135],
        [135, 135, 175],
        [135, 135, 215],
        [135, 135, 255],
        [135, 175, 0],
        [135, 175, 95],
        [135, 175, 135],
        [135, 175, 175],
        [135, 175, 215],
        [135, 175, 255],
        [135, 215, 0],
        [135, 215, 95],
        [135, 215, 135],
        [135, 215, 175],
        [135, 215, 215],
        [135, 215, 255],
        [135, 255, 0],
        [135, 255, 95],
        [135, 255, 135],
        [135, 255, 175],
        [135, 255, 215],
        [135, 255, 255],
        [175, 0, 0],
        [175, 0, 95],
        [175, 0, 135],
        [175, 0, 175],
        [175, 0, 215],
        [175, 0, 255],
        [175, 95, 0],
        [175, 95, 95],
        [175, 95, 135],
        [175, 95, 175],
        [175, 95, 215],
        [175, 95, 255],
        [175, 135, 0],
        [175, 135, 95],
        [175, 135, 135],
        [175, 135, 175],
        [175, 135, 215],
        [175, 135, 255],
        [175, 175, 0],
        [175, 175, 95],
        [175, 175, 135],
        [175, 175, 175],
        [175, 175, 215],
        [175, 175, 255],
        [175, 215, 0],
        [175, 215, 95],
        [175, 215, 135],
        [175, 215, 175],
        [175, 215, 215],
        [175, 215, 255],
        [175, 255, 0],
        [175, 255, 95],
        [175, 255, 135],
        [175, 255, 175],
        [175, 255, 215],
        [175, 255, 255],
        [215, 0, 0],
        [215, 0, 95],
        [215, 0, 135],
        [215, 0, 175],
        [215, 0, 215],
        [215, 0, 255],
        [215, 95, 0],
        [215, 95, 95],
        [215, 95, 135],
        [215, 95, 175],
        [215, 95, 215],
        [215, 95, 255],
        [215, 135, 0],
        [215, 135, 95],
        [215, 135, 135],
        [215, 135, 175],
        [215, 135, 215],
        [215, 135, 255],
        [215, 175, 0],
        [215, 175, 95],
        [215, 175, 135],
        [215, 175, 175],
        [215, 175, 215],
        [215, 175, 255],
        [215, 215, 0],
        [215, 215, 95],
        [215, 215, 135],
        [215, 215, 175],
        [215, 215, 215],
        [215, 215, 255],
        [215, 255, 0],
        [215, 255, 95],
        [215, 255, 135],
        [215, 255, 175],
        [215, 255, 215],
        [215, 255, 255],
        [255, 0, 0],
        [255, 0, 95],
        [255, 0, 135],
        [255, 0, 175],
        [255, 0, 215],
        [255, 0, 255],
        [255, 95, 0],
        [255, 95, 95],
        [255, 95, 135],
        [255, 95, 175],
        [255, 95, 215],
        [255, 95, 255],
        [255, 135, 0],
        [255, 135, 95],
        [255, 135, 135],
        [255, 135, 175],
        [255, 135, 215],
        [255, 135, 255],
        [255, 175, 0],
        [255, 175, 95],
        [255, 175, 135],
        [255, 175, 175],
        [255, 175, 215],
        [255, 175, 255],
        [255, 215, 0],
        [255, 215, 95],
        [255, 215, 135],
        [255, 215, 175],
        [255, 215, 215],
        [255, 215, 255],
        [255, 255, 0],
        [255, 255, 95],
        [255, 255, 135],
        [255, 255, 175],
        [255, 255, 215],
        [255, 255, 255],
        [8, 8, 8],
        [18, 18, 18],
        [28, 28, 28],
        [38, 38, 38],
        [48, 48, 48],
        [58, 58, 58],
        [68, 68, 68],
        [78, 78, 78],
        [88, 88, 88],
        [98, 98, 98],
        [108, 108, 108],
        [118, 118, 118],
        [128, 128, 128],
        [138, 138, 138],
        [148, 148, 148],
        [158, 158, 158],
        [168, 168, 168],
        [178, 178, 178],
        [188, 188, 188],
        [198, 198, 198],
        [208, 208, 208],
        [218, 218, 218],
        [228, 228, 228],
        [238, 238, 238],
    ]
}

fn hex_from_env(var_name: &str) -> [u8; 3] {
    let val = std::env::var(var_name);
    match val {
        Ok(code) => match code.parse::<i64>() {
            Ok(val) => hex_to_rgb(val),
            Err(err) => {
                crate::vlog(format!(
                    "cannot parse env var {}, value: {}, err: {}",
                    var_name,
                    code,
                    err.to_string()
                ));
                [0, 0, 0]
            }
        },
        Err(err) => {
            crate::vlog(format!(
                "cannot read env var {}, err: {}",
                var_name,
                err.to_string()
            ));
            [0, 0, 0]
        }
    }
}

pub(crate) fn hex_to_rgb(hex: i64) -> [u8; 3] {
    let r = ((hex >> 16) & 0xFF) as u8;
    let g = ((hex >> 8) & 0xFF) as u8;
    let b = (hex & 0xFF) as u8;
    [r, g, b]
}
