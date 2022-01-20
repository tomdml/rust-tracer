use std::ops::{Add, Neg, Sub, Mul, Div};
use crate::utils::fp_approx_eq;


#[derive(Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
    pub magnitude: f64,
}

impl Add for Tuple {
    type Output = Tuple;
    fn add(self, other: Tuple) -> Tuple {
        Tuple::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl Sub for Tuple {
    type Output = Tuple;
    fn sub(self, other: Tuple) -> Tuple {
        self + -other
    }
}

impl Neg for Tuple {
    type Output = Tuple;
    fn neg(self) -> Tuple {
        Tuple::new(
            -self.x,
            -self.y,
            -self.z,
            -self.w,
        )
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;
    fn mul(self, scalar: f64) -> Tuple {
        Tuple::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            self.w * scalar,
        )
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;
    fn div(self, scalar: f64) -> Tuple {
        self * (1. / scalar)
    }
}

impl PartialEq for Tuple {

    fn eq(&self, other: &Tuple) -> bool {
        fp_approx_eq(self.x, other.x) &&
        fp_approx_eq(self.y, other.y) &&
        fp_approx_eq(self.z, other.z) &&
        fp_approx_eq(self.w, other.w)
    }

}

impl Tuple {

    fn new<T: Into<f64>>(x: T, y: T, z: T, w: T) -> Tuple {
        let (x, y, z, w) = (x.into(), y.into(), z.into(), w.into());

        let magnitude: f64 = (
            x.powi(2) + y.powi(2) + z.powi(2) + w.powi(2)
        ).sqrt();

        Tuple { x, y, z, w, magnitude }
    }

    fn point<T: Into<f64>>(x: T, y: T, z: T) -> Tuple {
        Tuple::new(x.into(), y.into(), z.into(), 1.)
    }

    fn vector<T: Into<f64>>(x: T, y: T, z: T) -> Tuple {
        Tuple::new(x.into(), y.into(), z.into(), 0.)
    }

    fn normalize(&self) -> Tuple {
        Tuple::new(
            self.x / self.magnitude,
            self.y / self.magnitude,
            self.z / self.magnitude,
            self.w / self.magnitude
        )
    }

    /// The dot product is proportional to the cosine of the angle between the two input vectors.
    fn dot(&self, other: &Tuple) -> f64 {
        assert_eq!(self.w, 0.);
        assert_eq!(other.w, 0.);

        self.x * other.x +
        self.y * other.y +
        self.z * other.z +
        self.w * other.w
    }

    /// The cross product produces a vector perpendicular to the two input vectors.
    fn cross(&self, other: &Tuple) -> Tuple {
        assert_eq!(self.w, 0.);
        assert_eq!(other.w, 0.);

        Tuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tuple_with_w_1_is_a_point() {
        let t = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert_eq!(t.x, 4.3);
        assert_eq!(t.y, -4.2);
        assert_eq!(t.z, 3.1);
        assert_eq!(t.w, 1.0);
    }

    #[test]
    fn a_tuple_with_w_0_is_a_vector() {
        let t = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert_eq!(t.x, 4.3);
        assert_eq!(t.y, -4.2);
        assert_eq!(t.z, 3.1);
        assert_eq!(t.w, 0.0);
    }

    #[test]
    fn point_creates_tuples_with_w_1() {
        let p = Tuple::point(4, -4, 3);

        assert_eq!(p, Tuple::new(4, -4, 3, 1));
    }

    #[test]
    fn vector_creates_tuples_with_w_0() {
        let p = Tuple::vector(4, -4, 3);

        assert_eq!(p, Tuple::new(4, -4, 3, 0));
    }

    #[test]
    fn adding_two_tuples() {
        let a = Tuple::new(3, -2, 5, 1);
        let b = Tuple::new(-2, 3, 1, 0);

        assert_eq!(a + b, Tuple::new(1, 1, 6, 1));
    }

    #[test]
    fn subtracting_two_points() {
        let a = Tuple::point(3, 2, 1);
        let b = Tuple::point(5, 6, 7);

        assert_eq!(a - b, Tuple::vector(-2, -4, -6));
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let a = Tuple::point(3, 2, 1);
        let b = Tuple::vector(5, 6, 7);

        assert_eq!(a - b, Tuple::point(-2, -4, -6));
    }

    #[test]
    fn subtracting_two_vectors() {
        let a = Tuple::point(3, 2, 1);
        let b = Tuple::vector(5, 6, 7);

        assert_eq!(a - b, Tuple::point(-2, -4, -6));
    }

    #[test]
    fn negating_a_tuple() {
        let a = Tuple::new(1, -2, 3, -4);

        assert_eq!(-a, Tuple::new(-1, 2, -3, 4));
    }


    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = Tuple::new(1, -2, 3, -4);

        assert_eq!(a * 3.5, Tuple::new(3.5, -7., 10.5, -14.));
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let a = Tuple::new(1, -2, 3, -4);

        assert_eq!(a * 0.5, Tuple::new(0.5, -1., 1.5, -2.));
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a = Tuple::new(1, -2, 3, -4);

        assert_eq!(a / 2., Tuple::new(0.5, -1., 1.5, -2.));
    }

    #[test]
    fn magnitude_of_vector_100() {
        let v = Tuple::vector(1, 0, 0);

        assert_eq!(v.magnitude, 1.);
    }

    #[test]
    fn magnitude_of_vector_010() {
        let v = Tuple::vector(0, 1, 0);

        assert_eq!(v.magnitude, 1.);
    }

    #[test]
    fn magnitude_of_vector_001() {
        let v = Tuple::vector(0, 0, 1);

        assert_eq!(v.magnitude, 1.);
    }

    #[test]
    fn magnitude_of_vector_123() {
        let v = Tuple::vector(1, 2, 3);

        assert_eq!(v.magnitude, 14_f64.sqrt());
    }

    #[test]
    fn magnitude_of_vector_negative_123() {
        let v = Tuple::vector(-1, -2, -3);

        assert_eq!(v.magnitude, 14_f64.sqrt());
    }

    #[test]
    fn normalizing_vector_400_gives_100() {
        let v = Tuple::vector(4, 0, 0);

        assert_eq!(v.normalize(), Tuple::vector(1, 0, 0));
    }

    #[test]
    fn normalizing_vector_123() {
        let v = Tuple::vector(1, 2, 3);

        assert_eq!(v.normalize(),Tuple::vector(0.26726, 0.53452, 0.80178));
    }

    #[test]
    fn dot_product_of_two_tuples() {
        let a = Tuple::vector(1, 2, 3);
        let b = Tuple::vector(2, 3, 4);

        assert_eq!(a.dot(&b), 20.);
    }

    #[test]
    fn cross_product_of_two_tuples() {
        let a = Tuple::vector(1, 2, 3);
        let b = Tuple::vector(2, 3, 4);

        assert_eq!(a.cross(&b), Tuple::vector(-1, 2, -1));
        assert_eq!(b.cross(&a), Tuple::vector(1, -2, 1));

    }

}
