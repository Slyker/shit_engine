pub mod color_from;
pub mod conversion;
pub mod hsv;
pub mod rgb;
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Rgb(rgb::Rgb),
    Hsv(hsv::Hsv),
}

impl Color {
    #[allow(dead_code)]
    pub fn get_rgb(&self) -> rgb::Rgb {
        match self {
            Color::Rgb(rgb) => rgb.to_owned(),
            Color::Hsv(hsv) => rgb::Rgb::from(hsv),
        }
    }
    #[allow(dead_code)]
    pub fn get_hsv(&self) -> hsv::Hsv {
        match self {
            Color::Rgb(rgb) => hsv::Hsv::from(rgb),
            Color::Hsv(hsv) => hsv.to_owned(),
        }
    }
}
