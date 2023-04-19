mod tests {
    use std::time;
    use division_math::{Matrix4x4, Vector3, Vector4};

    #[test]
    fn main_bench() {
        bench_vec();
        bench_mat();
    }

    fn bench_vec() {
        let time_before = time::SystemTime::now();
        let acc = get_vec_accum();
        let time_diff = time_before.elapsed().unwrap();

        println!("Vector Benchmark. Time past: {} ms", time_diff.as_secs_f32() * 1000f32);
        println!("result: {:?}", acc);
    }

    #[no_mangle]
    fn get_vec_accum() -> Vector3 {
        let mut acc = Vector3::zero();
        for i in 0..((1000000000/4)*4) {
            let f = i as f32;
            let v1 = Vector3::new(f, f + 13f32, f + 17f32);
            let v2 = Vector3::new(f + 19f32, f + 23f32, f + 27f32);

            acc = acc + (v1 * v2) + (v1 / v2);
        }

        acc
    }

    fn bench_mat() {
        let set = (0..10000000).map(|_i| {
            Matrix4x4::scale(Vector4::from_xyz_w(
                Vector3::new(get_random_float(), get_random_float(), get_random_float()),
                1.
            ))
        }).collect();

        let time_before = time::SystemTime::now();
        let acc = get_mat_accum(set);
        let time_diff = time_before.elapsed().unwrap();

        println!("Matrix Benchmark. Time past: {} ms", time_diff.as_secs_f32() * 1000f32);
        println!("result: {:?}", acc);
    }

    fn get_random_float() -> f32 {
        (rand::random::<f32>() + 1.) * 10000.
    }

    #[no_mangle]
    fn get_mat_accum(set: Vec<Matrix4x4>) -> Matrix4x4 {
        let mut m = Matrix4x4::identity();

        for el in set {
            m = el * m * el.inverse();
        }

        m
    }
}