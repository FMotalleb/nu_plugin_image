use image::DynamicImage;

use crate::image_to_ansi::nu_plugin::IntoAnsiParams;

use super::{make_ansi, string_writer::StringWriter};

pub fn to_ansi(img: &DynamicImage, config: &IntoAnsiParams) -> Result<String, String> {
    let stdout = &mut StringWriter::new();
    let _ = make_ansi(stdout, img, config);

    // if config.restore_cursor {
    //     execute!(&mut stdout, RestorePosition)?;
    // };

    Ok(stdout.read().to_string())
}
