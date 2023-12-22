mod tests {
    use division_math::{Vector3, approx};

    #[test]
    fn vector_to_scalar_mul() {
        let v = Vector3::one() * 2.;
        assert_eq!(v.x, 2.);
        assert_eq!(v.y, 2.);
        assert_eq!(v.z, 2.);
    }

    #[test]
    #[inline(never)]
    fn vector_to_vector_mul() {
        let v1 = Vector3::all(3f32);
        let v2 = Vector3::all(2f32);

        let res = v1 * v2;
        assert_eq!(res, Vector3::all(6f32));
    }

    #[test]
    #[inline(never)]
    fn vector_to_normalized() {
        let v1 = Vector3::all(2f32);
        let norm = v1.normalized();

        assert!(approx(norm.length(), 1.));
    }

    #[test]
    fn vector_cross() {
        let right = Vector3::right();
        let up = Vector3::up();

        assert_eq!(Vector3::cross(right, up), Vector3::forward());
    }

    #[test]
    fn vector_from_tuple() {
        let x = (1., 2., 3.);

        let expected = Vector3::new(1., 2., 3.);
        let actual = Vector3::from(x);

        assert_eq!(expected, actual)
    }

    #[test]
    fn vector_clamp_length() {
        let v = Vector3::new(3., 3., 3.);
        let expected_len = (3f32 * 3f32).sqrt() / 2.;

        let v = v.clamp_length(0., expected_len);
        let actual_len = v.length();

        assert!(approx(actual_len, expected_len));
    }
}