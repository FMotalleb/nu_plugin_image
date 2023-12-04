//! From https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters

use std::slice::Iter;

use crate::ansi_to_image::color::{Color, ColorType};

#[derive(Debug)]
pub(super) enum EscapeSequence {
    Reset,

    BlackLetterFont,
    Bold,
    Faint,
    Italic,
    RapidBlink,
    SlowBlink,
    Underline,

    NotBold,
    NotUnderline,
    NormalIntensity,
    NotItalicNorBlackLetter,
    NotBlinking,

    ReverseVideo,
    Conceal,
    CrossedOut,

    DefaultForegroundColor,
    DefaultBackgroundColor,

    PrimaryFont,
    SetAlternativeFont,

    ForegroundColor(ColorType),
    BackgroundColor(ColorType),

    DisableProportionalSpacing,
    NeitherSuperscriptNorSubscript,

    NotReserved,

    Unimplemented(Vec<u16>),
    Ignore,
}

impl EscapeSequence {
    pub(super) fn parse_params(params: Vec<&u16>) -> Vec<EscapeSequence> {
        let iter = &mut params.iter();
        let mut result = vec![];
        while iter.len() > 0 {
            result.push(Self::consume_and_parse(iter))
        }
        result
    }
    fn consume_and_parse(iter: &mut Iter<&u16>) -> Self {
        if let Some(current) = iter.next() {
            return match *current {
                0 => Self::Reset,
                1 => Self::Bold,
                2 => Self::Faint,
                3 => Self::Italic,
                4 => Self::Underline,
                5 => Self::SlowBlink,
                6 => Self::RapidBlink,

                7 => Self::ReverseVideo,
                8 => Self::Conceal,
                9 => Self::CrossedOut,

                10 => Self::PrimaryFont,

                11 => Self::SetAlternativeFont,
                12 => Self::SetAlternativeFont,
                13 => Self::SetAlternativeFont,
                14 => Self::SetAlternativeFont,
                15 => Self::SetAlternativeFont,
                16 => Self::SetAlternativeFont,
                17 => Self::SetAlternativeFont,
                18 => Self::SetAlternativeFont,
                19 => Self::SetAlternativeFont,

                20 => Self::BlackLetterFont,
                21 => Self::NotBold,
                22 => Self::NormalIntensity,
                23 => Self::NotItalicNorBlackLetter,
                24 => Self::NotUnderline,
                25 => Self::NotBlinking,

                26 => Self::Ignore, // Proportional spacing

                27 => Self::NotReserved,
                28 => Self::Ignore, // Reveal
                29 => Self::Ignore, // Not crossed out

                30 => Self::ForegroundColor(ColorType::Normal(Color::Black)),
                31 => Self::ForegroundColor(ColorType::Normal(Color::Red)),
                32 => Self::ForegroundColor(ColorType::Normal(Color::Green)),
                33 => Self::ForegroundColor(ColorType::Normal(Color::Yellow)),
                34 => Self::ForegroundColor(ColorType::Normal(Color::Blue)),
                35 => Self::ForegroundColor(ColorType::Normal(Color::Magenta)),
                36 => Self::ForegroundColor(ColorType::Normal(Color::Cyan)),
                37 => Self::ForegroundColor(ColorType::Normal(Color::White)),
                38 => match iter.next() {
                    Some(mode) => Self::ForegroundColor(parse_color(mode, iter)),
                    None => {
                        eprintln!(
                            "[SEQUENCE_PARSER] foreground color mode is not supplied, parse_color(null, ...)",
                        );
                        Self::Ignore
                    }
                },
                39 => Self::DefaultForegroundColor,

                40 => Self::BackgroundColor(ColorType::Normal(Color::Black)),
                41 => Self::BackgroundColor(ColorType::Normal(Color::Red)),
                42 => Self::BackgroundColor(ColorType::Normal(Color::Green)),
                43 => Self::BackgroundColor(ColorType::Normal(Color::Yellow)),
                44 => Self::BackgroundColor(ColorType::Normal(Color::Blue)),
                45 => Self::BackgroundColor(ColorType::Normal(Color::Magenta)),
                46 => Self::BackgroundColor(ColorType::Normal(Color::Cyan)),
                47 => Self::BackgroundColor(ColorType::Normal(Color::White)),
                48 => match iter.next() {
                    Some(mode) => Self::BackgroundColor(parse_color(mode, iter)),
                    None => {
                        eprintln!(
                            "[SEQUENCE_PARSER] background color mode is not supplied, parse_color(null, ...)",
                        );
                        Self::Ignore
                    }
                },
                49 => Self::DefaultBackgroundColor,
                50 => Self::DisableProportionalSpacing,
                53 => Self::CrossedOut,

                75 => Self::NeitherSuperscriptNorSubscript,

                90 => Self::BackgroundColor(ColorType::Bright(Color::Black)),
                91 => Self::BackgroundColor(ColorType::Bright(Color::Red)),
                92 => Self::BackgroundColor(ColorType::Bright(Color::Green)),
                93 => Self::BackgroundColor(ColorType::Bright(Color::Yellow)),
                94 => Self::BackgroundColor(ColorType::Bright(Color::Blue)),
                95 => Self::BackgroundColor(ColorType::Bright(Color::Magenta)),
                96 => Self::BackgroundColor(ColorType::Bright(Color::Cyan)),
                97 => Self::BackgroundColor(ColorType::Bright(Color::White)),

                100 => Self::BackgroundColor(ColorType::Bright(Color::Black)),
                101 => Self::BackgroundColor(ColorType::Bright(Color::Red)),
                102 => Self::BackgroundColor(ColorType::Bright(Color::Green)),
                103 => Self::BackgroundColor(ColorType::Bright(Color::Yellow)),
                104 => Self::BackgroundColor(ColorType::Bright(Color::Blue)),
                105 => Self::BackgroundColor(ColorType::Bright(Color::Magenta)),
                106 => Self::BackgroundColor(ColorType::Bright(Color::Cyan)),
                107 => Self::BackgroundColor(ColorType::Bright(Color::White)),

                v => Self::Unimplemented(vec![3, *v]),
            };
        }
        Self::Ignore
    }
}

fn parse_color(mode: &u16, iter: &mut Iter<&u16>) -> ColorType {
    match mode {
        5 => {
            let color = iter.next();
            if let Some(color) = color {
                let color = match color {
                    0 => ColorType::Normal(Color::Black),
                    1 => ColorType::Normal(Color::Red),
                    2 => ColorType::Normal(Color::Green),
                    3 => ColorType::Normal(Color::Yellow),
                    4 => ColorType::Normal(Color::Blue),
                    5 => ColorType::Normal(Color::Magenta),
                    6 => ColorType::Normal(Color::Cyan),
                    7 => ColorType::Normal(Color::White),

                    8 => ColorType::Bright(Color::Black),
                    9 => ColorType::Bright(Color::Red),
                    10 => ColorType::Bright(Color::Green),
                    11 => ColorType::Bright(Color::Yellow),
                    12 => ColorType::Bright(Color::Blue),
                    13 => ColorType::Bright(Color::Magenta),
                    14 => ColorType::Bright(Color::Cyan),
                    15 => ColorType::Bright(Color::White),

                    // These are fixed colors and could be used like ansi 38;5;numberm or 48;5;numberm
                    16..=255 => ColorType::Fixed(**color as u8),

                    v => {
                        eprintln!("[COLOR_PARSER] fixed color value out of range, parse_fixed_color(code: {})",v);
                        return ColorType::PrimaryForeground;
                    }
                };
                return color;
            } else {
                eprintln!(
                    "[COLOR_PARSER] fixed color value not supplied, parse_fixed_color(code: null)"
                );
                return ColorType::PrimaryForeground;
            }
        }
        2 => match (iter.next(), iter.next(), iter.next()) {
            (Some(r), Some(g), Some(b)) => {
                let color = ColorType::Rgb {
                    field1: (**r as u8, **g as u8, **b as u8),
                };
                return color;
            }
            (r, g, b) => {
                eprintln!("[COLOR_PARSER] rgb color value not supplied (correctly), parse_rgb_color({}, {}, {})",
                r.map(|i| i.to_string() ).unwrap_or("null".to_string()),
                g.map(|i| i.to_string() ).unwrap_or("null".to_string()),
                b.map(|i| i.to_string() ).unwrap_or("null".to_string()));
                return ColorType::PrimaryForeground;
            }
        },
        v => {
            eprintln!(
                "[COLOR_PARSER] color mode is not supplied correctly, parse_color({}, ...)",
                v
            );
            return ColorType::PrimaryForeground;
        }
    }
}
