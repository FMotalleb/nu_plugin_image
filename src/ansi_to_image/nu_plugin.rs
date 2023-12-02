use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::{Span, Value};

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
    let font = match call.get_flag_value("font").map(|value| match value {
        Value::String { val, .. } => FontFamily::from_name(val),
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

fn make_params_err(text: String, span: Option<Span>) -> LabeledError {
    return LabeledError {
        label: "faced an error when tried to parse the params".to_string(),
        msg: text,
        span: span,
    };
}
