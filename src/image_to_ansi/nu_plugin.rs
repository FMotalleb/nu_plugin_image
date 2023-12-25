use image::codecs::png::PngDecoder;
use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::{Span, Value};

pub fn image_to_ansi(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    match build_params(call, input) {
        Ok(params) => {
            let mut config = viuer::Config::default();
            config.use_stderr = true;
            config.absolute_offset = false;
            config.use_iterm = true;
            config.use_kitty = false;
            config.height = params.height;
            config.width = params.width;

            let img =
                image::DynamicImage::from_decoder(PngDecoder::new(params.file.as_slice()).unwrap());
            let result = viuer::to_ansi(&img.unwrap(), &config);
            return Ok(Value::string(result.unwrap(), call.head));
        }
        Err(err) => Err(err),
    }
}

struct IntoAnsiParams {
    file: Vec<u8>,
    // verbose: bool,
    reverse_bg: bool,
    blinking: bool,
    width: Option<u32>,
    height: Option<u32>,
    character: Option<String>,
    font_width: u32,
    font_height: u32,
}

fn build_params(call: &EvaluatedCall, input: &Value) -> Result<IntoAnsiParams, LabeledError> {
    let mut params = IntoAnsiParams {
        file: [].to_vec(),
        // verbose: false,
        blinking: false,
        reverse_bg: false,
        height: None,
        width: None,
        character: None,
        font_height: 0,
        font_width: 0,
    };
    match input.as_binary() {
        Ok(file) => params.file = file.to_owned(),
        Err(err) => return Err(make_params_err(err.to_string(), Some(call.head))),
    };
    // params.verbose = call.has_flag("verbose");
    params.reverse_bg = call.has_flag("reverse-bg");
    params.blinking = call.has_flag("blink");
    params.width = match load_u32(call, "width") {
        Ok(value) => Some(value),
        Err(_) => None,
    };
    params.height = match load_u32(call, "height") {
        Ok(value) => Some(value),
        Err(_) => None,
    };
    params.font_width = match load_u32(call, "font-width") {
        Ok(value) => value,
        Err(_) => 20,
    };
    params.font_height = match load_u32(call, "font-height") {
        Ok(value) => value,
        Err(_) => 30,
    };
    params.character = match call.get_flag_value("char") {
        Some(ch) => match ch {
            Value::String { ref val, .. } => {
                if val.len() == 1 {
                    Some(val.to_owned())
                } else {
                    return Err(make_params_err(
                        format!(
                            "char value must be a single character. instead received `{}`",
                            val
                        ),
                        Some(ch.span()),
                    ));
                }
            }
            _ => None,
        },
        None => None,
    };

    Ok(params)
}

fn load_u32(call: &EvaluatedCall, flag_name: &str) -> Result<u32, LabeledError> {
    match call.get_flag_value(flag_name) {
        Some(val) => match val {
            Value::Int { .. } => match val.as_int().unwrap().try_into() {
                Ok(value) => Ok(value),
                Err(err) => Err(make_params_err(err.to_string(), Some(call.head))),
            },
            _ => Err(make_params_err(
                format!("value of `{}` is not an integer", flag_name),
                Some(call.head),
            )),
        },
        None => Err(make_params_err(
            format!("cannot find `{}` parameter", flag_name),
            Some(call.head),
        )),
    }
}

fn make_params_err(text: String, span: Option<Span>) -> LabeledError {
    return LabeledError {
        label: "faced an error when tried to parse the params".to_string(),
        msg: text,
        span: span,
    };
}
