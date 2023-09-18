mod tests {
    use division_math::{Matrix4x4, Vector3, Vector4};

    #[test]
    pub fn matrix_inverse_test() {
        let m = Matrix4x4::identity() * 2.;
        let m_i = m.inverse();

        assert!(Matrix4x4::approx(m * m_i, Matrix4x4::identity()));
    }

    #[test]
    pub fn matrix_identity_mul_test() {
        let m = Matrix4x4::from_columns(
            Vector4::new(0., 1., 2., 3.), 
            Vector4::new(3., 1., 2., 0.), 
            Vector4::new(4., 5., 6., 7.), 
            Vector4::new(7., 6., 5., 4.)
        );
        assert!(Matrix4x4::approx(m * Matrix4x4::identity(), m));
    }

    #[test]
    pub fn matrix_transform_mul_text() {
        let scale_expected = 10.;
        let translate = -30.;

        let s = Matrix4x4::scale(Vector3::all(scale_expected));
        let t = Matrix4x4::translation(Vector3::all(translate));

        let transform = s * t;

        let translate_expected = translate * scale_expected;
        assert_eq!(transform[3][0], translate_expected);
        assert_eq!(transform[3][1], translate_expected);
        assert_eq!(transform[3][2], translate_expected);
        assert_eq!(transform[3][3], 1.);

    }

    #[test]
    pub fn matrix_two_dimension_index_assignment() {
        let expected = 50.;
        let mut m = Matrix4x4::identity();
        m[3][2] = expected;

        assert_eq!(m[3][2], expected);
    }

    #[test]
    pub fn matrix_ortho_test() {
        let m = Matrix4x4::ortho(0., 512., 0., 512.);
        let point = Vector3::new(0., 512., 0.).to_vec4_as_point();

        let result = m * point;

        assert_eq!(result.x, -1.);
        assert_eq!(result.y, 1.);
        assert_eq!(result.z, 0.);
    }
}