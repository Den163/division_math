use itertools::{iproduct, Itertools};
use std::fmt::Write;
use std::path::Path;
use std::process::Command;
use std::{env, fs};

#[derive(Clone)]
struct FuncArg {
    name: String,
    type_name: String,
}

const VEC_FIELDS: [&str; 4] = ["x", "y", "z", "w"];

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let swizzle_gen_path = Path::new(&out_dir).join("vector.swizzling.gen.rs");
    let math_gen_path = Path::new(&out_dir).join("vector.math.gen.rs");

    fs::write(&swizzle_gen_path, create_vector_fields_swizzles()).unwrap();
    fs::write(&math_gen_path, create_vec_math()).unwrap();

    let _ = Command::new("rustfmt")
        .arg(math_gen_path.to_str().unwrap())
        .arg(swizzle_gen_path.to_str().unwrap())
        .spawn();

    println!("cargo:rerun-if-changed=build.rs");
}

fn create_vector_fields_swizzles() -> String {
    let vec_dims = [2, 3, 4];
    let mut str = String::new();
    str.write_str("/*** This code was generated automatically ***/\n\n")
        .unwrap();

    for impl_dim in vec_dims {
        str.write_fmt(format_args!("impl Vector{} {{\n", impl_dim))
            .unwrap();

        for (f0, f1) in iproduct!(0..impl_dim, 0..impl_dim) {
            let fields = [f0, f1];

            str.write_str(fields_indices_to_function(&fields).as_str())
                .unwrap();
        }

        for (f0, f1, f2) in iproduct!(0..impl_dim, 0..impl_dim, 0..impl_dim) {
            let fields = [f0, f1, f2];

            str.write_str(fields_indices_to_function(&fields).as_str())
                .unwrap();
        }

        for (f0, f1, f2, f3) in iproduct!(0..impl_dim, 0..impl_dim, 0..impl_dim, 0..impl_dim) {
            let fields = [f0, f1, f2, f3];

            str.write_str(fields_indices_to_function(&fields).as_str())
                .unwrap();
        }

        str.write_str("}\n").unwrap();
    }

    str
}

fn fields_indices_to_function(fields: &[i32]) -> String {
    let vec_fields = ["x", "y", "z", "w"];

    let output_vec_size = fields.len();
    let func_name = fields
        .iter()
        .map(|f| vec_fields[*f as usize].to_string())
        .collect::<Vec<String>>()
        .join("");

    let args = fields
        .iter()
        .map(|f| format!("self.{}", vec_fields[*f as usize]))
        .collect::<Vec<String>>()
        .join(",");

    format_args!(
        "\t#[inline(always)] pub fn {func_name}(self) -> Vector{output_vec_size} 
        {{ Vector{output_vec_size}::new({args}) }}\n",
    )
    .to_string()
}

fn create_vec_math() -> String {
    let mut result = String::new();
    let vec_dims = [2, 3, 4];

    result
        .write_str("/*** This code was generated automatically ***/\n\n")
        .unwrap();

    for dim in vec_dims {
        let vec_type = format!("Vector{dim}");

        result
            .write_fmt(format_args!("impl {vec_type} {{\n\n"))
            .unwrap();
        result
            .write_str(
                create_expanded_math_func(
                    "lerp",
                    dim,
                    &[FuncArg {
                        name: "t".to_string(),
                        type_name: "f32".to_string(),
                    }],
                )
                .as_str(),
            )
            .unwrap();

        for s in [
            create_new_func(dim),
            create_all_func(dim),
            create_zero_func(dim),
            create_one_func(dim),
            create_length_funcs(dim),
            create_approx_func(dim),
            create_normalized_func(dim),
            create_ptr_funcs(dim),
            create_clamp_funcs(dim),
        ] {
            result.write_str(s.as_str()).unwrap();
        }

        result.write_str("\n}\n").unwrap();

        for s in [
            create_component_wise_vec_func_simd("Add", "+", dim),
            create_component_wise_vec_func_simd("Sub", "-", dim),
            create_component_wise_vec_func_simd("Mul", "*", dim),
            create_component_wise_vec_func_simd("Div", "/", dim),
            create_component_wise_vec_func("Rem", "%", dim),
            create_scalar_vec_func_simd("Mul", "*", dim),
            create_scalar_vec_func_simd("Div", "/", dim),
            create_scalar_vec_func("Rem", "%", dim),
            create_typed_op_assign_func("AddAssign", "add_assign", "+", "Self", dim),
            create_typed_op_assign_func("SubAssign", "sub_assign", "-", "Self", dim),
            create_op_assign_funcs("MulAssign", "mul_assign", "*", dim),
            create_op_assign_funcs("DivAssign", "div_assign", "/", dim),
            create_op_assign_funcs("RemAssign", "rem_assign", "%", dim),
            create_vec_index_func(dim),
            create_vec_index_mut_func(dim),
            create_neg_func(dim),
            create_from_tuple_impl(dim),
        ] {
            result.write_str(s.as_str()).unwrap();
        }
    }

    result
}

fn create_expanded_math_func(
    func_name: &str,
    vec_dimension: usize,
    additional_args: &[FuncArg],
) -> String {
    let vec_type = format!("Vector{vec_dimension}");

    let std_defs = format!("p0: Vector{vec_dimension}, p1: Vector{vec_dimension}");
    let func_call_args = VEC_FIELDS
        .iter()
        .take(vec_dimension)
        .map(|f| {
            format!(
                "{}({})",
                func_name,
                [format!("p0.{f}"), format!("p1.{f}")]
                    .into_iter()
                    .chain(additional_args.iter().map(|a| { a.name.clone() }))
                    .join(", ")
            )
        })
        .join(", ");

    let add_arg_defs = additional_args
        .iter()
        .map(|a| format!("{}: {}", a.name, a.type_name))
        .join(", ");

    let arg_defs = [std_defs, add_arg_defs].join(", ");

    format!(
        "#[inline] pub fn {func_name}({arg_defs}) -> {vec_type} 
        {{ {vec_type}::new({func_call_args}) }}\n"
    )
}

fn create_new_func(dim: usize) -> String {
    let params_list = VEC_FIELDS
        .iter()
        .take(dim)
        .map(|f| format!("{f}: f32"))
        .join(",");
    let args_list = VEC_FIELDS.iter().take(dim).join(",");

    format!("pub fn new({params_list}) -> Vector{dim} {{ Vector{dim} {{ {args_list} }} }}")
}

fn create_all_func(dim: usize) -> String {
    let args_list = (0..dim).map(|_| "value").join(",");

    format!(
        "#[inline] pub fn all(value: f32) -> Vector{dim} 
        {{ Vector{dim}::new({args_list}) }}\n",
    )
}

fn create_zero_func(dim: usize) -> String {
    format!("#[inline] pub fn zero() -> Vector{dim} {{ Vector{dim}::all(0f32) }}\n")
}

fn create_one_func(dim: usize) -> String {
    format!("#[inline] pub fn one() -> Vector{dim} {{ Vector{dim}::all(1f32) }}\n")
}

fn create_length_funcs(dim: usize) -> String {
    let formula = VEC_FIELDS
        .iter()
        .take(dim)
        .map(|f| format!("self.{f} * self.{f}"))
        .join("+");

    format!(
        "
        #[inline] pub fn length_sqr(self) -> f32 {{ {formula} }}
        #[inline] pub fn length(self) -> f32 {{ self.length_sqr().sqrt() }}
    "
    )
}

fn create_approx_func(vec_dimension: usize) -> String {
    let fields = VEC_FIELDS.iter().take(vec_dimension);
    let vec_type = format!("Vector{vec_dimension}");

    let approx_calls = fields
        .into_iter()
        .map(|f| format!("approx(x.{f}, y.{f})"))
        .join(" && ");

    format!(
        "#[inline] pub fn approx(x: {vec_type}, y: {vec_type}) -> bool 
        {{ {approx_calls} }}\n"
    )
}

fn create_normalized_func(dim: usize) -> String {
    format!("#[inline] pub fn normalized(self) -> Vector{dim} {{ self / self.length() }}")
}

fn create_ptr_funcs(dim: usize) -> String {
    format!(
        "
            #[inline]
            pub fn as_ptr(&self) -> *const f32 {{
                self as *const Vector{dim} as *const f32
            }}
            #[inline]
            pub fn as_mut_ptr(&mut self) -> *mut f32 {{
                self as *mut Vector{dim} as *mut f32
            }}
        "
    )
}

fn create_clamp_funcs(dim: usize) -> String {
    let ty = format!("Vector{dim}");
    let construction = VEC_FIELDS.iter().take(dim)
        .map(|f| format!("self.{f}.clamp(min.{f}, max.{f})"))
        .join(",");

    format!("
        #[inline]
        pub fn clamp(self, min: {ty}, max: {ty}) -> {ty} {{
            {ty}::new({construction})
        }}

        #[inline]
        pub fn clamp_length(self, min: f32, max: f32) -> {ty} {{
            let sqr_min = min * min;
            let sqr_max = max * max;

            let len_sqr = self.length_sqr();

            if len_sqr < sqr_min {{
                self.normalized() * sqr_min.sqrt()
            }} else if len_sqr > sqr_max {{
                self.normalized() * sqr_max.sqrt()
            }} else {{
                return self;
            }}
        }}
    ")
}

fn create_component_wise_vec_func_simd(
    operator_trait: &str,
    operator_token: &str,
    dim: usize,
) -> String {
    let simd_prefix = get_simd_prefix(dim);
    let func = create_component_wise_vec_func(operator_trait, operator_token, dim);

    format!(
        "
        {simd_prefix}
        {func}
        "
    )
}

fn create_component_wise_vec_func(
    operator_trait: &str,
    operator_token: &str,
    dim: usize,
) -> String {
    let func_name = operator_trait.to_lowercase();
    let new_vec_result_args = VEC_FIELDS
        .iter()
        .take(dim)
        .map(|f| format!("self.{f} {operator_token} rhs.{f}"))
        .join(",");

    format!(
        "
            impl std::ops::{operator_trait}<Vector{dim}> for Vector{dim} {{
                type Output = Vector{dim};

                #[inline]
                fn {func_name}(self, rhs: Vector{dim}) -> Vector{dim} 
                {{ Vector{dim}::new({new_vec_result_args}) }}
            }}
        "
    )
}

fn create_scalar_vec_func_simd(operator_trait: &str, operator_token: &str, dim: usize) -> String {
    let simd_prefix = get_simd_prefix(dim);
    let func = create_scalar_vec_func(operator_trait, operator_token, dim);

    format!(
        "
        {simd_prefix}
        {func}
        "
    )
}

fn create_scalar_vec_func(operator_trait: &str, operator_token: &str, dim: usize) -> String {
    let func_name = operator_trait.to_lowercase();
    let new_vec_result_args = VEC_FIELDS
        .iter()
        .take(dim)
        .map(|f| format!("self.{f} {operator_token} rhs"))
        .join(",");

    format!(
        "
            impl std::ops::{operator_trait}<f32> for Vector{dim} {{
                type Output = Vector{dim};

                #[inline]
                fn {func_name}(self, rhs: f32) -> Vector{dim} 
                {{ Vector{dim}::new({new_vec_result_args}) }}
            }}
        "
    )
}

fn get_simd_prefix(dim: usize) -> String {
    if dim == 4 {
        "#[cfg(not(feature = \"enable_simd\"))]".to_string()
    } else {
        String::new()
    }
}

fn create_vec_index_func(dim: usize) -> String {
    format!(
        "
            impl std::ops::Index<usize> for Vector{dim} {{
                type Output = f32;

                #[inline]
                fn index(&self, index: usize) -> &Self::Output {{
                    debug_assert!(index < {dim});

                    unsafe {{
                        let ptr = self as *const Vector{dim} as *const f32;
                        & *ptr.add(index)
                    }}
                }}
            }}
        "
    )
}

fn create_vec_index_mut_func(dim: usize) -> String {
    format!(
        "
            impl std::ops::IndexMut<usize> for Vector{dim} {{
                #[inline]
                fn index_mut(&mut self, index: usize) -> &mut Self::Output {{
                    assert!(index < {dim});

                    unsafe {{
                        let ptr = self as *mut Vector{dim} as *mut f32;
                        &mut *ptr.add(index)
                    }}
                }}
            }}
        "
    )
}

fn create_neg_func(dim: usize) -> String {
    let new_args = VEC_FIELDS
        .iter()
        .take(dim)
        .map(|f| format!("-self.{f}"))
        .join(",");

    format!(
        "
            impl std::ops::Neg for Vector{dim} {{
                type Output = Vector{dim};

                #[inline]
                fn neg(self) -> Self::Output {{
                    Vector{dim}::new({new_args})
                }}
            }}
        "
    )
}

fn create_op_assign_funcs(
    op_trait: &str,
    func_name: &str,
    op_token: &str,
    dim: usize,
) -> String {
    let vec_op_func = create_typed_op_assign_func(op_trait, func_name, op_token, "Self", dim);
    let scalar_op_func = create_typed_op_assign_func(op_trait, func_name, op_token, "f32", dim);

    format!(
        "
        {vec_op_func}
        {scalar_op_func}
        "
    )
}

fn create_typed_op_assign_func(
    op_trait: &str,
    func_name: &str,
    op_token: &str,
    rhs_type: &str,
    dim: usize,
) -> String {
    format!(
        "
        impl std::ops::{op_trait}<{rhs_type}> for Vector{dim} {{
            fn {func_name}(&mut self, rhs: {rhs_type}) {{
                *self = * self {op_token} rhs
            }}
        }}
    "
    )
}

fn create_from_tuple_impl(dim: usize) -> String {
    let tuple_type = format!("({})",(0..dim).map(|_| "f32").join(","));
    let fields = VEC_FIELDS.iter().take(dim).join(",");

    format!(
        "
            impl From<{tuple_type}> for Vector{dim} {{
                #[inline]
                fn from(({fields}): {tuple_type}) -> Vector{dim} {{
                    Vector{dim}::new({fields})
                }}
            }}
        "
    )
}