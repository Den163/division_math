mod tests {
    use division_math::{Vector3, approx, lerp};

    #[test]
    pub fn lerp_tests() {
        assert!(approx(lerp(0., 1., 0.5), 0.5));
        assert!(approx(lerp(0.5, 1., 0.5), 0.75));
        assert!(approx(lerp(0., 1., 0.), 0.));
        assert!(approx(lerp(0., 1., 1.), 1.));
    }

    #[test]
    pub fn vector_lerp_test() {
        assert!(Vector3::approx(
            Vector3::lerp(Vector3::zero(), Vector3::one(), 0.5),
            Vector3::all(0.5)
        ));
    }
}