use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::Value;

use super::ansi_to_image::make_image;

pub fn ansi_to_image(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    let i = input.as_binary().unwrap();
    let size = match call.get_flag_value("width") {
        Some(val) => Some(val.as_int().unwrap() as u32),
        None => None,
    };
    let out = call
        .get_flag_value("output-path")
        .unwrap()
        .as_path()
        .unwrap();
    make_image(out.as_path(), size, i);
    Ok(Value::nothing(call.head))
}
