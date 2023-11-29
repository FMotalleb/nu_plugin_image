use image::RgbImage;
use include_flate::flate;
use rusttype::{Font, Scale};
use std::{
    io::{BufReader, Read},
    path::Path,
};
use vte::Parser;

use crate::ansi_to_image::{
    palette::Palette,
    printer::{self, Settings},
};

pub fn make_image(output_path: &Path, png_width: Option<u32>, input: &[u8]) {
    flate!(static FONT_MEDIUM: [u8] from
        "resources/IosevkaTerm/IosevkaTermNerdFontMono-Medium.ttf");
    flate!(static FONT_BOLD: [u8] from
        "resources/IosevkaTerm/IosevkaTermNerdFontMono-Bold.ttf");
    flate!(static FONT_ITALIC: [u8] from
        "resources/IosevkaTerm/IosevkaTermNerdFontMono-Italic.ttf");
    flate!(static FONT_ITALIC_BOLD: [u8] from
        "resources/IosevkaTerm/IosevkaTermNerdFontMono-BoldItalic.ttf");

    let font = Font::try_from_bytes(&FONT_MEDIUM).unwrap();
    let font_bold = Font::try_from_bytes(&FONT_BOLD).unwrap();
    let font_italic = Font::try_from_bytes(&FONT_ITALIC).unwrap();
    let font_italic_bold = Font::try_from_bytes(&FONT_ITALIC_BOLD).unwrap();

    let font_height = 50.0;
    let scale = Scale {
        x: font_height,
        y: font_height,
    };

    let palette = Palette::Custom;

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

            Ok(n) => {
                for byte in &buf[..n] {
                    state_machine.advance(&mut performer, *byte);
                }
            }

            Err(err) => {
                eprintln!("err: {err}");
                break;
            }
        }
    }

    let image: RgbImage = performer.into();

    image.save(output_path).unwrap();
}
