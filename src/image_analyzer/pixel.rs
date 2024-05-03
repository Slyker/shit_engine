use super::{color::Color, point::Point};
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct Pixel {
    pub color: Color,
    pub points: Vec<Point>,
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct PixelVec {
    pub pixels: Vec<Pixel>,
    pub points_count: usize,
}

impl PixelVec {
    pub fn new() -> Self {
        Self {
            pixels: Vec::new(),
            points_count: 0,
        }
    }

    pub fn push(&mut self, (color, point): (Color, Point)) {
        self.points_count += 1;
        if let Some(pixel) = self.pixels.iter_mut().find(|pixel| pixel.color == color) {
            pixel.points.push(point);
            return;
        }
        self.pixels.push(Pixel {
            color,
            points: vec![point],
        });
    }
    #[allow(dead_code)]
    pub fn get_by_color(&self, color: &Color) -> Option<&Pixel> {
        self.pixels.iter().find(|pixel| &pixel.color == color)
    }
}

#[cfg(test)]
mod tests {
    use crate::image_analyzer::color::rgb::Rgb;

    use super::*;
    #[test]
    fn test_push() {
        let mut pixel_vec = PixelVec::new();
        let point = Point::new(0, 0);
        let color = Color::from(Rgb::new(0, 0, 0));
        pixel_vec.push((color.clone(), point.clone()));
        assert_eq!(pixel_vec.points_count, 1);
        assert_eq!(pixel_vec.pixels.len(), 1);
        assert_eq!(pixel_vec.pixels[0].points.len(), 1);
        assert_eq!(pixel_vec.pixels[0].points[0], point);
        assert_eq!(pixel_vec.pixels[0].color, color);
    }
    #[test]
    fn test_push_multiple() {
        let mut pixel_vec = PixelVec::new();
        let point = Point::new(0, 0);
        let color = Color::from(Rgb::new(0, 0, 0));
        pixel_vec.push((color.clone(), point.clone()));
        pixel_vec.push((color.clone(), point.clone()));
        assert_eq!(pixel_vec.points_count, 2);
        assert_eq!(pixel_vec.pixels.len(), 1);
        assert_eq!(pixel_vec.pixels[0].points.len(), 2);
        assert_eq!(pixel_vec.pixels[0].points[0], point.clone());
        assert_eq!(pixel_vec.pixels[0].points[1], point);
        assert_eq!(pixel_vec.pixels[0].color, color);
    }
    #[test]
    fn test_push_multiple_colors() {
        let mut pixel_vec = PixelVec::new();
        let point = Point::new(0, 0);
        let color = Color::from(Rgb::new(0, 0, 0));
        let color2 = Color::from(Rgb::new(0, 0, 1));
        pixel_vec.push((color.clone(), point.clone()));
        pixel_vec.push((color2.clone(), point.clone()));
        assert_eq!(pixel_vec.points_count, 2);
        assert_eq!(pixel_vec.pixels.len(), 2);
        assert_eq!(pixel_vec.pixels[0].points.len(), 1);
        assert_eq!(pixel_vec.pixels[1].points.len(), 1);
        assert_eq!(pixel_vec.pixels[0].points[0], point.clone());
        assert_eq!(pixel_vec.pixels[1].points[0], point);
        assert_eq!(pixel_vec.pixels[0].color.clone(), color.clone());
        assert_eq!(pixel_vec.pixels[1].color, color2);
    }
}
