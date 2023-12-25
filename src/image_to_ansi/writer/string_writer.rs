use std::io::{self, Write};

use termcolor::{Color, ColorSpec, WriteColor};

/// Override Output stream
pub struct StringWriter {
    inner_buf: Vec<u8>,
}

impl StringWriter {
    pub fn new() -> StringWriter {
        StringWriter { inner_buf: vec![] }
    }
    pub fn read(&mut self) -> String {
        let result = String::from_utf8_lossy(self.inner_buf.as_slice());
        return result.to_string();
    }

    fn write_str(&mut self, s: &str) -> io::Result<()> {
        self.write(s.as_bytes()).map(|_| ())
    }

    fn write_color(&mut self, fg: bool, c: &Color, intense: bool) -> io::Result<()> {
        macro_rules! write_intense {
            ($clr:expr) => {
                if fg {
                    self.write_str(concat!("\x1B[38;5;", $clr, "m"))
                } else {
                    self.write_str(concat!("\x1B[48;5;", $clr, "m"))
                }
            };
        }
        macro_rules! write_normal {
            ($clr:expr) => {
                if fg {
                    self.write_str(concat!("\x1B[3", $clr, "m"))
                } else {
                    self.write_str(concat!("\x1B[4", $clr, "m"))
                }
            };
        }
        macro_rules! write_var_ansi_code {
            ($pre:expr, $($code:expr),+) => {{
                // The loop generates at worst a literal of the form
                // '255,255,255m' which is 12-bytes.
                // The largest `pre` expression we currently use is 7 bytes.
                // This gives us the maximum of 19-bytes for our work buffer.
                let pre_len = $pre.len();
                assert!(pre_len <= 7);
                let mut fmt = [0u8; 19];
                fmt[..pre_len].copy_from_slice($pre);
                let mut i = pre_len - 1;
                $(
                    let c1: u8 = ($code / 100) % 10;
                    let c2: u8 = ($code / 10) % 10;
                    let c3: u8 = $code % 10;
                    let mut printed = false;

                    if c1 != 0 {
                        printed = true;
                        i += 1;
                        fmt[i] = b'0' + c1;
                    }
                    if c2 != 0 || printed {
                        i += 1;
                        fmt[i] = b'0' + c2;
                    }
                    // If we received a zero value we must still print a value.
                    i += 1;
                    fmt[i] = b'0' + c3;
                    i += 1;
                    fmt[i] = b';';
                )+

                fmt[i] = b'm';
                self.write_all(&fmt[0..i+1])
            }}
        }
        macro_rules! write_custom {
            ($ansi256:expr) => {
                if fg {
                    write_var_ansi_code!(b"\x1B[38;5;", $ansi256)
                } else {
                    write_var_ansi_code!(b"\x1B[48;5;", $ansi256)
                }
            };

            ($r:expr, $g:expr, $b:expr) => {{
                if fg {
                    write_var_ansi_code!(b"\x1B[38;2;", $r, $g, $b)
                } else {
                    write_var_ansi_code!(b"\x1B[48;2;", $r, $g, $b)
                }
            }};
        }
        if intense {
            match *c {
                Color::Black => write_intense!("8"),
                Color::Blue => write_intense!("12"),
                Color::Green => write_intense!("10"),
                Color::Red => write_intense!("9"),
                Color::Cyan => write_intense!("14"),
                Color::Magenta => write_intense!("13"),
                Color::Yellow => write_intense!("11"),
                Color::White => write_intense!("15"),
                Color::Ansi256(c) => write_custom!(c),
                Color::Rgb(r, g, b) => write_custom!(r, g, b),
                Color::__Nonexhaustive => unreachable!(),
            }
        } else {
            match *c {
                Color::Black => write_normal!("0"),
                Color::Blue => write_normal!("4"),
                Color::Green => write_normal!("2"),
                Color::Red => write_normal!("1"),
                Color::Cyan => write_normal!("6"),
                Color::Magenta => write_normal!("5"),
                Color::Yellow => write_normal!("3"),
                Color::White => write_normal!("7"),
                Color::Ansi256(c) => write_custom!(c),
                Color::Rgb(r, g, b) => write_custom!(r, g, b),
                Color::__Nonexhaustive => unreachable!(),
            }
        }
    }
}

impl Write for StringWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner_buf.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner_buf.flush()
    }
}

impl WriteColor for StringWriter {
    fn supports_color(&self) -> bool {
        return true;
    }

    fn set_color(&mut self, spec: &ColorSpec) -> io::Result<()> {
        if spec.reset() {
            self.reset()?;
        }
        if spec.bold() {
            self.write_str("\x1B[1m")?;
        }
        if spec.dimmed() {
            self.write_str("\x1B[2m")?;
        }
        if spec.italic() {
            self.write_str("\x1B[3m")?;
        }
        if spec.underline() {
            self.write_str("\x1B[4m")?;
        }
        if spec.strikethrough() {
            self.write_str("\x1B[9m")?;
        }
        if let Some(ref c) = spec.fg() {
            self.write_color(true, c, spec.intense())?;
        }
        if let Some(ref c) = spec.bg() {
            self.write_color(false, c, spec.intense())?;
        }
        Ok(())
    }
    fn reset(&mut self) -> io::Result<()> {
        self.write_str("\x1B[0m")
    }

    fn is_synchronous(&self) -> bool {
        false
    }
}
