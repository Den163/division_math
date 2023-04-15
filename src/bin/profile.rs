use std::time;
use division_math::Vector3;

fn main() {
    let time_before = time::SystemTime::now();

    let acc = bench_fn();

    let time_diff = time_before.elapsed().unwrap();

    println!("Time past: {} ms", time_diff.as_secs_f32() * 1000f32);
    println!("result: {}", acc);
}

#[inline(never)]
fn bench_fn() -> Vector3 {
    let mut acc = Vector3::zero();
    for i in 0..((1000000000/4)*4) {
        let f = i as f32;
        let v1 = Vector3::new(f, f + 13f32, f + 17f32);
        let v2 = Vector3::new(f + 19f32, f + 23f32, f + 27f32);

        acc = acc + (v1 * v2) + (v1 / v2);
    }

    acc
}