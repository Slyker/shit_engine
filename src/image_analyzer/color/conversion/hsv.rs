use crate::image_analyzer::color::{hsv::Hsv, rgb::Rgb};

use super::hsv_from_rgb;

impl From<Rgb> for Hsv {
    fn from(rgb: Rgb) -> Self {
        hsv_from_rgb(&rgb)
    }
}

impl From<&Rgb> for Hsv {
    fn from(rgb: &Rgb) -> Self {
        hsv_from_rgb(rgb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_rgb() {
        let rgb = Rgb {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        };
        let hsv = Hsv {
            h: 0.0,
            s: 1.0,
            v: 1.0,
            a: 1.0,
        };
        assert_eq!(hsv, Hsv::from(rgb));
    }
}
