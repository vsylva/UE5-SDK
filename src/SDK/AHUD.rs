#[struct_macro::inherit(super::AActor::AActor, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct AHUD {
    #[offset(0x04E0)]
    pub Canvas: *mut super::UCanvas::UCanvas,
}
