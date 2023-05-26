#[cfg(all(feature = "enable_simd", target_feature = "neon"))]
pub(crate) mod neon;