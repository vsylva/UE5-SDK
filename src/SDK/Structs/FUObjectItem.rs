#[repr(C)]
pub struct FUObjectItem {
    pub Object:       *mut ::core::ffi::c_void,
    pub Flags:        i32,
    pub ClusterIndex: i32,
    pub SerialNumber: i32,
    Pad_14:           [u8; 4],
}

impl FUObjectItem {
    #[inline]
    pub const fn zero() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
