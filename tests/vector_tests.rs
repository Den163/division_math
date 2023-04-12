mod tests {
    use division_math::Vector3;

    #[test]
    fn vector_to_scalar_mul() {
        let v = Vector3::one() * 2f32;

        assert_eq!(v.x, 2f32);
        assert_eq!(v.y, 2f32);
        assert_eq!(v.z, 2f32);
    }

    #[test]
    fn vector_to_vector_mul() {
        let v1 = Vector3::all(3f32);
        let v2 = Vector3::all(2f32);

        let res = v1 * v2;
        assert_eq!(res, Vector3::all(6f32));
    }
}