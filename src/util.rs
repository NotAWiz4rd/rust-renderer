pub fn a_equal_b(a: f64, b: f64) -> bool {
    a.abs() - b.abs() < 1e-10
}
