mod math;
mod vector2;
mod vector3;
mod vector4;
mod matrix4x4;


pub use vector2::*;
pub use vector3::*;
pub use vector4::*;
pub use matrix4x4::*;

pub(crate) mod simd;
pub use math::*;

include!(concat!(env!("OUT_DIR"), "/vector.swizzling.gen.rs"));