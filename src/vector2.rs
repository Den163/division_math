use std::fmt::Debug;

#[repr(C)]
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

impl Vector2 {
    #[inline]
    pub fn new(x: f32, y: f32) -> Vector2 { Vector2 {x, y} }
}