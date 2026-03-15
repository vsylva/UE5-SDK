use super::Structs::FVector::FVector;

#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct FMinimalViewInfo {
    pub location: FVector,
    pub rotation: super::Structs::FRotator::FRotator,
    pub fov:      f32,
}

impl FMinimalViewInfo {
    #[inline]
    pub const fn zero() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
