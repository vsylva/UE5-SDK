use core::ops::{Add, Div, Mul, Sub};

use ::core::ops::{DivAssign, MulAssign};

#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct FVector2D {
    pub X: f64,
    pub Y: f64,
}

impl FVector2D {
    #[inline]
    pub const fn zero() -> Self {
        unsafe { ::core::mem::zeroed() }
    }

    #[inline]
    pub const fn half() -> Self {
        Self { X: 0.5, Y: 0.5, }
    }

    #[inline]
    pub const fn identity() -> Self {
        Self { X: 1.0, Y: 1.0, }
    }

    #[inline]
    pub const fn dot6() -> Self {
        Self { X: 0.6, Y: 0.6, }
    }

    #[inline]
    pub const fn dot7() -> Self {
        Self { X: 0.7, Y: 0.7, }
    }
}

impl Add for FVector2D {
    type Output = Self;
    fn add(self, other: Self,) -> Self {
        Self { X: self.X + other.X, Y: self.Y + other.Y, }
    }
}

impl Sub for FVector2D {
    type Output = Self;
    fn sub(self, other: Self,) -> Self {
        Self { X: self.X - other.X, Y: self.Y - other.Y, }
    }
}

impl Mul for FVector2D {
    type Output = Self;
    fn mul(self, other: Self,) -> Self {
        Self { X: self.X * other.X, Y: self.Y * other.Y, }
    }
}

impl Mul<f64,> for FVector2D {
    type Output = Self;
    fn mul(self, scalar: f64,) -> Self {
        Self { X: self.X * scalar, Y: self.Y * scalar, }
    }
}

impl Div for FVector2D {
    type Output = Self;
    fn div(self, other: Self,) -> Self {
        Self { X: self.X / other.X, Y: self.Y / other.Y, }
    }
}

impl Div<f64,> for FVector2D {
    type Output = Self;
    fn div(self, scalar: f64,) -> Self {
        Self { X: self.X / scalar, Y: self.Y / scalar, }
    }
}

impl MulAssign<f64,> for FVector2D {
    #[inline]
    fn mul_assign(&mut self, scalar: f64,) {
        self.X *= scalar;
        self.Y *= scalar;
    }
}

impl DivAssign<f64,> for FVector2D {
    #[inline]
    fn div_assign(&mut self, scalar: f64,) {
        self.X /= scalar;
        self.Y /= scalar;
    }
}
