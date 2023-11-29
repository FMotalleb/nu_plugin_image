use terminal_size::{terminal_size, Height, Width};

use crate::image_to_ansi::{lib::img_to_ansi, AnsiImageError, ColorChar, DisplayMode};

fn get_terminal_size() -> Result<(u16, u16), AnsiImageError> {
    match terminal_size() {
        Some((Width(w), Height(h))) => Ok((w, h)),
        None => Err(AnsiImageError::new("Failed to get terminal dimensions.")),
    }
}

fn get_display_mode(character: Option<String>, reverse: bool, blink: bool) -> u8 {
    // Set display mode flags.
    let mut mode: u8 = 0;

    if let Some(_) = character {
        mode = mode
            | match reverse {
                true => DisplayMode::REVERSE as u8,
                false => DisplayMode::NORMAL as u8,
            };
    } else {
        mode = mode | DisplayMode::REVERSE as u8;
    }

    mode = mode
        | match blink {
            true => DisplayMode::BLINK as u8,
            false => DisplayMode::NORMAL as u8,
        };

    mode
}

pub fn to_ansi_converter(
    file: &Vec<u8>,
    // verbose: bool,
    reverse_bg: bool,
    blinking: bool,
    out_width: u32,
    out_height: u32,
    character: Option<String>,
    font_w_px: u32,
    font_h_px: u32,
) -> Result<String, AnsiImageError> {
    let out_char = match character.clone() {
        Some(c) => c,
        None => String::from(ColorChar::DEFAULT_CHAR),
    };

    let result = get_terminal_size();

    if let Err(e) = result {
        return Err(e);
    }

    // let term_dims = result.unwrap();

    // debug!("Terminal is {} x {} characters.", term_dims.0, term_dims.1);

    // Get output width and height, in characters.
    // Default to console size.
    // let out_width = get_int_arg("x", &matches, term_dims.0);
    // let out_height = get_int_arg("y", &matches, term_dims.1);

    // debug!("Output {} x {} characters", out_width, out_height);

    // Get character size in terminal.
    // Defaults to 8x16 pixels.
    // let font_w_px = get_int_arg("font-width", &matches, 8);
    // let font_h_px = get_int_arg("font-height", &matches, 16);

    let ansi_mode = get_display_mode(character, reverse_bg, blinking);

    let img_str = match img_to_ansi(
        file,
        out_width,
        out_height.into(),
        font_w_px.into(),
        font_h_px.into(),
        &out_char,
        ansi_mode,
    ) {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

    Ok(img_str)
}
