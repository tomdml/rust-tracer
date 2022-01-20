use std::ops::{Add, Neg, Sub, Mul, Div};
use crate::utils::fp_approx_eq;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color::new(
            self.r + other.r,
            self.g + other.g,
            self.b + other.b,
        )
    }
}

impl Sub for Color {
    type Output = Color;
    fn sub(self, other: Color) -> Color {
        self + -other
    }
}

impl Neg for Color {
    type Output = Color;
    fn neg(self) -> Color {
        Color::new(
            -self.r,
            -self.g,
            -self.b,
        )
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, scalar: f32) -> Color {
        Color::new(
            self.r * scalar,
            self.g * scalar,
            self.b * scalar,
        )
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        Color::new(
            self.r * other.r,
            self.g * other.g,
            self.b * other.b,
        )
    }
}

impl Div<f32> for Color {
    type Output = Color;
    fn div(self, scalar: f32) -> Color {
        self * (1. / scalar)
    }
}

impl PartialEq for Color {

    fn eq(&self, other: &Color) -> bool {
        fp_approx_eq(self.r, other.r) &&
        fp_approx_eq(self.g, other.g) &&
        fp_approx_eq(self.b, other.b)
    }

}

impl Color {

    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_rgb_tuples() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(c.r, -0.5);
        assert_eq!(c.g, 0.4);
        assert_eq!(c.b, 1.7);
    }

    #[test]
    fn adding_colors() {
        let a = Color::new(0.9, 0.6, 0.75);
        let b = Color::new(0.7, 0.1, 0.25);

        assert_eq!(a + b, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let a = Color::new(0.9, 0.6, 0.75);
        let b = Color::new(0.7, 0.1, 0.25);

        assert_eq!(a - b, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);

        assert_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiplying_color_by_color() {
        let a = Color::new(1., 0.2, 0.4);
        let b = Color::new(0.9, 1., 0.1);

        assert_eq!(a * b, Color::new(0.9, 0.2, 0.04));
    }



}
