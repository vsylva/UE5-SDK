use super::FMinimalViewInfo::FMinimalViewInfo;

#[struct_macro::disinherit]
#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct FCameraCacheEntry {
    #[offset(0x0010)]
    pub POV: FMinimalViewInfo,
}

impl FCameraCacheEntry {
    #[inline]
    pub const fn zero() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
