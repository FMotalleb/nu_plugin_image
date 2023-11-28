use clap::Parser;
use std::path::PathBuf;

/// Convert ansi output to pngs
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub(super) struct Opt {
    /// Path to the input file. File should contain utf8 text that uses ANSI
    /// escape codes.
    #[structopt(short, long)]
    pub(super) input_path: PathBuf,

    /// Path to output file. Will always write a png regardless of file
    /// extenstion.
    #[structopt(short, long)]
    pub(super) output_path: PathBuf,

    /// Maximum width of the png
    #[structopt(short, long)]
    pub(super) png_width: Option<u32>,
}
