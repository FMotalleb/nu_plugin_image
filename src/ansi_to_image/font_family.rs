use std::fmt;
use std::fmt::{Display, Formatter};

use include_flate::flate;
use rusttype::Font;

#[derive(Debug)]
pub struct FontFamily<'a> {
    pub name: String,
    pub regular: Font<'a>,
    pub bold: Font<'a>,
    pub italic: Font<'a>,
    pub bold_italic: Font<'a>,
}

impl FontFamily<'_> {
    pub fn list() -> Vec<String> {
        vec![]
    }
    pub fn from_name(name: String) -> Self {
        return Self::default();
    }
    pub fn try_from_bytes(
        name: Option<String>,
        regular: &'static [u8],
        bold: &'static [u8],
        italic: &'static [u8],
        bold_italic: &'static [u8],
    ) -> Option<FontFamily<'static>> {
        let regular = Font::try_from_bytes(regular);
        let bold = Font::try_from_bytes(bold);
        let italic = Font::try_from_bytes(italic);
        let bold_italic = Font::try_from_bytes(bold_italic);
        match (regular, bold, italic, bold_italic) {
            (Some(regular), Some(bold), Some(italic), Some(bold_italic)) => {
                return Some(FontFamily {
                    name: name.unwrap_or("Custom".to_string()),
                    regular,
                    bold,
                    italic,
                    bold_italic,
                })
            }
            _ => None,
        }
    }
    pub fn ubuntu() -> Self {
        flate!(static REGULAR: [u8] from
            "resources/fonts/Ubuntu/Regular.ttf");
        flate!(static BOLD:   [u8] from
            "resources/fonts/Ubuntu/Bold.ttf");
        flate!(static ITALIC:  [u8] from
            "resources/fonts/Ubuntu/Italic.ttf");
        flate!(static BOLD_ITALIC:  [u8] from
            "resources/fonts/Ubuntu/BoldItalic.ttf");
        FontFamily::try_from_bytes(
            Some("Ubunto".to_string()),
            &REGULAR,
            &BOLD,
            &ITALIC,
            &BOLD_ITALIC,
        )
        .unwrap()
    }
    pub fn source_code_pro() -> Self {
        flate!(static REGULAR: [u8] from
            "resources/fonts/SourceCodePro/Regular.otf");
        flate!(static BOLD:   [u8] from
            "resources/fonts/SourceCodePro/Bold.otf");
        flate!(static ITALIC:  [u8] from
            "resources/fonts/SourceCodePro/Italic.otf");
        flate!(static BOLD_ITALIC:  [u8] from
            "resources/fonts/SourceCodePro/BoldItalic.otf");
        FontFamily::try_from_bytes(
            Some("SourceCodePro".to_string()),
            &REGULAR,
            &BOLD,
            &ITALIC,
            &BOLD_ITALIC,
        )
        .unwrap()
    }
    pub fn iosevka_term() -> Self {
        flate!(static REGULAR: [u8] from
            "resources/fonts/IosevkaTerm/Medium.ttf");
        flate!(static BOLD:   [u8] from
            "resources/fonts/IosevkaTerm/Bold.ttf");
        flate!(static ITALIC:  [u8] from
            "resources/fonts/IosevkaTerm/Italic.ttf");
        flate!(static BOLD_ITALIC:  [u8] from
            "resources/fonts/IosevkaTerm/BoldItalic.ttf");
        FontFamily::try_from_bytes(
            Some("IosevkaTerm".to_string()),
            &REGULAR,
            &BOLD,
            &ITALIC,
            &BOLD_ITALIC,
        )
        .unwrap()
    }
    pub fn anonymous_pro() -> Self {
        flate!(static REGULAR: [u8] from
            "resources/fonts/Anonymous_Pro/Regular.ttf");
        flate!(static BOLD:   [u8] from
            "resources/fonts/Anonymous_Pro/Bold.ttf");
        flate!(static ITALIC:  [u8] from
            "resources/fonts/Anonymous_Pro/Italic.ttf");
        flate!(static BOLD_ITALIC:  [u8] from
            "resources/fonts/Anonymous_Pro/BoldItalic.ttf");
        FontFamily::try_from_bytes(
            Some("AnonymousPro".to_string()),
            &REGULAR,
            &BOLD,
            &ITALIC,
            &BOLD_ITALIC,
        )
        .unwrap()
    }
}

impl<'a> Default for FontFamily<'a> {
    fn default() -> Self {
        Self::ubuntu()
    }
}

impl Display for FontFamily<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
