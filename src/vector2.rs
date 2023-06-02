use std::fmt::{Debug};
use std::ops::{Index, IndexMut, Mul};

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

impl Index<usize> for Vector2 {
    type Output = f32;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 2);

        unsafe {
            let ptr = self as *const Vector2 as *const f32;
            & *ptr.add(index)
        }
    }
}

impl IndexMut<usize> for Vector2 {

    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < 2);

        unsafe {
            let ptr = self as *mut Vector2 as *mut f32;
            &mut *ptr.add(index)
        }
    }
}

impl Mul<Vector2> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Self) -> Self::Output { Vector2::new(self.x * rhs.x, self.y * rhs.y) }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Self::Output { Vector2::new(self.x * rhs, self.y * rhs) }
}