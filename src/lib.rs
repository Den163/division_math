mod vector2;
mod vector3;
mod vector4;
mod matrix4x4;

pub mod math;

pub use vector2::*;
pub use vector3::*;
pub use vector4::*;
pub use matrix4x4::*;

include!(concat!(env!("OUT_DIR"), "/vector.swizzling.gen.rs"));