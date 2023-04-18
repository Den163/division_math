use std::{env, fs};
use std::fmt::{Write};
use std::path::Path;
use itertools::{iproduct, Itertools};

#[derive(Clone)]
struct FuncArg {
    name: String,
    type_name: String,
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let swizzle_gen_path = Path::new(&out_dir).join("vector.swizzling.gen.rs");
    let math_gen_path = Path::new(&out_dir).join("vector.math.gen.rs");

    fs::write(swizzle_gen_path, create_vector_fields_swizzles()).unwrap();
    fs::write(math_gen_path, create_vec_math()).unwrap();

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

fn create_vec_math() -> String {
    let mut result = String::new();
    let vec_dims = [2, 3, 4];

    for dim in vec_dims {
        let vec_type = format!("Vector{}", dim);


        result.write_fmt(format_args!("impl {} {{\n\n", vec_type)).unwrap();
        result.write_str(create_expanded_math_func(
            "lerp", dim,
            &[FuncArg { name: "t".to_string(), type_name: "f32".to_string() }],
        ).as_str()).unwrap();
        result.write_str(create_approx_func(dim).as_str()).unwrap();

        result.write_str("\n}\n").unwrap();
    }

    result
}

fn create_expanded_math_func(func_name: &str, vec_dimension: usize, additional_args: &[FuncArg]) -> String {
    let vec_type = format!("Vector{}", vec_dimension);
    let fields = ["x", "y", "z", "w"];

    let std_defs = format!("p0: Vector{}, p1: Vector{}", vec_dimension, vec_dimension);
    let func_call_args = fields.into_iter()
        .take(vec_dimension)
        .map(|f| {
            format!("{}({})",
                    func_name,
                    [format!("p0.{}", f), format!("p1.{}", f)].into_iter()
                        .chain(additional_args.iter().map(|a| { a.name.clone() }))
                        .join(", "))
        })
        .join(", ");

    let add_arg_defs = additional_args.iter()
        .map(|a| { format!("{}: {}", a.name, a.type_name) })
        .join(", ");

    let arg_defs = [std_defs, add_arg_defs].join(", ");

    format!(
        "#[inline] pub fn {}({}) -> {} {{ {}::new({}) }}\n",
        func_name, arg_defs, vec_type, vec_type, func_call_args
    )
}

fn create_approx_func(vec_dimension: usize) -> String {
    let fields = ["x", "y", "z", "w"].iter().take(vec_dimension);
    let vec_type = format!("Vector{}", vec_dimension);

    let approx_calls = fields.into_iter()
        .map(|f| { format!("approx(x.{}, y.{})", f, f) })
        .join(" && ");

    format!(
        "#[inline] pub fn approx(x: {}, y: {}) -> bool {{ {} }}\n",
        vec_type, vec_type, approx_calls
    )
}