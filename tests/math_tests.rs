mod tests {
    use division_math::{math, Vector3};

    #[test]
    pub fn lerp_tests() {
        assert!(math::approx(math::lerp(0., 1., 0.5), 0.5));
        assert!(math::approx(math::lerp(0.5, 1., 0.5), 0.75));
        assert!(math::approx(math::lerp(0., 1., 0.), 0.));
        assert!(math::approx(math::lerp(0., 1., 1.), 1.));
    }

    #[test]
    pub fn vector_lerp_test() {
        assert!(Vector3::approx(
            Vector3::lerp(Vector3::zero(), Vector3::one(), 0.5),
            Vector3::all(0.5)
        ));
    }
}