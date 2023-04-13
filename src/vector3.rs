use std::ops::{Add, Div, Index, Mul, Neg, Sub};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vector3 {
    data: [f32;3],
}

impl Vector3 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 { Vector3 { data: [x,y,z] } }
    #[inline]
    pub fn all(v: f32) -> Vector3 { Vector3 { data: [v,v,v] } }

    #[inline]
    pub fn zero() -> Vector3 { Vector3::all(0f32) }
    #[inline]
    pub fn one() -> Vector3 { Vector3::all(1f32) }
    #[inline]
    pub fn forward() -> Vector3 { Vector3 { data: [0f32, 0f32, 1f32] } }
    #[inline]
    pub fn right() -> Vector3 { Vector3 { data: [1f32, 0f32, 0f32] } }
    #[inline]
    pub fn up() -> Vector3 { Vector3 { data: [0f32, 1f32, 0f32] } }

    #[inline]
    pub fn cross(l: Vector3, r: Vector3) -> Vector3 { (l * r.yzx() - l.yzx() * r).yzx() }
    #[inline]
    pub fn dot(l: Vector3, r: Vector3) -> f32 { l.x() * r.x() + l.y() * r.y() + l.z() * r.z() }

    #[inline(always)]
    pub fn x(self) -> f32 { self.data[0] }
    #[inline(always)]
    pub fn y(self) -> f32 { self.data[1] }
    #[inline(always)]
    pub fn z(self) -> f32 { self.data[2] }

    #[inline]
    pub fn zyx(self) -> Vector3 { Vector3::new(self.z(), self.y(), self.x()) }
    #[inline]
    pub fn zxy(self) -> Vector3 { Vector3::new(self.z(), self.x(), self.y()) }
    #[inline]
    pub fn yzx(self) -> Vector3 { Vector3::new(self.y(), self.z(), self.x()) }
    #[inline]
    pub fn yxz(self) -> Vector3 { Vector3::new(self.y(), self.x(), self.z()) }
    #[inline]
    pub fn xzy(self) -> Vector3 { Vector3::new(self.x(), self.z(), self.y()) }
    #[inline]
    pub fn xyz(self) -> Vector3 { Vector3::new(self.x(), self.y(), self.z()) }

    #[inline]
    pub fn normalized(self) -> Vector3 { self * (1f32 / Vector3::dot(self, self).sqrt())  }
    #[inline]
    pub fn magnitude(self) -> f32 { Vector3::dot(self, self).sqrt() }
    #[inline]
    pub fn magnitude_sqr(self) -> f32 { Vector3::dot(self, self) }
}


impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 { data: [self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()] }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Vector3 { data: [self.x() * rhs, self.y() * rhs, self.z() * rhs ] }
    }
}

impl Div<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn div(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z())
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Vector3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    #[inline]
    fn neg(self) -> Self::Output {
        Vector3::new(-self.x(), -self.y(), -self.z())
    }
}

impl Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 3);

        &self.data[index]
    }
}