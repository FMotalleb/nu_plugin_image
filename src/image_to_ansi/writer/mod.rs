pub mod lib;
mod string_writer;

use image::{DynamicImage, GenericImageView};

mod block;
pub use block::make_ansi;

/// Resize a [image::DynamicImage] so that it fits within optional width and height bounds.
/// If none are provided, terminal size is used instead.
pub fn resize(img: &DynamicImage, width: Option<u32>, height: Option<u32>) -> DynamicImage {
    let (w, h) = find_best_fit(img, width, height);

    // find_best_fit returns values in terminal cells. Hence, we multiply by two
    // because a 5x10 image can fit in 5x5 cells. However, a 5x9 image will also
    // fit in 5x5 and 1 is deducted in such cases.
    img.resize_exact(
        w,
        2 * h - img.height() % 2,
        image::imageops::FilterType::Triangle,
    )
}

/// Find the best dimensions for the printed image, based on user's input.
/// Returns the dimensions of how the image should be printed in **terminal cells**.
///
/// The behaviour is different based on the provided width and height:
/// - If both are None, the image will be resized to fit in the terminal. Aspect ratio is preserved.
/// - If only one is provided and the other is None, it will fit the image in the provided boundary. Aspect ratio is preserved.
/// - If both are provided, the image will be resized to match the new size. Aspect ratio is **not** preserved.
///
/// Example:
/// Use None for both dimensions to use terminal size (80x24) instead.
/// The image ratio is 2:1, the terminal can be split into 80x46 squares.
/// The best fit would be to use the whole width (80) and 40 vertical squares,
/// which is equivalent to 20 terminal cells.
///
/// let img = image::DynamicImage::ImageRgba8(image::RgbaImage::new(160, 80));
/// let (w, h) = find_best_fit(&img, None, None);
/// assert_eq!(w, 80);
/// assert_eq!(h, 20);
//TODO: it might make more sense to change signiture from img to (width, height)
fn find_best_fit(img: &DynamicImage, width: Option<u32>, height: Option<u32>) -> (u32, u32) {
    let (img_width, img_height) = img.dimensions();

    // Match user's width and height preferences
    match (width, height) {
        (None, None) => {
            let (term_w, term_h) = terminal_size();
            let (w, h) = fit_dimensions(img_width, img_height, term_w as u32, term_h as u32);

            // One less row because two reasons:
            // - the prompt after executing the command will take a line
            // - gifs flicker
            let h = if h == term_h as u32 { h - 1 } else { h };
            (w, h)
        }
        // Either width or height is specified, will fit and preserve aspect ratio.
        (Some(w), None) => fit_dimensions(img_width, img_height, w, img_height),
        (None, Some(h)) => fit_dimensions(img_width, img_height, img_width, h),

        // Both width and height are specified, will resize to match exactly
        (Some(w), Some(h)) => (w, h),
    }
}

/// Given width & height of an image, scale the size so that it can fit within given bounds
/// while preserving aspect ratio. Will only scale down - if dimensions are smaller than the
/// bounds, they will be returned unmodified.
///
/// Note: input bounds are meant to hold dimensions of a terminal, where the height of a cell is
/// twice it's width. It is best illustrated in an example:
///
/// Trying to fit a 100x100 image in 40x15 terminal cells. The best fit, while having an aspect
/// ratio of 1:1, would be to use all of the available height, 15, which is
/// equivalent in size to 30 vertical cells. Hence, the returned dimensions will be 30x15.
///
/// assert_eq!((30, 15), viuer::fit_dimensions(100, 100, 40, 15));
fn fit_dimensions(width: u32, height: u32, bound_width: u32, bound_height: u32) -> (u32, u32) {
    let bound_height = 2 * bound_height;

    if width <= bound_width && height <= bound_height {
        return (width, std::cmp::max(1, height / 2 + height % 2));
    }

    let ratio = width * bound_height;
    let nratio = bound_width * height;

    let use_width = nratio <= ratio;
    let intermediate = if use_width {
        height * bound_width / width
    } else {
        width * bound_height / height
    };

    if use_width {
        (bound_width, std::cmp::max(1, intermediate / 2))
    } else {
        (intermediate, std::cmp::max(1, bound_height / 2))
    }
}

const DEFAULT_TERM_SIZE: (u16, u16) = (80, 24);

pub fn terminal_size() -> (u16, u16) {
    match crossterm::terminal::size() {
        Ok(s) => s,
        Err(_) => DEFAULT_TERM_SIZE,
    }
}
