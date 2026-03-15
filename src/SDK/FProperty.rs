#[struct_macro::inherit(super::FField::FField, ObjFlags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct FProperty {
    #[offset(0x0030)]
    pub ArrayDim:      i32,
    pub ElementSize:   i32,
    pub PropertyFlags: u64,
    Pad_40:            [u8; 0x4],
    pub Offset:        i32,
    Pad_48:            [u8; 0x28],
}
