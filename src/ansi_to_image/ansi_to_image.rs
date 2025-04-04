use image::RgbaImage;
use std::{
    io::{BufReader, Read},
    path::Path,
};
use vte::Parser;

use crate::{
    ansi_to_image::{
        font_family::FontFamily,
        palette::Palette,
        printer::{self, Settings},
    },
    warn,
};

use super::internal_scale::InternalScale;

pub fn make_image(
    output_path: &Path,
    font_family: FontFamily,
    png_width: Option<u32>,
    input: &[u8],
    palette: Palette,
) {
    // let  = FontFamily::default();

    let font = font_family.regular;
    let font_bold = font_family.bold;
    let font_italic = font_family.italic;
    let font_italic_bold = font_family.bold_italic;

    let font_height = 50.0;
    let scale = InternalScale {
        x: font_height,
        y: font_height,
    };

    // let palette = Palette::Vscode;

    let mut state_machine = Parser::new();
    let mut performer = printer::new(Settings {
        font,
        font_bold,
        font_italic,
        font_italic_bold,
        font_height,
        scale,
        palette,
        png_width,
    });
    let reader = &mut BufReader::new(input);
    let mut buf = [0; 2048];

    loop {
        match reader.read(&mut buf) {
            Ok(0) => break,

            Ok(n) => state_machine.advance(&mut performer, &buf[..n]),

            Err(err) => {
                warn!("{err}");
                break;
            }
        }
    }

    let image: RgbaImage = performer.into();

    image.save(output_path).unwrap();
}
