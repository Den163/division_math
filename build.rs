use std::{env, fs};
use std::fmt::{Write};
use std::path::Path;
use itertools::{iproduct};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("vector_swizzles.gen.rs");

    fs::write(dest_path, create_vector_fields_swizzles()).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}

fn create_vector_fields_swizzles() -> String {
    let vec_dims = [2, 3, 4];
    let mut str = String::new();

    for impl_dim in vec_dims {
        str.write_fmt(format_args!("impl Vector{} {{\n", impl_dim)).unwrap();

        for (f0, f1) in iproduct!(0..impl_dim, 0..impl_dim) {
            let fields = [f0, f1];

            str.write_str(fields_indices_to_function(&fields).as_str()).unwrap();
        }

        for (f0, f1, f2) in iproduct!(0..impl_dim, 0..impl_dim, 0..impl_dim) {
            let fields = [f0, f1, f2];

            str.write_str(fields_indices_to_function(&fields).as_str()).unwrap();
        }

        for (f0, f1, f2, f3) in iproduct!(0..impl_dim, 0..impl_dim, 0..impl_dim, 0..impl_dim) {
            let fields = [f0, f1, f2, f3];

            str.write_str(fields_indices_to_function(&fields).as_str()).unwrap();
        }

        str.write_str("}\n").unwrap();
    }

    str
}

fn fields_indices_to_function(fields: &[i32]) -> String {
    let vec_fields = ["x", "y", "z", "w"];

    let output_vec_size = fields.len();
    let func_name = fields.iter()
        .map(|f| { vec_fields[*f as usize].to_string() })
        .collect::<Vec<String>>()
        .join("");

    let args = fields.iter()
        .map(|f| { format!("self.{}", vec_fields[*f as usize]) })
        .collect::<Vec<String>>()
        .join(",");

    format_args!(
        "\t#[inline(always)] pub fn {}(self) -> Vector{} {{ Vector{}::new({}) }}\n",
        func_name, output_vec_size, output_vec_size, args
    ).to_string()
}