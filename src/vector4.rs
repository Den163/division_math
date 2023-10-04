#[derive(PartialEq, Copy, Clone, Debug)]
#[repr(C)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    #[inline]
    pub fn dot(l: Vector4, r: Vector4) -> f32 {
        l.x * r.x + l.y * r.y + l.z * r.z + l.w * r.w
    }

    #[inline(always)]
    pub fn r(self) -> f32 {
        self.x
    }
    #[inline(always)]
    pub fn g(self) -> f32 {
        self.y
    }
    #[inline(always)]
    pub fn b(self) -> f32 {
        self.z
    }
    #[inline(always)]
    pub fn a(self) -> f32 {
        self.w
    }
}

#[cfg(feature = "enable_simd")]
impl std::ops::Add<Vector4> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn add(self, rhs: Vector4) -> Vector4 {
        Vector4::add_simd(self, rhs)
    }
}

#[cfg(feature = "enable_simd")]
impl std::ops::Sub<Vector4> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn sub(self, rhs: Vector4) -> Vector4 {
        Vector4::sub_simd(self, rhs)
    }
}

#[cfg(feature = "enable_simd")]
impl std::ops::Mul<Vector4> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn mul(self, rhs: Vector4) -> Vector4 {
        Vector4::mul_simd(self, rhs)
    }
}

#[cfg(feature = "enable_simd")]
impl std::ops::Mul<f32> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Vector4::mul_scalar_simd(self, rhs)
    }
}

#[cfg(feature = "enable_simd")]
impl std::ops::Div<Vector4> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn div(self, rhs: Vector4) -> Self::Output {
        Vector4::div_simd(self, rhs)
    }
}

#[cfg(feature = "enable_simd")]
impl std::ops::Div<f32> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        Vector4::div_scalar_simd(self, rhs)
    }
}