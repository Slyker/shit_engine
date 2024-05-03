use super::rgb::Rgb;
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Hsv {
    pub h: f64,
    pub s: f64,
    pub v: f64,
    pub a: f64,
}

impl Hsv {
    pub fn new(h: f64, s: f64, v: f64) -> Self {
        let hsv = Hsv { h, s, v, a: 1.0 };
        Hsv::assert_valid_bounds(&hsv);
        hsv
    }

    pub fn new_with_alpha(h: f64, s: f64, v: f64, a: f64) -> Self {
        let hsv = Hsv { h, s, v, a };
        Hsv::assert_valid_bounds(&hsv);
        hsv
    }

    pub fn valid_bounds(values: &Hsv) -> Result<(), String> {
        if values.h < 0.0 || values.h > 360.0 {
            return Err("Hue is not between 0.0 and 360.0".to_string());
        }
        if values.s < 0.0 || values.s > 1.0 {
            return Err("Saturation is not between 0.0 and 1.0".to_string());
        }
        if values.v < 0.0 || values.v > 1.0 {
            return Err("Value is not between 0.0 and 1.0".to_string());
        }
        Ok(())
    }

    pub fn assert_valid_bounds(values: &Hsv) {
        let valid = Hsv::valid_bounds(values);
        assert!(valid.is_ok(), "{}", valid.unwrap_err());
    }
    #[allow(dead_code)]
    pub fn diff(&self, other: &Hsv) -> Hsv {
        Hsv {
            h: (self.h - other.h).abs(),
            s: (self.s - other.s).abs(),
            v: (self.v - other.v).abs(),
            a: (self.a - other.a).abs(),
        }
    }
    #[allow(dead_code)]
    pub fn compare(&self, other: &Hsv, tolerance: Hsv) -> bool {
        if  (self.h - other.h).abs() <= tolerance.h {
            if  (self.s - other.s).abs() <= tolerance.s {
                if  (self.v - other.v).abs() <= tolerance.v {
                    if  (self.a - other.a).abs() <= tolerance.a {
                        return true;
                    }
                }
            }
        }
        false
    }
    #[allow(dead_code)]
    pub fn compare_from_rgb(&self, other: &Rgb, tolerance: Hsv) -> bool {
        let other = Hsv::from(other);
        if  (self.h - other.h).abs() <= tolerance.h {
            if  (self.s - other.s).abs() <= tolerance.s {
                if  (self.v - other.v).abs() <= tolerance.v {
                    if  (self.a - other.a).abs() <= tolerance.a {
                        return true;
                    }
                }
            }
        }
        false
    }
    #[allow(dead_code)]
    pub fn compare_to_rgb(&self, other: &Rgb, tolerance: Rgb) -> bool {
        let rgb = Rgb::from(self);
        rgb.compare(other, tolerance)
    }
}

impl From<[f64; 3]> for Hsv {
    fn from(hsv: [f64; 3]) -> Self {
        let hsv = Hsv {
            h: hsv[0],
            s: hsv[1],
            v: hsv[2],
            a: 1.0, // Opaque
        };
        Hsv::assert_valid_bounds(&hsv);
        hsv
    }
}

impl From<[f64; 4]> for Hsv {
    fn from(hsva: [f64; 4]) -> Self {
        let hsv = Hsv {
            h: hsva[0],
            s: hsva[1],
            v: hsva[2],
            a: hsva[3],
        };
        Hsv::assert_valid_bounds(&hsv);
        hsv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_bounds() {
        // Valid
        let _ = Hsv::from([0.0, 0.0, 0.0, 1.0]);
        let _ = Hsv::from([360.0, 1.0, 1.0, 1.0]);
        let _ = Hsv::from([180.0, 0.5, 0.5, 1.0]);
    }

    #[test]
    #[should_panic(expected = "Hue is not between 0.0 and 360.0")]
    fn test_invalid_bounds_hue_negative() {
        let _ = Hsv::from([-1.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    #[should_panic(expected = "Hue is not between 0.0 and 360.0")]
    fn test_invalid_bounds_hue_high() {
        let _ = Hsv::from([361.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    #[should_panic(expected = "Saturation is not between 0.0 and 1.0")]
    fn test_invalid_bounds_saturation_negative() {
        let _ = Hsv::from([0.0, -0.1, 0.0, 1.0]);
    }

    #[test]
    #[should_panic(expected = "Saturation is not between 0.0 and 1.0")]
    fn test_invalid_bounds_saturation_high() {
        let _ = Hsv::from([0.0, 1.1, 0.0, 1.0]);
    }

    #[test]
    #[should_panic(expected = "Value is not between 0.0 and 1.0")]
    fn test_invalid_bounds_value_negative() {
        let _ = Hsv::from([0.0, 0.0, -0.1, 1.0]);
    }

    #[test]
    #[should_panic(expected = "Value is not between 0.0 and 1.0")]
    fn test_invalid_bounds_value_high() {
        let _ = Hsv::from([0.0, 0.0, 1.1, 1.0]);
    }
}
