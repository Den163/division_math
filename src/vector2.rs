use std::ops::{Index, IndexMut};

#[repr(C)]
#[derive(PartialEq, Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

impl Vector2 {

}

impl Index<usize> for Vector2 {
    type Output = f32;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 2);

        unsafe {
            let ptr = self as *const Vector2 as *const f32;
            ptr.add(index).as_ref().unwrap()
        }
    }
}

impl IndexMut<usize> for Vector2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < 2);

        unsafe {
            let ptr = self as *mut Vector2 as *mut f32;
            ptr.add(index).as_mut().unwrap()
        }
    }
}