use std::fmt::Debug;

use crate::warn;

use crate::ansi_to_image::color::{Color, ColorType};
type ColorOption = Option<[u8; 4]>;
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
    pub primary_foreground: [u8; 4],
    pub primary_background: [u8; 4],

    pub black: [u8; 4],
    pub red: [u8; 4],
    pub green: [u8; 4],
    pub yellow: [u8; 4],
    pub blue: [u8; 4],
    pub magenta: [u8; 4],
    pub cyan: [u8; 4],
    pub white: [u8; 4],

    pub bright_black: [u8; 4],
    pub bright_red: [u8; 4],
    pub bright_green: [u8; 4],
    pub bright_yellow: [u8; 4],
    pub bright_blue: [u8; 4],
    pub bright_magenta: [u8; 4],
    pub bright_cyan: [u8; 4],
    pub bright_white: [u8; 4],

    pub fixed: [[u8; 4]; 256],
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

    pub(super) fn get_color(&self, color: ColorType) -> [u8; 4] {
        let palette = self.palette();

        match color {
            ColorType::PrimaryForeground => palette.primary_foreground,
            ColorType::PrimaryBackground => palette.primary_background,
            ColorType::Rgb { field1: rgb } => [rgb.0, rgb.1, rgb.2, 255],
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
        primary_foreground: [229, 229, 229, 255],
        primary_background: [24, 24, 24, 255],

        black: [0, 0, 0, 255],
        red: [205, 49, 49, 255],
        green: [13, 188, 121, 255],
        yellow: [229, 229, 16, 255],
        blue: [36, 114, 200, 255],
        magenta: [188, 63, 188, 255],
        cyan: [17, 168, 205, 255],
        white: [229, 229, 229, 255],

        bright_black: [102, 102, 102, 255],
        bright_red: [241, 76, 76, 255],
        bright_green: [35, 209, 139, 255],
        bright_yellow: [245, 245, 67, 255],
        bright_blue: [59, 142, 234, 255],
        bright_magenta: [214, 112, 214, 255],
        bright_cyan: [41, 184, 219, 255],
        bright_white: [229, 229, 229, 255],

        fixed: fixed_colors(),
    }
}
fn palette_xterm() -> PaletteData {
    PaletteData {
        // primary_background: "0x161616".parse().unwrap()
        // primary_foreground: "0xf2f2f2".parse().unwrap()
        primary_foreground: [229, 229, 229, 255],
        primary_background: [0, 0, 0, 255],

        black: [0, 0, 0, 255],
        red: [205, 0, 0, 255],
        green: [0, 205, 0, 255],
        yellow: [205, 205, 0, 255],
        blue: [0, 0, 238, 255],
        magenta: [205, 0, 205, 255],
        cyan: [0, 205, 205, 255],
        white: [229, 229, 229, 255],

        bright_black: [127, 127, 127, 255],
        bright_red: [255, 0, 0, 255],
        bright_green: [0, 255, 0, 255],
        bright_yellow: [255, 255, 0, 255],
        bright_blue: [0, 0, 252, 255],
        bright_magenta: [255, 0, 255, 255],
        bright_cyan: [0, 255, 255, 255],
        bright_white: [255, 255, 255, 255],

        fixed: fixed_colors(),
    }
}
fn palette_eclipse() -> PaletteData {
    PaletteData {
        // primary_background: "0x161616".parse().unwrap()
        // primary_foreground: "0xf2f2f2".parse().unwrap()
        primary_foreground: [229, 229, 229, 255],
        primary_background: [0, 0, 0, 255],

        black: [0, 0, 0, 255],
        red: [205, 0, 0, 255],
        green: [0, 205, 0, 255],
        yellow: [205, 205, 0, 255],
        blue: [0, 0, 238, 255],
        magenta: [205, 0, 205, 255],
        cyan: [0, 205, 205, 255],
        white: [229, 229, 229, 255],

        bright_black: [0, 0, 0, 255],
        bright_red: [255, 0, 0, 255],
        bright_green: [0, 255, 0, 255],
        bright_yellow: [255, 255, 0, 255],
        bright_blue: [0, 0, 252, 255],
        bright_magenta: [255, 0, 255, 255],
        bright_cyan: [0, 255, 255, 255],
        bright_white: [255, 255, 255, 255],

        fixed: fixed_colors(),
    }
}
fn palette_ubuntu() -> PaletteData {
    PaletteData {
        // primary_background: "0x161616".parse().unwrap()
        // primary_foreground: "0xf2f2f2".parse().unwrap()
        primary_foreground: [204, 204, 204, 255],
        primary_background: [1, 1, 1, 255],

        black: [1, 1, 1, 255],
        red: [222, 56, 43, 255],
        green: [57, 181, 74, 255],
        yellow: [255, 199, 6, 255],
        blue: [0, 111, 184, 255],
        magenta: [118, 38, 113, 255],
        cyan: [44, 181, 233, 255],
        white: [204, 204, 204, 255],

        bright_black: [128, 128, 128, 255],
        bright_red: [255, 0, 0, 255],
        bright_green: [0, 255, 0, 255],
        bright_yellow: [255, 255, 0, 255],
        bright_blue: [0, 0, 255, 255],
        bright_magenta: [255, 0, 255, 255],
        bright_cyan: [0, 255, 255, 255],
        bright_white: [255, 255, 255, 255],

        fixed: fixed_colors(),
    }
}
fn palette_mirc() -> PaletteData {
    PaletteData {
        // primary_background: "0x161616".parse().unwrap()
        // primary_foreground: "0xf2f2f2".parse().unwrap()
        primary_foreground: [210, 210, 210, 255],
        primary_background: [0, 0, 0, 255],

        black: [0, 0, 0, 255],
        red: [127, 0, 0, 255],
        green: [0, 147, 0, 255],
        yellow: [252, 127, 0, 255],
        blue: [0, 0, 127, 255],
        magenta: [156, 0, 156, 255],
        cyan: [0, 147, 147, 255],
        white: [210, 210, 210, 255],

        bright_black: [127, 127, 127, 255],
        bright_red: [255, 0, 0, 255],
        bright_green: [0, 252, 0, 255],
        bright_yellow: [255, 255, 0, 255],
        bright_blue: [0, 0, 252, 255],
        bright_magenta: [255, 0, 255, 255],
        bright_cyan: [0, 255, 255, 255],
        bright_white: [255, 255, 255, 255],

        fixed: fixed_colors(),
    }
}
fn palette_putty() -> PaletteData {
    PaletteData {
        // primary_background: "0x161616".parse().unwrap()
        // primary_foreground: "0xf2f2f2".parse().unwrap()
        primary_foreground: [187, 187, 187, 255],
        primary_background: [0, 0, 0, 255],

        black: [0, 0, 0, 255],
        red: [187, 0, 0, 255],
        green: [0, 187, 0, 255],
        yellow: [187, 187, 0, 255],
        blue: [0, 0, 187, 255],
        magenta: [187, 0, 187, 255],
        cyan: [0, 187, 187, 255],
        white: [187, 187, 187, 255],

        bright_black: [85, 85, 85, 255],
        bright_red: [255, 85, 85, 255],
        bright_green: [85, 255, 85, 255],
        bright_yellow: [255, 255, 85, 255],
        bright_blue: [85, 85, 255, 255],
        bright_magenta: [255, 85, 255, 255],
        bright_cyan: [85, 255, 255, 255],
        bright_white: [255, 255, 255, 255],

        fixed: fixed_colors(),
    }
}
fn palette_terminal_app() -> PaletteData {
    PaletteData {
        // primary_background: "0x161616".parse().unwrap()
        // primary_foreground: "0xf2f2f2".parse().unwrap()
        primary_foreground: [203, 204, 205, 255],
        primary_background: [0, 0, 0, 255],

        black: [0, 0, 0, 255],
        red: [194, 54, 33, 255],
        green: [37, 188, 36, 255],
        yellow: [173, 173, 39, 255],
        blue: [73, 46, 225, 255],
        magenta: [211, 56, 211, 255],
        cyan: [51, 187, 200, 255],
        white: [203, 204, 205, 255],

        bright_black: [129, 131, 131, 255],
        bright_red: [252, 57, 31, 255],
        bright_green: [49, 231, 34, 255],
        bright_yellow: [234, 236, 35, 255],
        bright_blue: [88, 51, 255, 255],
        bright_magenta: [249, 53, 248, 255],
        bright_cyan: [20, 240, 240, 255],
        bright_white: [233, 235, 235, 255],

        fixed: fixed_colors(),
    }
}
fn palette_win_10() -> PaletteData {
    PaletteData {
        // primary_background: "0x161616".parse().unwrap()
        // primary_foreground: "0xf2f2f2".parse().unwrap()
        primary_foreground: [204, 204, 204, 255],
        primary_background: [12, 12, 12, 255],

        black: [12, 12, 12, 255],
        red: [197, 15, 31, 255],
        green: [19, 161, 14, 255],
        yellow: [193, 156, 0, 255],
        blue: [0, 55, 218, 255],
        magenta: [136, 23, 152, 255],
        cyan: [58, 150, 221, 255],
        white: [204, 204, 204, 255],

        bright_black: [118, 118, 118, 255],
        bright_red: [231, 72, 86, 255],
        bright_green: [22, 198, 12, 255],
        bright_yellow: [249, 241, 165, 255],
        bright_blue: [59, 120, 255, 255],
        bright_magenta: [180, 0, 158, 255],
        bright_cyan: [97, 214, 214, 255],
        bright_white: [242, 242, 242, 255],

        fixed: fixed_colors(),
    }
}
fn palette_win_xp() -> PaletteData {
    PaletteData {
        // primary_background: "0x161616".parse().unwrap()
        // primary_foreground: "0xf2f2f2".parse().unwrap()
        primary_foreground: [192, 192, 192, 255],
        primary_background: [0, 0, 0, 255],

        black: [0, 0, 0, 255],
        red: [128, 0, 0, 255],
        green: [0, 128, 0, 255],
        yellow: [128, 128, 0, 255],
        blue: [0, 0, 128, 255],
        magenta: [128, 0, 128, 255],
        cyan: [0, 128, 128, 255],
        white: [192, 192, 192, 255],

        bright_black: [128, 128, 128, 255],
        bright_red: [255, 0, 0, 255],
        bright_green: [0, 255, 0, 255],
        bright_yellow: [255, 255, 0, 255],
        bright_blue: [0, 0, 255, 255],
        bright_magenta: [255, 0, 255, 255],
        bright_cyan: [0, 255, 255, 255],
        bright_white: [255, 255, 255, 255],

        fixed: fixed_colors(),
    }
}
fn palette_win_power_shell() -> PaletteData {
    PaletteData {
        // primary_background: "0x161616".parse().unwrap()
        // primary_foreground: "0xf2f2f2".parse().unwrap()
        primary_foreground: [192, 192, 192, 255],
        primary_background: [1, 36, 86, 255],

        black: [0, 0, 0, 255],
        red: [128, 0, 0, 255],
        green: [0, 128, 0, 255],
        yellow: [238, 237, 240, 255],
        blue: [0, 0, 128, 255],
        magenta: [1, 36, 86, 255],
        cyan: [0, 128, 128, 255],
        white: [192, 192, 192, 255],

        bright_black: [128, 128, 128, 255],
        bright_red: [255, 0, 0, 255],
        bright_green: [0, 255, 0, 255],
        bright_yellow: [255, 255, 0, 255],
        bright_blue: [0, 0, 255, 255],
        bright_magenta: [255, 0, 255, 255],
        bright_cyan: [0, 255, 255, 255],
        bright_white: [255, 255, 255, 255],

        fixed: fixed_colors(),
    }
}

fn palette_test() -> PaletteData {
    PaletteData {
        // primary_background: "0x161616".parse().unwrap()
        // primary_foreground: "0xf2f2f2".parse().unwrap()
        primary_foreground: [0, 0, 0, 255],
        primary_background: [255, 255, 255, 255],

        black: [0, 0, 0, 255],
        red: [255, 0, 0, 255],
        green: [0, 255, 0, 255],
        yellow: [249, 168, 37, 255],
        blue: [0, 0, 255, 255],
        magenta: [168, 37, 191, 255],
        cyan: [0, 131, 143, 255],
        white: [255, 255, 255, 255],

        bright_black: [44, 44, 44, 255],
        bright_red: [198, 40, 40, 255],
        bright_green: [85, 139, 46, 255],
        bright_yellow: [249, 168, 37, 255],
        bright_blue: [21, 101, 193, 255],
        bright_magenta: [168, 37, 191, 255],
        bright_cyan: [0, 131, 143, 255],
        bright_white: [255, 255, 255, 255],

        fixed: fixed_colors(),
    }
}

fn fixed_colors() -> [[u8; 4]; 256] {
    [
        [0, 0, 0, 255],
        [128, 0, 0, 255],
        [0, 128, 0, 255],
        [128, 128, 0, 255],
        [0, 0, 128, 255],
        [128, 0, 128, 255],
        [0, 128, 128, 255],
        [192, 192, 192, 255],
        [128, 128, 128, 255],
        [255, 0, 0, 255],
        [0, 255, 0, 255],
        [255, 255, 0, 255],
        [0, 0, 255, 255],
        [255, 0, 255, 255],
        [0, 255, 255, 255],
        [255, 255, 255, 255],
        [0, 0, 0, 255],
        [0, 0, 95, 255],
        [0, 0, 135, 255],
        [0, 0, 175, 255],
        [0, 0, 215, 255],
        [0, 0, 255, 255],
        [0, 95, 0, 255],
        [0, 95, 95, 255],
        [0, 95, 135, 255],
        [0, 95, 175, 255],
        [0, 95, 215, 255],
        [0, 95, 255, 255],
        [0, 135, 0, 255],
        [0, 135, 95, 255],
        [0, 135, 135, 255],
        [0, 135, 175, 255],
        [0, 135, 215, 255],
        [0, 135, 255, 255],
        [0, 175, 0, 255],
        [0, 175, 95, 255],
        [0, 175, 135, 255],
        [0, 175, 175, 255],
        [0, 175, 215, 255],
        [0, 175, 255, 255],
        [0, 215, 0, 255],
        [0, 215, 95, 255],
        [0, 215, 135, 255],
        [0, 215, 175, 255],
        [0, 215, 215, 255],
        [0, 215, 255, 255],
        [0, 255, 0, 255],
        [0, 255, 95, 255],
        [0, 255, 135, 255],
        [0, 255, 175, 255],
        [0, 255, 215, 255],
        [0, 255, 255, 255],
        [95, 0, 0, 255],
        [95, 0, 95, 255],
        [95, 0, 135, 255],
        [95, 0, 175, 255],
        [95, 0, 215, 255],
        [95, 0, 255, 255],
        [95, 95, 0, 255],
        [95, 95, 95, 255],
        [95, 95, 135, 255],
        [95, 95, 175, 255],
        [95, 95, 215, 255],
        [95, 95, 255, 255],
        [95, 135, 0, 255],
        [95, 135, 95, 255],
        [95, 135, 135, 255],
        [95, 135, 175, 255],
        [95, 135, 215, 255],
        [95, 135, 255, 255],
        [95, 175, 0, 255],
        [95, 175, 95, 255],
        [95, 175, 135, 255],
        [95, 175, 175, 255],
        [95, 175, 215, 255],
        [95, 175, 255, 255],
        [95, 215, 0, 255],
        [95, 215, 95, 255],
        [95, 215, 135, 255],
        [95, 215, 175, 255],
        [95, 215, 215, 255],
        [95, 215, 255, 255],
        [95, 255, 0, 255],
        [95, 255, 95, 255],
        [95, 255, 135, 255],
        [95, 255, 175, 255],
        [95, 255, 215, 255],
        [95, 255, 255, 255],
        [135, 0, 0, 255],
        [135, 0, 95, 255],
        [135, 0, 135, 255],
        [135, 0, 175, 255],
        [135, 0, 215, 255],
        [135, 0, 255, 255],
        [135, 95, 0, 255],
        [135, 95, 95, 255],
        [135, 95, 135, 255],
        [135, 95, 175, 255],
        [135, 95, 215, 255],
        [135, 95, 255, 255],
        [135, 135, 0, 255],
        [135, 135, 95, 255],
        [135, 135, 135, 255],
        [135, 135, 175, 255],
        [135, 135, 215, 255],
        [135, 135, 255, 255],
        [135, 175, 0, 255],
        [135, 175, 95, 255],
        [135, 175, 135, 255],
        [135, 175, 175, 255],
        [135, 175, 215, 255],
        [135, 175, 255, 255],
        [135, 215, 0, 255],
        [135, 215, 95, 255],
        [135, 215, 135, 255],
        [135, 215, 175, 255],
        [135, 215, 215, 255],
        [135, 215, 255, 255],
        [135, 255, 0, 255],
        [135, 255, 95, 255],
        [135, 255, 135, 255],
        [135, 255, 175, 255],
        [135, 255, 215, 255],
        [135, 255, 255, 255],
        [175, 0, 0, 255],
        [175, 0, 95, 255],
        [175, 0, 135, 255],
        [175, 0, 175, 255],
        [175, 0, 215, 255],
        [175, 0, 255, 255],
        [175, 95, 0, 255],
        [175, 95, 95, 255],
        [175, 95, 135, 255],
        [175, 95, 175, 255],
        [175, 95, 215, 255],
        [175, 95, 255, 255],
        [175, 135, 0, 255],
        [175, 135, 95, 255],
        [175, 135, 135, 255],
        [175, 135, 175, 255],
        [175, 135, 215, 255],
        [175, 135, 255, 255],
        [175, 175, 0, 255],
        [175, 175, 95, 255],
        [175, 175, 135, 255],
        [175, 175, 175, 255],
        [175, 175, 215, 255],
        [175, 175, 255, 255],
        [175, 215, 0, 255],
        [175, 215, 95, 255],
        [175, 215, 135, 255],
        [175, 215, 175, 255],
        [175, 215, 215, 255],
        [175, 215, 255, 255],
        [175, 255, 0, 255],
        [175, 255, 95, 255],
        [175, 255, 135, 255],
        [175, 255, 175, 255],
        [175, 255, 215, 255],
        [175, 255, 255, 255],
        [215, 0, 0, 255],
        [215, 0, 95, 255],
        [215, 0, 135, 255],
        [215, 0, 175, 255],
        [215, 0, 215, 255],
        [215, 0, 255, 255],
        [215, 95, 0, 255],
        [215, 95, 95, 255],
        [215, 95, 135, 255],
        [215, 95, 175, 255],
        [215, 95, 215, 255],
        [215, 95, 255, 255],
        [215, 135, 0, 255],
        [215, 135, 95, 255],
        [215, 135, 135, 255],
        [215, 135, 175, 255],
        [215, 135, 215, 255],
        [215, 135, 255, 255],
        [215, 175, 0, 255],
        [215, 175, 95, 255],
        [215, 175, 135, 255],
        [215, 175, 175, 255],
        [215, 175, 215, 255],
        [215, 175, 255, 255],
        [215, 215, 0, 255],
        [215, 215, 95, 255],
        [215, 215, 135, 255],
        [215, 215, 175, 255],
        [215, 215, 215, 255],
        [215, 215, 255, 255],
        [215, 255, 0, 255],
        [215, 255, 95, 255],
        [215, 255, 135, 255],
        [215, 255, 175, 255],
        [215, 255, 215, 255],
        [215, 255, 255, 255],
        [255, 0, 0, 255],
        [255, 0, 95, 255],
        [255, 0, 135, 255],
        [255, 0, 175, 255],
        [255, 0, 215, 255],
        [255, 0, 255, 255],
        [255, 95, 0, 255],
        [255, 95, 95, 255],
        [255, 95, 135, 255],
        [255, 95, 175, 255],
        [255, 95, 215, 255],
        [255, 95, 255, 255],
        [255, 135, 0, 255],
        [255, 135, 95, 255],
        [255, 135, 135, 255],
        [255, 135, 175, 255],
        [255, 135, 215, 255],
        [255, 135, 255, 255],
        [255, 175, 0, 255],
        [255, 175, 95, 255],
        [255, 175, 135, 255],
        [255, 175, 175, 255],
        [255, 175, 215, 255],
        [255, 175, 255, 255],
        [255, 215, 0, 255],
        [255, 215, 95, 255],
        [255, 215, 135, 255],
        [255, 215, 175, 255],
        [255, 215, 215, 255],
        [255, 215, 255, 255],
        [255, 255, 0, 255],
        [255, 255, 95, 255],
        [255, 255, 135, 255],
        [255, 255, 175, 255],
        [255, 255, 215, 255],
        [255, 255, 255, 255],
        [8, 8, 8, 255],
        [18, 18, 18, 255],
        [28, 28, 28, 255],
        [38, 38, 38, 255],
        [48, 48, 48, 255],
        [58, 58, 58, 255],
        [68, 68, 68, 255],
        [78, 78, 78, 255],
        [88, 88, 88, 255],
        [98, 98, 98, 255],
        [108, 108, 108, 255],
        [118, 118, 118, 255],
        [128, 128, 128, 255],
        [138, 138, 138, 255],
        [148, 148, 148, 255],
        [158, 158, 158, 255],
        [168, 168, 168, 255],
        [178, 178, 178, 255],
        [188, 188, 188, 255],
        [198, 198, 198, 255],
        [208, 208, 208, 255],
        [218, 218, 218, 255],
        [228, 228, 228, 255],
        [238, 238, 238, 255],
    ]
}

fn hex_from_env(var_name: &str) -> [u8; 4] {
    let val = std::env::var(var_name);
    match val {
        Ok(code) => match strhex_to_rgba(code) {
            Some(color) => color,
            None => {
                warn!("invalid hex value for env var {}", var_name);
                [0, 0, 0, 0]
            }
        },
        Err(err) => {
            warn!("cannot read env var {}, err: {}", var_name, err.to_string());
            [0, 0, 0, 0]
        }
    }
}
pub(crate) fn hex_to_rgba(hex: i64, alpha: Option<u8>) -> [u8; 4] {
    let r = ((hex >> 16) & 0xFF) as u8;
    let g = ((hex >> 8) & 0xFF) as u8;
    let b = (hex & 0xFF) as u8;
    let a = alpha.unwrap_or(255); // Default alpha to 255 (full) if not provided
    [r, g, b, a]
}

pub(crate) fn strhex_to_rgba(hex: String) -> Option<[u8; 4]> {
    let hex = hex.trim_start_matches("0x").trim_start_matches("#");

    let has_alpha = hex.len() == 8;

    if let Ok(hex) = i64::from_str_radix(&hex, 16) {
        let alpha = if has_alpha {
            Some((hex >> 24) as u8)
        } else {
            None
        };
        Some(hex_to_rgba(hex, alpha))
    } else {
        warn!("invalid hex {}", hex);
        None
    }
}
