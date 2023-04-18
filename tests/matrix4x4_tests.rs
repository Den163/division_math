mod tests {
    use division_math::{Matrix4x4};

    #[test]
    pub fn matrix_inverse_test() {
        let m = Matrix4x4::identity() * 2.;
        let m_i = m.inverse();

        assert!(Matrix4x4::approx(m * m_i, Matrix4x4::identity()));
    }
}