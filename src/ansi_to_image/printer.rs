use image::{Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::collections::BTreeMap;
use vte::{Params, Perform};

use crate::ansi_to_image::{color::ColorType, escape::EscapeSequence, palette::Palette};

pub(super) struct Settings<'a> {
    pub(super) font: Font<'a>,
    pub(super) font_bold: Font<'a>,
    pub(super) font_italic: Font<'a>,
    pub(super) font_italic_bold: Font<'a>,
    pub(super) font_height: f32,
    pub(super) scale: Scale,
    pub(super) palette: Palette,
    pub(super) png_width: Option<u32>,
}

#[derive(Debug, Default)]
struct SettingsInternal {
    glyph_advance_width: f32,
    new_line_distance: u32,
    png_width: Option<u32>,
}

#[derive(Debug)]
struct TextEntry {
    character: char,
    foreground_color: ColorType,
    background_color: ColorType,
    font: FontState,
    underline: bool,
}

#[derive(Debug, Clone, Copy)]
enum FontState {
    Normal,
    Bold,
    Italic,
    ItalicBold,
}

#[derive(Debug)]
struct State {
    text: BTreeMap<(u32, u32), TextEntry>,
    current_x: u32,
    current_y: u32,
    foreground_color: ColorType,
    background_color: ColorType,
    font: FontState,
    last_execute_byte: Option<u8>,
    underline: bool,
}

pub(super) struct Printer<'a> {
    settings: Settings<'a>,
    settings_internal: SettingsInternal,
    state: State,
}

pub(super) fn new(settings: Settings) -> Printer {
    let glyph_advance_width = settings
        .font
        .glyph('_')
        .scaled(settings.scale)
        .h_metrics()
        .advance_width;

    let new_line_distance = settings.font_height as u32;

    let png_width = settings.png_width;

    let settings_internal = SettingsInternal {
        glyph_advance_width,
        new_line_distance,
        png_width,
    };

    Printer {
        settings,
        settings_internal,
        state: State::default(),
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            text: BTreeMap::new(),
            current_x: 0,
            current_y: 0,
            foreground_color: ColorType::PrimaryForeground,
            background_color: ColorType::PrimaryBackground,
            font: FontState::Normal,
            last_execute_byte: None,
            underline: false,
        }
    }
}

impl<'a> Perform for Printer<'a> {
    fn print(&mut self, character: char) {
        self.state.text.insert(
            (self.state.current_x, self.state.current_y),
            TextEntry {
                character,
                foreground_color: self.state.foreground_color,
                background_color: self.state.background_color,
                font: self.state.font,
                underline: self.state.underline,
            },
        );

        self.state.current_x += self.settings_internal.glyph_advance_width as u32;

        if let Some(png_width) = self.settings_internal.png_width {
            if self.state.current_x > png_width {
                self.state.current_x = 0;
                self.state.current_y += self.settings_internal.new_line_distance;
            }
        }
    }

    fn execute(&mut self, byte: u8) {
        match byte {
            // ^M 	0x0D 	CR 	Carriage Return 	Moves the cursor to column zero.
            0x0d => {
                self.state.current_x = 0;
            }

            // ^J 	0x0A 	LF 	Line Feed 	Moves to next line, scrolls the display up if at bottom of the
            // screen. Usually does not move horizontally, though programs should not rely on this.
            0x0a => {
                self.state.current_x = 0;
                self.state.current_y += self.settings_internal.new_line_distance;
            }

            _ => eprintln!("[execute] {byte}, {byte:02x}"),
        }

        self.state.last_execute_byte = Some(byte)
    }

    fn hook(&mut self, params: &Params, intermediates: &[u8], ignore: bool, c: char) {
        eprintln!(
            "[hook] params={params:?}, intermediates={intermediates:?}, ignore={ignore:?}, \
             char={c:?}"
        );
    }

    fn put(&mut self, byte: u8) {
        eprintln!("[put] {byte:02x}");
    }

    fn unhook(&mut self) {
        eprintln!("[unhook]");
    }

    fn osc_dispatch(&mut self, params: &[&[u8]], bell_terminated: bool) {
        eprintln!("[osc_dispatch] params={params:?} bell_terminated={bell_terminated}2");
    }

    fn csi_dispatch(&mut self, params: &Params, intermediates: &[u8], ignore: bool, c: char) {
        eprintln!(
            "[csi_dispatch] params={params:?}, intermediates={intermediates:?}, ignore={ignore:?}, char={c:?}"
        );
        let actions = EscapeSequence::parse_params(params);

        for action in actions {
            match action {
                EscapeSequence::Reset => {
                    let defaults = State::default();

                    self.state.foreground_color = defaults.foreground_color;
                    self.state.background_color = defaults.background_color;
                    self.state.font = defaults.font;
                    self.state.underline = false;
                }

                EscapeSequence::Bold => self.state.font += FontState::Bold,
                EscapeSequence::Italic => self.state.font += FontState::Italic,
                EscapeSequence::Underline => self.state.underline = true,

                EscapeSequence::NotBold => self.state.font -= FontState::Bold,
                EscapeSequence::NotItalicNorBlackLetter => self.state.font -= FontState::Italic,
                EscapeSequence::NotUnderline => self.state.underline = false,

                EscapeSequence::ForegroundColor(color_type) => {
                    self.state.foreground_color = color_type
                }
                EscapeSequence::BackgroundColor(color_type) => {
                    self.state.background_color = color_type
                }

                EscapeSequence::DefaultForegroundColor => {
                    self.state.foreground_color = ColorType::PrimaryForeground
                }

                EscapeSequence::DefaultBackgroundColor => {
                    self.state.background_color = ColorType::PrimaryBackground
                }

                EscapeSequence::BlackLetterFont
                | EscapeSequence::Faint
                | EscapeSequence::SlowBlink
                | EscapeSequence::NotBlinking
                | EscapeSequence::ReverseVideo
                | EscapeSequence::Conceal
                | EscapeSequence::CrossedOut
                | EscapeSequence::PrimaryFont
                | EscapeSequence::SetAlternativeFont
                | EscapeSequence::DisableProportionalSpacing
                | EscapeSequence::NeitherSuperscriptNorSubscript
                | EscapeSequence::NotReserved
                | EscapeSequence::NormalIntensity
                | EscapeSequence::RapidBlink => {
                    eprintln!("not implemented for action: {action:?}")
                }
                EscapeSequence::Unimplemented(value) => {
                    eprintln!("not implemented for value: {value:?}")
                }
            }
        }
    }

    fn esc_dispatch(&mut self, _intermediates: &[u8], _ignore: bool, _byte: u8) {}
}

impl<'a> From<Printer<'a>> for RgbImage {
    fn from(printer: Printer) -> Self {
        let width = printer
            .state
            .text
            .keys()
            .map(|(x, _)| x)
            .max()
            .unwrap_or(&0)
            + printer.settings_internal.glyph_advance_width as u32;

        let height = printer
            .state
            .text
            .keys()
            .map(|(_, y)| y)
            .max()
            .unwrap_or(&0)
            + printer.settings_internal.new_line_distance;

        let mut image = RgbImage::new(width, height);

        // Set primary background
        for (_x, _y, pixel) in image.enumerate_pixels_mut() {
            *pixel = image::Rgb(
                printer
                    .settings
                    .palette
                    .get_color(ColorType::PrimaryBackground),
            );
        }

        // Render background before foreground from bottom to top to make it look better
        printer.state.text.iter().rev().for_each(|((x, y), entry)| {
            let background_end_x = x + printer.settings_internal.glyph_advance_width as u32;
            let background_end_y = y + printer.settings.font_height as u32;

            for x in *x..background_end_x {
                for y in *y..background_end_y {
                    let pixel =
                        image::Rgb(printer.settings.palette.get_color(entry.background_color));

                    image.put_pixel(x, y, pixel);
                }
            }
        });

        printer.state.text.iter().for_each(|((x, y), entry)| {
            let font = match entry.font {
                FontState::Normal => &printer.settings.font,
                FontState::Bold => &printer.settings.font_bold,
                FontState::Italic => &printer.settings.font_italic,
                FontState::ItalicBold => &printer.settings.font_italic_bold,
            };

            draw_text_mut(
                &mut image,
                Rgb(printer.settings.palette.get_color(entry.foreground_color)),
                (*x).try_into().unwrap(),
                (*y).try_into().unwrap(),
                printer.settings.scale,
                font,
                &entry.character.to_string(),
            );

            if entry.underline {
                // let underline_start = *x;
                // let underline_end = x + printer.settings_internal.glyph_advance_width as u32;
                // let underline_y = (y - 6) + printer.settings.font_height as u32;

                // for underline_x in underline_start..underline_end {
                //     let pixel =
                //         image::Rgb(printer.settings.palette.get_color(entry.foreground_color));

                //     image.put_pixel(underline_x, underline_y - 1, pixel);
                //     image.put_pixel(underline_x, underline_y, pixel);
                // }
            }
        });

        image
    }
}

impl std::ops::AddAssign for FontState {
    fn add_assign(&mut self, other: Self) {
        let new_self = match (&self, other) {
            (Self::Normal, Self::Normal) => Self::Normal,

            (Self::Bold, Self::Bold) | (Self::Bold, Self::Normal) | (Self::Normal, Self::Bold) => {
                Self::Bold
            }

            (Self::Italic, Self::Italic)
            | (Self::Italic, Self::Normal)
            | (Self::Normal, Self::Italic) => Self::Italic,

            (Self::Bold, Self::Italic)
            | (Self::Bold, Self::ItalicBold)
            | (Self::ItalicBold, Self::Bold)
            | (Self::ItalicBold, Self::Italic)
            | (Self::ItalicBold, Self::ItalicBold)
            | (Self::ItalicBold, Self::Normal)
            | (Self::Italic, Self::Bold)
            | (Self::Italic, Self::ItalicBold)
            | (Self::Normal, Self::ItalicBold) => Self::ItalicBold,
        };

        *self = new_self
    }
}

impl std::ops::SubAssign for FontState {
    fn sub_assign(&mut self, other: Self) {
        let new_self = match (&self, other) {
            (Self::Italic, Self::Italic)
            | (Self::ItalicBold, Self::ItalicBold)
            | (Self::Bold, Self::Bold)
            | (Self::Normal, Self::Normal)
            | (Self::Normal, Self::Bold)
            | (Self::Normal, Self::Italic)
            | (Self::Bold, Self::ItalicBold)
            | (Self::Italic, Self::ItalicBold)
            | (Self::Normal, Self::ItalicBold) => Self::Normal,

            (Self::Bold, Self::Normal)
            | (Self::Bold, Self::Italic)
            | (Self::ItalicBold, Self::Italic) => Self::Bold,

            (Self::Italic, Self::Normal)
            | (Self::Italic, Self::Bold)
            | (Self::ItalicBold, Self::Bold) => Self::Italic,

            (Self::ItalicBold, Self::Normal) => Self::ItalicBold,
        };

        *self = new_self
    }
}
