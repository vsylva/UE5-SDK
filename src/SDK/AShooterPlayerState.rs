use super::FTribeData::FTribeData;

#[struct_macro::inherit(super::APlayerState::APlayerState, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct AShooterPlayerState {
    #[offset(0x0CE8)]
    pub MyTribeData: FTribeData,
}
