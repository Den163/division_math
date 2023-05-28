mod tests {
    use division_math::{Matrix4x4, Vector4, Vector3};

    #[test]
    pub fn matrix_inverse_test() {
        let m = Matrix4x4::identity() * 2.;
        let m_i = m.inverse();

        assert!(Matrix4x4::approx(m * m_i, Matrix4x4::identity()));
    }

    #[test]
    pub fn matrix_ortho_test() {
        let m = Matrix4x4::ortho(0., 512., 0., 512., 0., 1.);
        let point = Vector3::new(0., 512., 0.).to_vec4_as_point();

        let result = point * m;

        assert_eq!(result.x, -1.);
        assert_eq!(result.y, 1.);
        assert_eq!(result.z, 0.);
    }
}