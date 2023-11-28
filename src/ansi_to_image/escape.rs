//! From https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters

use vte::Params;

use crate::ansi_to_image::color::{Color, ColorType};

#[derive(Debug)]
pub(super) enum EscapeSequence {
    Reset,

    BlackletterFont,
    Bold,
    Faint,
    Italic,
    RapidBlink,
    SlowBlink,
    Underline,

    NotBold,
    NotUnderline,
    NormalItensity,
    NotItalicNorBlackletter,
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
}

impl EscapeSequence {
    pub(super) fn parse_params(params: &Params) -> Vec<EscapeSequence> {
        let params_slice = params.iter().flatten().collect::<Vec<_>>();

        match params_slice.as_slice() {
            [first] => vec![Self::parse_single(first)],

            [first, second] => vec![Self::parse_single(first), Self::parse_single(second)],

            // Set foreground (38) or background (48) color
            [fg_or_bg, 5, n] => {
                let color = match n {
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

                    // TODO: Implemente rgb colors https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit
                    16..=255 => ColorType::Rgb((0, 0, 0)),

                    _ => return vec![Self::Unimplemented(vec![**fg_or_bg, 5, **n])],
                };

                match fg_or_bg {
                    // foreground
                    38 => vec![Self::ForegroundColor(color)],

                    // background
                    48 => vec![Self::BackgroundColor(color)],

                    _ => vec![Self::Unimplemented(vec![**fg_or_bg, 5, **n])],
                }
            }

            v => vec![Self::Unimplemented(v.iter().map(|v| **v).collect())],
        }
    }

    fn parse_single(value: &&u16) -> Self {
        match value {
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

            20 => Self::BlackletterFont,
            21 => Self::NotBold,
            22 => Self::NormalItensity,
            23 => Self::NotItalicNorBlackletter,
            24 => Self::NotUnderline,
            25 => Self::NotBlinking,

            27 => Self::NotReserved,

            30 => Self::ForegroundColor(ColorType::Normal(Color::Black)),
            31 => Self::ForegroundColor(ColorType::Normal(Color::Red)),
            32 => Self::ForegroundColor(ColorType::Normal(Color::Green)),
            33 => Self::ForegroundColor(ColorType::Normal(Color::Yellow)),
            34 => Self::ForegroundColor(ColorType::Normal(Color::Blue)),
            35 => Self::ForegroundColor(ColorType::Normal(Color::Magenta)),
            36 => Self::ForegroundColor(ColorType::Normal(Color::Cyan)),
            37 => Self::ForegroundColor(ColorType::Normal(Color::White)),

            39 => Self::DefaultForegroundColor,

            40 => Self::BackgroundColor(ColorType::Normal(Color::Black)),
            41 => Self::BackgroundColor(ColorType::Normal(Color::Red)),
            42 => Self::BackgroundColor(ColorType::Normal(Color::Green)),
            43 => Self::BackgroundColor(ColorType::Normal(Color::Yellow)),
            44 => Self::BackgroundColor(ColorType::Normal(Color::Blue)),
            45 => Self::BackgroundColor(ColorType::Normal(Color::Magenta)),
            46 => Self::BackgroundColor(ColorType::Normal(Color::Cyan)),
            47 => Self::BackgroundColor(ColorType::Normal(Color::White)),

            49 => Self::DefaultBackgroundColor,
            50 => Self::DisableProportionalSpacing,

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

            v => Self::Unimplemented(vec![**v]),
        }
    }
}
