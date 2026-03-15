use ::core::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct FRotator {
    pub Pitch: f64,
    pub Yaw:   f64,
    pub Roll:  f64,
}

impl FRotator {
    #[inline]
    pub const fn zero() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}

impl Sub for FRotator {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self,) -> Self::Output {
        Self { Pitch: self.Pitch - other.Pitch, Yaw: self.Yaw - other.Yaw, Roll: self.Roll - other.Roll, }
    }
}

impl Div<f64,> for FRotator {
    type Output = Self;

    #[inline]
    fn div(self, scalar: f64,) -> Self::Output {
        Self { Pitch: self.Pitch / scalar, Yaw: self.Yaw / scalar, Roll: self.Roll / scalar, }
    }
}

impl Mul<f64,> for FRotator {
    type Output = Self;

    #[inline]
    fn mul(self, scalar: f64,) -> Self::Output {
        Self { Pitch: self.Pitch * scalar, Yaw: self.Yaw * scalar, Roll: self.Roll * scalar, }
    }
}

impl Mul for FRotator {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self,) -> Self::Output {
        Self { Pitch: self.Pitch * other.Pitch, Yaw: self.Yaw * other.Yaw, Roll: self.Roll * other.Roll, }
    }
}

impl Add for FRotator {
    type Output = Self;

    #[inline]
    fn add(self, other: Self,) -> Self::Output {
        Self { Pitch: self.Pitch + other.Pitch, Yaw: self.Yaw + other.Yaw, Roll: self.Roll + other.Roll, }
    }
}
