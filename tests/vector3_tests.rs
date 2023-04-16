mod tests {
    use division_math::{Vector3};

    #[test]
    fn vector_to_scalar_mul() {
        let v = Vector3::one() * 2f32;
        assert_eq!(v.x, 2f32);
        assert_eq!(v.y, 2f32);
        assert_eq!(v.z, 2f32);
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

        assert!((norm.magnitude() - 1f32).abs() <= f32::EPSILON);
    }

    #[test]
    fn vector_cross() {
        let right = Vector3::right();
        let up = Vector3::up();

        assert_eq!(Vector3::cross(right, up), Vector3::forward());
    }
}