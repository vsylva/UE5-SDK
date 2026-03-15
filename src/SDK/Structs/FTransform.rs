#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct FTransform {
    pub Rotation:    super::FQuat::FQuat,
    pub Translation: super::FVector::FVector,
    Pad_38:          [u8; 0x8],
    pub Scale3D:     super::FVector::FVector,
    Pad_58:          [u8; 0x8],
}

impl FTransform {
    #[inline]
    pub const fn zero() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
