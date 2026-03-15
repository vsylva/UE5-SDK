#[struct_macro::inherit(super::AGameState::AGameState, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct AShooterGameState {
    #[offset(0x0598)]
    pub NumPlayerConnected: i32,

    #[offset(0x05A8)]
    pub ServerFramerate: f32,

    #[offset(0x05C8)]
    pub NetworkTime: f64,

    #[offset(0x05EC)]
    pub NumTamedDinos: i32,

    #[offset(0x0740)]
    pub PrivateNetworkTime: f64,
    pub LastServerSaveTime: f64,
    pub ServerSaveInterval: f32,
}
