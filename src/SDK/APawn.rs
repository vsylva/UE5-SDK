use super::APlayerState::APlayerState;

#[struct_macro::inherit(super::AActor::AActor, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct APawn {
    #[offset(0x04A8)]
    pub PlayerState: *mut APlayerState,
}
