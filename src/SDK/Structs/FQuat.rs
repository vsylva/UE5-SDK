#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct FQuat {
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
    pub W: f64,
}

impl FQuat {
    #[inline]
    pub const fn zero() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
