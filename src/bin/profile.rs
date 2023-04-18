use std::time;
use division_math::{Matrix4x4, Vector3};

fn main() {
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

#[inline(never)]
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
    let time_before = time::SystemTime::now();
    let acc = get_mat_accum();
    let time_diff = time_before.elapsed().unwrap();

    println!("Matrix Benchmark. Time past: {} ms", time_diff.as_secs_f32() * 1000f32);
    println!("result: {:?}", acc);
}

fn get_mat_accum() -> Matrix4x4 {
    let m = Matrix4x4::identity();
    let m = m * 13.;

    m.inverse()
}

