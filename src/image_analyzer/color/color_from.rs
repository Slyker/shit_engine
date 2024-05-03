use super::hsv::Hsv;
use super::rgb::Rgb;

use super::conversion::*;

/// From Rgb to Hsv conversion
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

/// From Hsv to Rgb conversion
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
    fn test_rgb_to_hsv() {
        let rgb = &Rgb {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        };
        let hsv = &Hsv {
            h: 0.0,
            s: 1.0,
            v: 1.0,
            a: 1.0,
        };
        assert_eq!(hsv, &Hsv::from(rgb));
    }

    #[test]
    fn test_hsv_to_rgb() {
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
