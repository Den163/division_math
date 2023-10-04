use std::fmt::Debug;

#[repr(C)]
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}