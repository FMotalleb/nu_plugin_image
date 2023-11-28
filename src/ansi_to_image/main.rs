use clap::Parser as _;
use image::RgbImage;
use include_flate::flate;
use rusttype::{Font, Scale};
use std::io::Read;
use vte::Parser;

use crate::ansi_to_image::{
    opt::Opt,
    pallete::Palette,
    printer::{self, Settings},
};

pub fn ansi_to_png() {
    // TODO add https://github.com/AlexanderThaller/ansi2png-rs/tree/main/resources
    let opt = Opt::parse();

    let mut input = std::io::BufReader::new(std::fs::File::open(opt.input_path).unwrap());

    flate!(static FONT: [u8] from
        "resources/ttf-iosevka-term-7.2.6/iosevka-term-extended.ttf");

    flate!(static FONT_BOLD: [u8] from
        "resources/ttf-iosevka-term-7.2.6/iosevka-term-extendedbold.ttf");

    flate!(static FONT_ITALIC: [u8] from
        "resources/ttf-iosevka-term-7.2.6/iosevka-term-extendeditalic.ttf");

    flate!(static FONT_ITALIC_BOLD: [u8] from
        "resources/ttf-iosevka-term-7.2.6/iosevka-term-extendedbolditalic.ttf");

    let font = Font::try_from_bytes(&FONT).unwrap();
    let font_bold = Font::try_from_bytes(&FONT_BOLD).unwrap();
    let font_italic = Font::try_from_bytes(&FONT_ITALIC).unwrap();
    let font_italic_bold = Font::try_from_bytes(&FONT_ITALIC_BOLD).unwrap();

    let font_height = 50.0;
    let scale = Scale {
        x: font_height,
        y: font_height,
    };

    let pallete = Palette::Custom;
    let png_width = opt.png_width;

    let mut statemachine = Parser::new();
    let mut performer = printer::new(Settings {
        font,
        font_bold,
        font_italic,
        font_italic_bold,
        font_height,
        scale,
        pallete,
        png_width,
    });

    let mut buf = [0; 2048];

    loop {
        match input.read(&mut buf) {
            Ok(0) => break,

            Ok(n) => {
                for byte in &buf[..n] {
                    statemachine.advance(&mut performer, *byte);
                }
            }

            Err(err) => {
                println!("err: {err}");
                break;
            }
        }
    }

    let image: RgbImage = performer.into();
    image.save(opt.output_path).unwrap();
}
