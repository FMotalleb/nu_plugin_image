use std::{fs::File, io::Read, path::PathBuf, time::SystemTime};

use crate::{debug, error, warn};
use ab_glyph::FontRef;
use nu_plugin::EvaluatedCall;
use nu_protocol::{LabeledError, Span, Value};

use crate::FontFamily;

use super::{
    ansi_to_image::make_image,
    palette::{strhex_to_rgba, Palette},
};

pub fn ansi_to_image(
    engine: &nu_plugin::EngineInterface,
    call: &EvaluatedCall,
    input: &Value,
) -> Result<Value, LabeledError> {
    let i: &[u8] = match input {
        Value::String {
            val,
            internal_span: _,
        } => val.as_bytes(),
        Value::Binary {
            val,
            internal_span: _,
        } => val,
        _ => {
            return Err(make_params_err(
                "cannot read input as binary data (maybe its empty)".to_string(),
                input.span(),
            ))
        }
    };
    let size = match call.get_flag_value("width") {
        Some(val) => match val.as_int().ok() {
            Some(value) => Some(value as u32),
            _ => None,
        },
        _ => None,
    };
    let font: FontFamily<'_> = resolve_font(call);
    let out_path = call.opt::<String>(0);

    let out = match out_path {
        Ok(Some(path)) => {
            debug!("received output name `{}`", path);
            if let Ok(value) = engine.get_current_dir() {
                let mut absolute = PathBuf::from(value);
                absolute.extend(PathBuf::from(path).iter());
                debug!(
                    "absolute output name `{}`",
                    absolute.to_str().unwrap_or("cannot convert path to string")
                );
                Some(absolute)
            } else {
                warn!("failed to fetch current directories path");
                Some(PathBuf::from(path))
            }
        }
        _ => {
            let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
            let current = engine.get_current_dir().map(|p| PathBuf::from(p));
            if let (Ok(now), Ok(current)) = (now, current) {
                let current = &mut current.clone();
                current.push(PathBuf::from(format!("nu-image-{}.png", now.as_secs())));
                Some(current.to_owned())
            } else {
                None
            }
        }
    };
    if let None = out {
        return Err(make_params_err(
            format!("cannot use time stamp as the file name timestamp please provide output path explicitly"),
            call.head,
        ));
    }
    let theme = match call
        .get_flag_value("theme")
        .map(|i| i.as_str().map(|f| f.to_string()))
    {
        Some(Ok(name)) => {
            if let Some(theme) = Palette::from_name(name.to_string()) {
                theme
            } else {
                error!("No theme found that matches the given name");
                Palette::default()
            }
        }
        _ => Palette::default(),
    };
    let theme = load_custom_theme(call, theme);

    let path = PathBuf::from(out.unwrap());
    make_image(path.as_path(), font, size, i, theme);

    Ok(Value::string(
        path.to_str().unwrap_or("error reading path").to_owned(),
        call.head,
    ))
}

fn resolve_font(call: &EvaluatedCall) -> FontFamily<'static> {
    let mut font: FontFamily<'static> = match call.get_flag_value("font").map(|value| match value {
        Value::String { val, .. } => Some(FontFamily::from_name(val)),
        _ => None,
    }) {
        Some(value) => {
            if let Some(font) = value {
                font
            } else {
                FontFamily::default()
            }
        }
        None => FontFamily::default(),
    };
    // TODO custom fonts disabled for now
    if let Some(path) = call.get_flag_value("font-regular") {
        let buffer = load_file(path);
        font.regular = FontRef::try_from_slice(buffer).unwrap();
    }
    if let Some(path) = call.get_flag_value("font-bold") {
        let buffer = load_file(path);
        font.bold = FontRef::try_from_slice(buffer).unwrap();
    }
    if let Some(path) = call.get_flag_value("font-italic") {
        let buffer = load_file(path);
        font.italic = FontRef::try_from_slice(buffer).unwrap();
    }
    if let Some(path) = call.get_flag_value("bold-italic") {
        let buffer = load_file(path);
        font.bold_italic = FontRef::try_from_slice(buffer).unwrap();
    }
    font
}

// fn load_file<'a>(path: Value) -> &'a [u8] {
//     let path = path.as_str().unwrap();
//     let mut file = File::open(PathBuf::from(path)).unwrap();
//     let mut buffer = Vec::new();

//     // read the whole file
//     let _ = file.read_to_end(&mut buffer);
//     buffer.as_slice()
// }

fn load_file<'a>(path: Value) -> &'a [u8] {
    let path = path.as_str().unwrap();
    let mut file = File::open(PathBuf::from(path)).unwrap();
    let mut buffer: Box<Vec<u8>> = Box::new(vec![]);
    file.read_to_end(&mut *buffer).unwrap();
    Box::leak(buffer)
}

fn make_params_err(text: String, span: Span) -> LabeledError {
    LabeledError::new(text).with_label("faced an error when tried to parse the params", span)
}
fn load_custom_theme(call: &EvaluatedCall, theme: Palette) -> Palette {
    let result = theme.palette().copy_with(
        read_hex_to_array(call, "custom-theme-fg"),
        read_hex_to_array(call, "custom-theme-bg"),
        read_hex_to_array(call, "custom-theme-black"),
        read_hex_to_array(call, "custom-theme-red"),
        read_hex_to_array(call, "custom-theme-green"),
        read_hex_to_array(call, "custom-theme-yellow"),
        read_hex_to_array(call, "custom-theme-blue"),
        read_hex_to_array(call, "custom-theme-magenta"),
        read_hex_to_array(call, "custom-theme-cyan"),
        read_hex_to_array(call, "custom-theme-white"),
        read_hex_to_array(call, "custom-theme-bright_black"),
        read_hex_to_array(call, "custom-theme-bright_red"),
        read_hex_to_array(call, "custom-theme-bright_green"),
        read_hex_to_array(call, "custom-theme-bright_yellow"),
        read_hex_to_array(call, "custom-theme-bright_blue"),
        read_hex_to_array(call, "custom-theme-bright_magenta"),
        read_hex_to_array(call, "custom-theme-bright_cyan"),
        read_hex_to_array(call, "custom-theme-bright_white"),
    );
    Palette::Custom(result)
}

fn read_hex_to_array(call: &EvaluatedCall, name: &str) -> Option<[u8; 4]> {
    if let Some(Value::String { val, .. }) = call.get_flag_value(name) {
        return strhex_to_rgba(val);
    }
    None
}
