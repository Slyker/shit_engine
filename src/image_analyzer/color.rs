use self::{hsv::Hsv, rgb::Rgb};

pub mod conversion;
pub mod hsv;
pub mod rgb;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    pub hsv: hsv::Hsv,
    pub rgb: rgb::Rgb,

    pub tolerance: hsv::Hsv,
}

impl Color {
    pub fn compare(&self, color: &Color) -> bool {
        self.hsv.compare(&color.hsv, self.tolerance)
    }
}

impl From<Hsv> for Color {
    fn from(hsv: Hsv) -> Self {
        let rgb = Rgb::from(hsv);
        let tolerance = Hsv::new(0.0, 0.0, 0.0);
        Color {
            hsv,
            rgb,
            tolerance,
        }
    }
}

impl From<(Hsv, Hsv)> for Color {
    fn from(hsv: (Hsv, Hsv)) -> Self {
        let tolerance = hsv.1;
        let hsv = hsv.0;
        Color {
            hsv,
            rgb: Rgb::new(0, 0, 0),
            tolerance,
        }
    }
}

impl From<Rgb> for Color {
    fn from(rgb: Rgb) -> Self {
        let hsv = Hsv::from(rgb);
        let tolerance = Hsv::new(0.0, 0.0, 0.0);
        Color {
            hsv,
            rgb,
            tolerance,
        }
    }
}

impl From<(Rgb, Hsv)> for Color {
    fn from(hsv: (Rgb, Hsv)) -> Self {
        let tolerance = hsv.1;
        let rgb = hsv.0;
        let hsv = Hsv::from(rgb);
        Color {
            hsv,
            rgb,
            tolerance,
        }
    }
}
