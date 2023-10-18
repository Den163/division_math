mod tests {
    use division_math::Vector4;

    #[test]
    fn vector_to_scalar_mul() {
        let v = Vector4::one() * 2f32;
        assert_eq!(v.x, 2f32);
        assert_eq!(v.y, 2f32);
        assert_eq!(v.z, 2f32);
    }

    #[test]
    #[inline(never)]
    fn vector_to_vector_mul() {
        let v1 = Vector4::all(3f32);
        let v2 = Vector4::all(2f32);

        let res = v1 * v2;
        assert_eq!(res, Vector4::all(6f32));
    }

    #[test]
    #[inline(never)]
    fn vector_to_normalized() {
        let v1 = Vector4::all(2f32);
        let norm = v1.normalized();

        assert!((norm.length() - 1f32).abs() <= f32::EPSILON);
    }

    #[test]
    #[inline(never)]
    fn vector_index() {
        let mut v = Vector4::new(1., 2., 3., 4.);
        let expected = 10.;
        v[2] = expected;

        assert_eq!(v.z, expected);
    }

    #[test]
    fn add_assign() {
        let origin = Vector4::new(1., 2., 3., 4.);
        let add = Vector4::one();
        let expected = origin + add;

        let mut v = origin;
        v += add;

        assert_eq!(v, expected);
    }

    #[test]
    fn sub_assign() {
        let origin = Vector4::new(1., 2., 3., 4.);
        let sub = Vector4::one();
        let expected = origin - sub;

        let mut v = origin;
        v -= sub;

        assert_eq!(v, expected);
    }

    #[test]
    fn mul_assign() {
        let origin = Vector4::new(1., 2., 3., 4.);
        let mul = Vector4::all(2.);
        let expected = origin * mul;

        let mut v = origin;
        v *= mul;

        assert_eq!(v, expected);
    }

    #[test]
    fn div_assign() {
        let origin = Vector4::new(1., 2., 3., 4.);
        let div = Vector4::all(2.);
        let expected = origin / div;

        let mut v = origin;
        v /= div;

        assert_eq!(v, expected);
    }

    #[test]
    fn rem_assign() {
        let origin = Vector4::new(1., 2., 3., 4.);
        let rem = Vector4::all(2.);
        let expected = origin % rem;

        let mut v = origin;
        v %= rem;

        assert_eq!(v, expected);
    }
}