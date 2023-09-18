mod tests {
    use std::time;
    use division_math::{Matrix4x4, Vector3, Vector4};

    #[test]
    fn main_bench() {
        bench_vec();
        bench_mat();
    }

    fn bench_vec() {
        let vec_set = (0..10000000).map(|_i| {
            Vector4::new(get_random_float(), get_random_float(),
                         get_random_float(), get_random_float())
        }).collect();

        let time_before = time::SystemTime::now();
        let acc = get_vec_accum(vec_set);
        let time_diff = time_before.elapsed().unwrap();

        println!("Vector Benchmark. Time past: {} ms", time_diff.as_secs_f32() * 1000f32);
        println!("result: {:?}", acc);
    }

    #[no_mangle]
    fn get_vec_accum(set: Vec<Vector4>) -> Vector4 {
        let mut acc = Vector4::zero();

        for i in 0..set.len() - 4 {
            acc = acc + (set[i] / set[i + 1]) + (set[i+2] / set[i+3].w) - set[i];
        }

        acc
    }

    fn bench_mat() {
        let set = (0..10000000).map(|_i| {
            Matrix4x4::scale(
                Vector3::new(get_random_float(), get_random_float(), get_random_float())
            )
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