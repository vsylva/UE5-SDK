#[struct_macro::inherit(super::UObject::UObject, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct UPlayer {
    #[offset(0x0030)]
    pub PlayerController: *mut super::APlayerController::APlayerController,
}
