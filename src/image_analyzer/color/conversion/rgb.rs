use crate::image_analyzer::color::{hsv::Hsv, rgb::Rgb};

use super::rgb_from_hsv;

impl From<Hsv> for Rgb {
    fn from(hsv: Hsv) -> Self {
        rgb_from_hsv(&hsv)
    }
}

impl From<&Hsv> for Rgb {
    fn from(hsv: &Hsv) -> Self {
        rgb_from_hsv(hsv)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_hsv() {
        let hsv = Hsv {
            h: 0.0,
            s: 1.0,
            v: 1.0,
            a: 1.0,
        };
        let rgb = Rgb {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        };
        assert_eq!(rgb, Rgb::from(hsv));
    }
}
