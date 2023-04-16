use std::ops::{Index, IndexMut};

#[repr(C)]
#[derive(PartialEq, Copy, Clone)]
pub struct Vector2 {
    pub xy: [f32;2]
}

impl Vector2 {

}

impl Index<usize> for Vector2 {
    type Output = f32;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output { &self.xy[index] }
}

impl IndexMut<usize> for Vector2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.xy[index] }
}