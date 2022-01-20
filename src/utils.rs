pub fn fp_approx_eq<T: Into<f64>>(a: T, b: T) -> bool {
    const EPSILON: f64 = 0.00001;
    (a.into() - b.into()).abs() < EPSILON
}
