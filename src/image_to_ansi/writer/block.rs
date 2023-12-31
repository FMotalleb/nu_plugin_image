use std::io::Error;

use crate::image_to_ansi::nu_plugin::IntoAnsiParams;

use ansi_colours::ansi256_from_rgb;
use image::{DynamicImage, GenericImageView, Rgba};
use termcolor::{Color, ColorSpec, WriteColor};

use crossterm::cursor::MoveRight;
use crossterm::execute;

const UPPER_HALF_BLOCK: &str = "\u{2580}";
const LOWER_HALF_BLOCK: &str = "\u{2584}";

// const CHECKERBOARD_BACKGROUND_LIGHT: (u8, u8, u8) = (153, 153, 153);
// const CHECKERBOARD_BACKGROUND_DARK: (u8, u8, u8) = (102, 102, 102);

pub fn make_ansi(
    stdout: &mut impl WriteColor,
    img: &DynamicImage,
    config: &IntoAnsiParams,
) -> Result<(), Error> {
    print_to_writecolor(stdout, img, config)
}

fn print_to_writecolor(
    stdout: &mut impl WriteColor,
    img: &DynamicImage,
    config: &IntoAnsiParams,
) -> Result<(), Error> {
    // adjust with x=0 and handle horizontal offset entirely below
    // adjust_offset(stdout, &Config { x: 0, ..*config })?;

    // resize the image so that it fits in the constraints, if any
    let img = super::resize(img, config.width, config.height);
    let (width, height) = img.dimensions();

    let mut row_color_buffer: Vec<ColorSpec> = vec![ColorSpec::new(); width as usize];
    let img_buffer = img.to_rgba8(); //TODO: Can conversion be avoided?

    for (curr_row, img_row) in img_buffer.enumerate_rows() {
        let is_even_row = curr_row % 2 == 0;
        let is_last_row = curr_row == height - 1;

        for pixel in img_row {
            // choose the half block's color
            let color = if is_pixel_transparent(pixel) {
                // TODO bg color
                // if config.transparent {
                None
                // } else {
                //     Some(get_transparency_color(curr_row, pixel.0, config.truecolor))
                // }
            } else {
                Some(get_color_from_pixel(pixel, config.truecolor))
            };

            // Even rows modify the background, odd rows the foreground
            // because lower half blocks are used by default
            let colorspec = &mut row_color_buffer[pixel.0 as usize];
            if is_even_row {
                colorspec.set_bg(color);
                if is_last_row {
                    write_colored_character(stdout, colorspec, true)?;
                }
            } else {
                colorspec.set_fg(color);
                write_colored_character(stdout, colorspec, false)?;
            }
        }

        if !is_even_row && !is_last_row {
            stdout.reset()?;
            writeln!(stdout, "\r")?;
        }
    }

    stdout.reset()?;
    writeln!(stdout)?;
    stdout.flush()?;
    Ok(())
}

fn write_colored_character(
    stdout: &mut impl WriteColor,
    c: &ColorSpec,
    is_last_row: bool,
) -> Result<(), Error> {
    let out_color;
    let out_char;
    let mut new_color;

    // On the last row use upper blocks and leave the bottom half empty (transparent)
    if is_last_row {
        new_color = ColorSpec::new();
        if let Some(bg) = c.bg() {
            new_color.set_fg(Some(*bg));
            out_char = UPPER_HALF_BLOCK;
        } else {
            execute!(stdout, MoveRight(1))?;
            return Ok(());
        }
        out_color = &new_color;
    } else {
        match (c.fg(), c.bg()) {
            (None, None) => {
                // completely transparent
                execute!(stdout, MoveRight(1))?;
                return Ok(());
            }
            (Some(bottom), None) => {
                // only top transparent
                new_color = ColorSpec::new();
                new_color.set_fg(Some(*bottom));
                out_color = &new_color;
                out_char = LOWER_HALF_BLOCK;
            }
            (None, Some(top)) => {
                // only bottom transparent
                new_color = ColorSpec::new();
                new_color.set_fg(Some(*top));
                out_color = &new_color;
                out_char = UPPER_HALF_BLOCK;
            }
            (Some(_top), Some(_bottom)) => {
                // both parts have a color
                out_color = c;
                out_char = LOWER_HALF_BLOCK;
            }
        }
    }
    stdout.set_color(out_color)?;
    write!(stdout, "{}", out_char)?;

    Ok(())
}

fn is_pixel_transparent(pixel: (u32, u32, &Rgba<u8>)) -> bool {
    pixel.2[3] == 0
}

// fn get_transparency_color(row: u32, col: u32, truecolor: bool) -> Color {
//     //imitate the transparent chess board pattern
//     let rgb = if row % 2 == col % 2 {
//         CHECKERBOARD_BACKGROUND_DARK
//     } else {
//         CHECKERBOARD_BACKGROUND_LIGHT
//     };
//     if truecolor {
//         Color::Rgb(rgb.0, rgb.1, rgb.2)
//     } else {
//         Color::Ansi256(ansi256_from_rgb(rgb))
//     }
// }

fn get_color_from_pixel(pixel: (u32, u32, &Rgba<u8>), truecolor: bool) -> Color {
    let (_x, _y, data) = pixel;
    let rgb = (data[0], data[1], data[2]);
    if truecolor {
        Color::Rgb(rgb.0, rgb.1, rgb.2)
    } else {
        Color::Ansi256(ansi256_from_rgb(rgb))
    }
}
