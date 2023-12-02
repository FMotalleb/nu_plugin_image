use std::fmt::{self, Display, Formatter};

use include_flate::flate;

#[derive(Debug)]
pub enum FontFamily {
    #[cfg(feature = "font-iosevka_term")]
    IosevkaTerm,
    #[cfg(feature = "font-anonymous_pro")]
    AnonymousPro,
    #[cfg(feature = "font-camingo_code")]
    CamingoCode,
    #[cfg(feature = "font-ubuntu")]
    Ubuntu,
}

impl FontFamily {
    pub fn from_name(name: String) -> Option<Self> {
        for item in Self::list().into_iter() {
            if item.to_string() == name {
                return Some(item);
            }
        }
        None
    }
    pub fn list() -> Vec<Self> {
        let mut result = vec![];
        #[cfg(feature = "font-iosevka_term")]
        result.push(Self::IosevkaTerm);
        #[cfg(feature = "font-anonymous_pro")]
        result.push(Self::AnonymousPro);
        #[cfg(feature = "font-camingo_code")]
        result.push(Self::CamingoCode);
        #[cfg(feature = "font-ubuntu")]
        result.push(Self::Ubuntu);
        result
    }

    pub fn regular(&self) -> &[u8] {
        match self {
            #[cfg(feature = "font-iosevka_term")]
            Self::IosevkaTerm => {
                flate!(static DATA: [u8] from
                    "resources/fonts/IosevkaTerm/IosevkaTermNerdFontMono-Medium.ttf");
                &DATA
            }
            #[cfg(feature = "font-anonymous_pro")]
            Self::AnonymousPro => {
                flate!(static DATA: [u8] from
                    "resources/fonts/Anonymous_Pro/Anonymous_Pro.ttf");
                &DATA
            }
            #[cfg(feature = "font-camingo_code")]
            Self::CamingoCode => {
                flate!(static DATA: [u8] from
                    "resources/fonts/CamingoCode/CamingoCode-Regular.ttf");
                &DATA
            }
            #[cfg(feature = "font-ubuntu")]
            Self::Ubuntu => {
                flate!(static DATA: [u8] from
                    "resources/fonts/Ubuntu/UbuntuMono-R.ttf");
                &DATA
            }
        }
    }
    pub fn bold(&self) -> &[u8] {
        match self {
            #[cfg(feature = "font-iosevka_term")]
            Self::IosevkaTerm => {
                flate!(static DATA: [u8] from
                    "resources/fonts/IosevkaTerm/IosevkaTermNerdFontMono-Bold.ttf");
                &DATA
            }
            #[cfg(feature = "font-anonymous_pro")]
            Self::AnonymousPro => {
                flate!(static DATA: [u8] from
                    "resources/fonts/Anonymous_Pro/Anonymous_Pro_Bold.ttf");
                &DATA
            }
            #[cfg(feature = "font-camingo_code")]
            Self::CamingoCode => {
                flate!(static DATA: [u8] from
                    "resources/fonts/CamingoCode/CamingoCode-Bold.ttf");
                &DATA
            }

            #[cfg(feature = "font-ubuntu")]
            Self::Ubuntu => {
                flate!(static DATA: [u8] from
                    "resources/fonts/Ubuntu/UbuntuMono-B.ttf");
                &DATA
            }
        }
    }
    pub fn italic(&self) -> &[u8] {
        match self {
            #[cfg(feature = "font-iosevka_term")]
            Self::IosevkaTerm => {
                flate!(static DATA: [u8] from
                    "resources/fonts/IosevkaTerm/IosevkaTermNerdFontMono-Italic.ttf");
                &DATA
            }
            #[cfg(feature = "font-anonymous_pro")]
            Self::AnonymousPro => {
                flate!(static DATA: [u8] from
                    "resources/fonts/Anonymous_Pro/Anonymous_Pro_Italic.ttf");
                &DATA
            }
            #[cfg(feature = "font-camingo_code")]
            Self::CamingoCode => {
                flate!(static DATA: [u8] from
                    "resources/fonts/CamingoCode/CamingoCode-Italic.ttf");
                &DATA
            }

            #[cfg(feature = "font-ubuntu")]
            Self::Ubuntu => {
                flate!(static DATA: [u8] from
                    "resources/fonts/Ubuntu/UbuntuMono-RI.ttf");
                &DATA
            }
        }
    }
    pub fn bold_italic(&self) -> &[u8] {
        match self {
            #[cfg(feature = "font-iosevka_term")]
            Self::IosevkaTerm => {
                flate!(static DATA: [u8] from
                    "resources/fonts/IosevkaTerm/IosevkaTermNerdFontMono-BoldItalic.ttf");
                &DATA
            }
            #[cfg(feature = "font-anonymous_pro")]
            Self::AnonymousPro => {
                flate!(static DATA: [u8] from
                    "resources/fonts/Anonymous_Pro/Anonymous_Pro_Bold_Italic.ttf");
                &DATA
            }
            #[cfg(feature = "font-camingo_code")]
            Self::CamingoCode => {
                flate!(static DATA: [u8] from
                    "resources/fonts/CamingoCode/CamingoCode-BoldItalic.ttf");
                &DATA
            }

            #[cfg(feature = "font-ubuntu")]
            Self::Ubuntu => {
                flate!(static DATA: [u8] from
                    "resources/fonts/Ubuntu/UbuntuMono-BI.ttf");
                &DATA
            }
        }
    }
}

impl Default for FontFamily {
    fn default() -> Self {
        #[cfg(feature = "font-iosevka_term")]
        return Self::IosevkaTerm;
        #[cfg(feature = "font-anonymous_pro")]
        return Self::AnonymousPro;
        #[cfg(feature = "font-camingo_code")]
        return Self::CamingoCode;
        #[cfg(feature = "font-ubuntu")]
        return Self::Ubuntu;
    }
}

impl Display for FontFamily {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match self {
            #[cfg(feature = "font-iosevka_term")]
            Self::IosevkaTerm => "IosevkaTerm",
            #[cfg(feature = "font-anonymous_pro")]
            Self::AnonymousPro => "AnonymousPro",
            #[cfg(feature = "font-camingo_code")]
            Self::CamingoCode => "CamingoCode",
            #[cfg(feature = "font-ubuntu")]
            Self::Ubuntu => "Ubuntu",
        };
        write!(f, "{}", name)
    }
}
