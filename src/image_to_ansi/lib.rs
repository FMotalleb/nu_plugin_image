extern crate image;

use image::GenericImageView;
// use log::debug; //{debug, info, trace, warn};

use std::error::Error;
use std::fmt;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Copy, Clone)]
pub enum DisplayMode {
    NORMAL = 0x00,
    REVERSE = 0x01,
    BLINK = 0x02,
}

#[derive(Debug)]
pub struct ColorChar {
    red: u8,
    green: u8,
    blue: u8,
    mode: u8,
}

#[derive(Debug)]
pub struct AnsiImageError {
    message: String,
}

impl AnsiImageError {
    pub fn new(msg: &str) -> AnsiImageError {
        AnsiImageError {
            message: msg.to_string(),
        }
    }

    pub fn from_string(err: &dyn Error) -> AnsiImageError {
        AnsiImageError {
            message: format!("{}", err),
        }
    }
}

impl fmt::Display for AnsiImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AnsiImageError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ColorChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut mode_flags = vec![];

        // Set ANSI flags
        match self.mode & DisplayMode::BLINK as u8 {
            0 => (),
            _ => mode_flags.push(ColorChar::ANSI_BLINK),
        };

        match self.mode & DisplayMode::REVERSE as u8 {
            0 => mode_flags.push(ColorChar::ANSI_FG_COLOR),
            _ => mode_flags.push(ColorChar::ANSI_BG_COLOR),
        }

        let mode_str = mode_flags.join(";");

        // ANSI mode followed by color spec
        write!(
            f,
            "\u{1b}[{};2;{};{};{}m",
            mode_str, self.red, self.green, self.blue
        )
    }
}

impl ColorChar {
    // End of line by OS
    #[cfg(windows)]
    const EOL: &'static str = "\r\n";
    #[cfg(not(windows))]
    const EOL: &'static str = "\n";

    // ANSI codes to include in output
    const ANSI_BLINK: &'static str = "5";
    const ANSI_FG_COLOR: &'static str = "38";
    const ANSI_BG_COLOR: &'static str = "48";

    // Reset console to defaults
    const ANSI_RESET: &'static str = "\u{1b}[25;39;49m";

    // Char to print when user does not supply one
    pub const DEFAULT_CHAR: &'static str = " ";

    pub fn new(r: u8, g: u8, b: u8, mode: u8) -> Self {
        ColorChar {
            red: r,
            green: g,
            blue: b,
            mode: mode,
        }
    }
}

pub fn get_char_for_area(
    img: &image::DynamicImage,
    x_pos: u32,
    y_pos: u32,
    width: u32,
    height: u32,
    mode: u8,
) -> ColorChar {
    let mut r: u32 = 0;
    let mut g: u32 = 0;
    let mut b: u32 = 0;

    for x in x_pos..(x_pos + width) {
        for y in y_pos..(y_pos + height) {
            let px = img.get_pixel(x, y);
            r += px[0] as u32;
            g += px[1] as u32;
            b += px[2] as u32;
        }
    }

    let sample_count = width * height;

    r = r / sample_count;
    g = g / sample_count;
    b = b / sample_count;

    if r > 255 || g > 255 || b > 255 {
        panic!("RGB values out of range.");
    }

    ColorChar::new(r as u8, g as u8, b as u8, mode)
}

pub fn img_to_ansi(
    image_buffer: &[u8],
    cons_w: u32,
    cons_h: u32,
    font_w_px: u32,
    font_h_px: u32,
    out_char: &String,
    mode: u8,
) -> Result<String, AnsiImageError> {
    // Sanity checking
    if cons_w == 0 {
        return Err(AnsiImageError::new("Console width is zero"));
    }

    if cons_h == 0 {
        return Err(AnsiImageError::new("Console height is zero"));
    }

    if font_w_px == 0 {
        return Err(AnsiImageError::new("Font pixel width is zero"));
    }

    if font_h_px == 0 {
        return Err(AnsiImageError::new("Font pixel height is zero"));
    }

    if out_char.graphemes(true).count() > 1 {
        return Err(AnsiImageError::new("Ouput character must be length one"));
    }

    // if image_file.len() == 0 {
    //     return Err(AnsiImageError::new("Empty filename."));
    // }

    let image = match image::load_from_memory(image_buffer) {
        Ok(i) => i,
        Err(e) => return Err(AnsiImageError::from_string(&e)), //.to_string()))
    };

    let img_dims = image.dimensions();

    // debug!("Image dims: {} x {}", img_dims.0, img_dims.1);
    // debug!("Output: {} x {} characters", cons_w, cons_h);

    // Size of console output, in pixels
    let cons_w_px = cons_w * font_w_px;
    let cons_h_px = cons_h * font_h_px;

    let ratio_w = cons_w_px as f64 / img_dims.0 as f64;
    let ratio_h = cons_h_px as f64 / img_dims.1 as f64;

    // debug!("Ratios {} {}", ratio_w, ratio_h);
    let mut targ_w;
    let mut targ_h;

    // Fit image to console size
    // Case: image is smaller than console
    if ratio_w > 1.0 && ratio_h > 1.0 {
        targ_w = img_dims.0;
        targ_h = img_dims.1;
    } else {
        // Case: Image is larger than console
        if ratio_h > ratio_w {
            targ_w = cons_w_px;
            targ_h = (img_dims.1 as f64 * ratio_w).floor() as u32;
        } else {
            targ_h = cons_h_px;
            targ_w = (img_dims.0 as f64 * ratio_h).floor() as u32;
        }
    }

    // Adjust target width and height to multiples
    // of console character sizes
    targ_w = targ_w - (targ_w % font_w_px);
    targ_h = targ_h - (targ_h % font_h_px);

    // Target size, in characters
    let targ_w_ch = targ_w / font_w_px;
    let targ_h_ch = targ_h / font_h_px;

    // debug!("Target Size: {} {} characters", targ_w_ch, targ_h_ch);

    // Derive the bounds of the sample area.
    // Drive from long edge of the image.
    let sample_w = img_dims.0 / targ_w_ch;
    let sample_h = img_dims.1 / targ_h_ch;

    // debug!("Sample size: {} x {}", sample_w, sample_h);

    let num_chunks_x = img_dims.0 / sample_w;
    let num_chunks_y = img_dims.1 / sample_h;

    // debug!("Horiz Chunks: {}", num_chunks_x);
    // debug!("Vert Chunks: {}", num_chunks_y);

    let mut out_str = String::from("");

    for chunk_y in 0..num_chunks_y {
        let y_pos = chunk_y * sample_h;

        for chunk_x in 0..num_chunks_x {
            let x_pos = chunk_x * sample_w;

            // Get average color for area
            let ch = get_char_for_area(&image, x_pos, y_pos, sample_w, sample_h, mode);

            // Format with out_char
            out_str = format!("{}{}{}", out_str, ch, out_char);
        }

        // Set reset string at end of line.
        out_str = format!("{}{}{}", out_str, ColorChar::ANSI_RESET, ColorChar::EOL);
    }

    Ok(out_str)
}
