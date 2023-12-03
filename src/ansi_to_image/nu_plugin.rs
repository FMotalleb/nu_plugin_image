use std::{fs::File, io::Read, path::PathBuf};

use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::{Span, Value};
use rusttype::Font;

use crate::FontFamily;

use super::ansi_to_image::make_image;

pub fn ansi_to_image(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    let i: &[u8] = match input.as_binary().ok() {
        Some(value) => value,
        _ => {
            return Err(make_params_err(
                "cannot read input as binary data (maybe its empty)".to_string(),
                Some(input.span()),
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
    eprintln!("selected font: {}", font.to_string());
    let out = match call.get_flag_value("output-path").map(|i| i.as_path()) {
        Some(path) if path.is_ok() => path.unwrap(),
        _ => {
            return Err(make_params_err(
                "`--output-path` parameter is not correct file path value".to_string(),
                Some(call.head),
            ))
        }
    };

    make_image(out.as_path(), font, size, i);

    Ok(Value::nothing(call.head))
}

fn resolve_font(call: &EvaluatedCall) -> FontFamily<'_> {
    let mut font = match call.get_flag_value("font").map(|value| match value {
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
    if let Some(path) = call.get_flag_value("font-regular") {
        let buffer = load_file(path);
        font.regular = Font::try_from_vec(buffer).unwrap();
    }
    if let Some(path) = call.get_flag_value("font-bold") {
        let buffer = load_file(path);
        font.bold = Font::try_from_vec(buffer).unwrap();
    }
    if let Some(path) = call.get_flag_value("font-italic") {
        let buffer = load_file(path);
        font.italic = Font::try_from_vec(buffer).unwrap();
    }
    if let Some(path) = call.get_flag_value("bold-italic") {
        let buffer = load_file(path);
        font.bold_italic = Font::try_from_vec(buffer).unwrap();
    }
    font
}

fn load_file(path: Value) -> Vec<u8> {
    let path = path.as_string().unwrap();
    let mut file = File::open(PathBuf::from(path)).unwrap();
    let mut buffer = Vec::new();

    // read the whole file
    let _ = file.read_to_end(&mut buffer);
    buffer
}

fn make_params_err(text: String, span: Option<Span>) -> LabeledError {
    return LabeledError {
        label: "faced an error when tried to parse the params".to_string(),
        msg: text,
        span: span,
    };
}
