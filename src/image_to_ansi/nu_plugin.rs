use std::env;

use image::codecs::png::PngDecoder;
use nu_plugin::EvaluatedCall;
use nu_protocol::{LabeledError, Span, Value};

pub fn image_to_ansi(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    match build_params(call, input) {
        Ok(params) => {
            let img = PngDecoder::new(params.file.as_slice())
                .map(|img| image::DynamicImage::from_decoder(img));
            match img {
                Ok(img) => {
                    let result = super::writer::lib::to_ansi(&img.unwrap(), &params);

                    return result
                        .map(|value| Value::string(value, call.head))
                        .map_err(|err| response_error(err, call.head));
                }
                Err(er) => Err(response_error(er.to_string(), call.head)),
            }
        }
        Err(err) => Err(err),
    }
}

pub(super) struct IntoAnsiParams {
    file: Vec<u8>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub truecolor: bool,
}

pub fn truecolor_available() -> bool {
    if let Ok(value) = env::var("COLORTERM") {
        value.contains("truecolor") || value.contains("24bit")
    } else {
        false
    }
}

fn build_params(call: &EvaluatedCall, input: &Value) -> Result<IntoAnsiParams, LabeledError> {
    let mut params = IntoAnsiParams {
        file: [].to_vec(),
        // verbose: false,
        height: None,
        width: None,
        truecolor: truecolor_available(),
    };
    match input.as_binary() {
        Ok(file) => params.file = file.to_owned(),
        Err(err) => return Err(make_params_err(err.to_string(), call.head)),
    };
    params.width = match load_u32(call, "width") {
        Ok(value) => Some(value),
        Err(_) => None,
    };
    params.height = match load_u32(call, "height") {
        Ok(value) => Some(value),
        Err(_) => None,
    };

    Ok(params)
}

fn load_u32(call: &EvaluatedCall, flag_name: &str) -> Result<u32, LabeledError> {
    match call.get_flag_value(flag_name) {
        Some(val) => match val {
            Value::Int { .. } => match val.as_int().unwrap().try_into() {
                Ok(value) => Ok(value),
                Err(err) => Err(make_params_err(err.to_string(), call.head)),
            },
            _ => Err(make_params_err(
                format!("value of `{}` is not an integer", flag_name),
                call.head,
            )),
        },
        None => Err(make_params_err(
            format!("cannot find `{}` parameter", flag_name),
            call.head,
        )),
    }
}

fn make_params_err(text: String, span: Span) -> LabeledError {
    return LabeledError::new(text)
        .with_label("faced an error when tried to parse the params", span);
}

fn response_error(text: String, span: Span) -> LabeledError {
    return LabeledError::new(text).with_label("cannot create image", span);
}
