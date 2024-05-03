use super::hsv::Hsv;
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug,PartialEq, Clone, Copy, Hash)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a:255 }
    }

    pub fn new_with_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn diff(&self, other: &Rgb) -> Rgb {
        Rgb {
            r: ((self.r as i16) - (other.r as i16)).abs() as u8,
            g: ((self.g as i16) - (other.g as i16)).abs() as u8,
            b: ((self.b as i16) - (other.b as i16)).abs() as u8,
            a: ((self.a as i16) - (other.a as i16)).abs() as u8,
        }
    }

    pub fn compare(&self, other: &Rgb, tolerance: Rgb) -> bool {
        let diff = self.diff(other);
        diff.r <= tolerance.r
            && diff.g <= tolerance.g
            && diff.b <= tolerance.b
            && diff.a <= tolerance.a
    }
    #[allow(dead_code)]
    pub fn compare_from_hsv(&self, other: &Hsv, tolerance: Rgb) -> bool {
        let diff = self.diff(&Rgb::from(other));
        diff.r <= tolerance.r
            && diff.g <= tolerance.g
            && diff.b <= tolerance.b
            && diff.a <= tolerance.a
    }
    #[allow(dead_code)]
    pub fn compare_to_hsv(&self, other: &Hsv, tolerance: Hsv) -> bool {
        let hsv = Hsv::from(self);
        hsv.compare(other, tolerance)
    }
}

impl From<[u8; 3]> for Rgb {
    fn from(rgb: [u8; 3]) -> Self {
        let rgb = Rgb {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
            a: 255, // Opaque
        };
        rgb
    }
}

impl From<[u8; 4]> for Rgb {
    fn from(rgba: [u8; 4]) -> Self {
        let rgb = Rgb {
            r: rgba[0],
            g: rgba[1],
            b: rgba[2],
            a: rgba[3],
        };
        rgb
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rgb_diff() {
        let rgb1 = Rgb {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        };
        let rgb2 = Rgb {
            r: 0,
            g: 255,
            b: 0,
            a: 255,
        };
        let diff = Rgb {
            r: 255,
            g: 255,
            b: 0,
            a: 0,
        };
        assert_eq!(diff, rgb1.diff(&rgb2));
    }

    #[test]
    fn test_rgb_compare() {
        let rgb1 = Rgb {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        };
        let rgb2 = Rgb {
            r: 0,
            g: 255,
            b: 0,
            a: 255,
        };
        let tolerance = Rgb {
            r: 255,
            g: 255,
            b: 0,
            a: 0,
        };
        assert!(rgb1.compare(&rgb2, tolerance));
    }

    #[test]
    fn test_hsv_diff() {
        let hsv1 = Hsv {
            h: 0.0,
            s: 1.0,
            v: 1.0,
            a: 1.0,
        };
        let hsv2 = Hsv {
            h: 120.0,
            s: 1.0,
            v: 1.0,
            a: 1.0,
        };
        let diff = Rgb {
            r: 255,
            g: 255,
            b: 0,
            a: 0,
        };
        assert_eq!(diff, Rgb::from(&hsv1).diff(&Rgb::from(&hsv2)));
    }

    #[test]
    fn test_hsv_compare() {
        let hsv1 = Hsv {
            h: 0.0,
            s: 1.0,
            v: 1.0,
            a: 1.0,
        };
        let hsv2 = Hsv {
            h: 120.0,
            s: 1.0,
            v: 1.0,
            a: 1.0,
        };
        let tolerance = Hsv {
            h: 180.0,
            s: 1.0,
            v: 1.0,
            a: 0.0,
        };
        assert!(hsv1.compare(&hsv2, tolerance));
    }

    #[test]
    fn test_rgb_compare_from_hsv() {
        let rgb1 = Rgb {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        };
        let hsv2 = Hsv {
            h: 120.0,
            s: 1.0,
            v: 1.0,
            a: 1.0,
        };
        let tolerance = Rgb {
            r: 255,
            g: 255,
            b: 0,
            a: 0,
        };
        assert!(rgb1.compare_from_hsv(&hsv2, tolerance));
    }

    #[test]
    fn test_hsv_compare_to_rgb() {
        let hsv1 = Hsv {
            h: 0.0,
            s: 1.0,
            v: 1.0,
            a: 1.0,
        };
        let rgb2 = Rgb {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        };
        let tolerance = Rgb {
            r: 255,
            g: 255,
            b: 0,
            a: 0,
        };
        assert!(hsv1.compare_to_rgb(&rgb2, tolerance));
    }

    #[test]
    fn test_from_array() {
        let rgb = Rgb {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        };
        let rgb_array = [255, 0, 0];
        assert_eq!(rgb, Rgb::from(rgb_array));

        let hsv = Hsv {
            h: 0.0,
            s: 1.0,
            v: 1.0,
            a: 1.0,
        };
        let hsv_array = [0.0, 1.0, 1.0, 1.0];
        assert_eq!(hsv, Hsv::from(hsv_array));
    }

    #[test]
    fn test_from_array_with_alpha() {
        let rgb = Rgb {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        };
        let rgb_array = [255, 0, 0, 255];
        assert_eq!(rgb, Rgb::from(rgb_array));

        let hsv = Hsv {
            h: 0.0,
            s: 1.0,
            v: 1.0,
            a: 1.0,
        };
        let hsv_array = [0.0, 1.0, 1.0, 1.0];
        assert_eq!(hsv, Hsv::from(hsv_array));
    }

}



