use ::core::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct FVector {
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
}

impl FVector {
    #[inline]
    pub const fn is_empty(&self,) -> bool {
        self.X == 0.0 && self.Y == 0.0 && self.Z == 0.0
    }

    #[inline]
    pub const fn zero() -> Self {
        Self { X: 0.0, Y: 0.0, Z: 0.0, }
    }

    #[inline]
    pub unsafe fn size(&self,) -> f64 {
        self.X * self.X + self.Y * self.Y + self.Z * self.Z
    }

    #[inline]
    pub unsafe fn magnitude(&self,) -> f64 {
        self.size().sqrt()
    }
}

impl Add for FVector {
    type Output = Self;

    #[inline]
    fn add(self, other: Self,) -> Self {
        Self { X: self.X + other.X, Y: self.Y + other.Y, Z: self.Z + other.Z, }
    }
}

impl Sub for FVector {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self,) -> Self {
        Self { X: self.X - other.X, Y: self.Y - other.Y, Z: self.Z - other.Z, }
    }
}

impl Mul<f64,> for FVector {
    type Output = Self;

    #[inline]
    fn mul(self, scalar: f64,) -> Self {
        Self { X: self.X * scalar, Y: self.Y * scalar, Z: self.Z * scalar, }
    }
}

impl Mul for FVector {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self,) -> Self {
        Self { X: self.X * other.X, Y: self.Y * other.Y, Z: self.Z * other.Z, }
    }
}

impl Div<f64,> for FVector {
    type Output = Self;

    #[inline]
    fn div(self, scalar: f64,) -> Self {
        let inv_scalar = 1.0 / scalar;
        Self { X: self.X * inv_scalar, Y: self.Y * inv_scalar, Z: self.Z * inv_scalar, }
    }
}

impl AddAssign for FVector {
    #[inline]
    fn add_assign(&mut self, other: Self,) {
        self.X += other.X;
        self.Y += other.Y;
        self.Z += other.Z;
    }
}
