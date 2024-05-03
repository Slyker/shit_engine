#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}
#[allow(dead_code)]
impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        let x = ((self.x as f64) - (other.x as f64)).powi(2);
        let y = ((self.y as f64) - (other.y as f64)).powi(2);
        (x + y).sqrt()
    }

    pub fn is_inside(&self, point_min: &Point, point_max: &Point) -> bool {
        self.x >= point_min.x &&
            self.x <= point_max.x &&
            self.y >= point_min.y &&
            self.y <= point_max.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let point1 = Point::new(0, 0);
        let point2 = Point::new(3, 4);
        assert_eq!(point1.distance(&point2), 5.0);
    }

    #[test]
    fn test_is_inside() {
        let point1 = Point::new(0, 0);
        let point2 = Point::new(3, 4);
        let point_min = Point::new(0, 0);
        let point_max = Point::new(3, 4);
        assert_eq!(point1.is_inside(&point_min, &point_max), true);
        assert_eq!(point2.is_inside(&point_min, &point_max), true);
    }
}
