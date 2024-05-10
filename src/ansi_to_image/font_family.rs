use std::fmt;
use std::fmt::{Display, Formatter};

use ab_glyph::FontRef;
use include_flate::flate;
type FontBuilder = fn() -> FontFamily<'static>;
#[derive(Debug)]
pub struct FontFamily<'a> {
    pub name: String,
    pub regular: FontRef<'a>,
    pub bold: FontRef<'a>,
    pub italic: FontRef<'a>,
    pub bold_italic: FontRef<'a>,
}

impl FontFamily<'static> {
    fn all_fonts() -> Vec<(String, FontBuilder)> {
        let mut result: Vec<(String, FontBuilder)> = vec![];
        result.push(("SourceCodePro".to_string(), Self::source_code_pro));
        #[cfg(feature = "font-ubuntu")]
        result.push(("Ubuntu".to_string(), Self::ubuntu));
        #[cfg(feature = "font-iosevka_term")]
        result.push(("IosevkaTerm".to_string(), Self::iosevka_term));
        #[cfg(feature = "font-anonymous_pro")]
        result.push(("AnonymousPro".to_string(), Self::anonymous_pro));
        result
    }
    pub fn list() -> Vec<String> {
        Self::all_fonts().into_iter().map(|i| i.0).collect()
    }

    pub fn from_name(name: String) -> Self {
        for value in Self::all_fonts() {
            if name == value.0 {
                return value.1();
            }
        }
        return Self::default();
    }
    pub fn try_from_bytes(
        name: Option<String>,
        regular: &'static [u8],
        bold: &'static [u8],
        italic: &'static [u8],
        bold_italic: &'static [u8],
    ) -> Option<FontFamily<'static>> {
        let regular = FontRef::try_from_slice(regular);
        let bold = FontRef::try_from_slice(bold);
        let italic = FontRef::try_from_slice(italic);
        let bold_italic = FontRef::try_from_slice(bold_italic);
        match (regular, bold, italic, bold_italic) {
            (Ok(regular), Ok(bold), Ok(italic), Ok(bold_italic)) => {
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

    #[cfg(feature = "font-ubuntu")]
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

    #[cfg(feature = "font-iosevka_term")]
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

    #[cfg(feature = "font-anonymous_pro")]
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

impl Default for FontFamily<'static> {
    fn default() -> Self {
        Self::source_code_pro()
    }
}

impl Display for FontFamily<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
