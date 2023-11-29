#[derive(Debug, Clone, Copy)]
pub(super) enum ColorType {
    PrimaryForeground,
    PrimaryBackground,
    Normal(Color),
    Bright(Color),
    Fixed(u8),
    Rgb((u8, u8, u8)),
}

#[derive(Debug, Clone, Copy)]
pub(super) enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}
