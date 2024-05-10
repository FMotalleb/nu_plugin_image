use ab_glyph::PxScale;

#[derive(Clone, Copy)]
pub struct InternalScale {
    /// Horizontal scale, in pixels.
    pub x: f32,
    /// Vertical scale, in pixels.
    pub y: f32,
}

// impl Into<Scale> for InternalScale {
//     fn into(self) -> Scale {
//         Scale {
//             x: self.x,
//             y: self.y,
//         }
//     }
// }

impl Into<PxScale> for InternalScale {
    fn into(self) -> PxScale {
        PxScale {
            x: self.x,
            y: self.y,
        }
    }
}
