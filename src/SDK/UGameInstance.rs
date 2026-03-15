#[struct_macro::inherit(super::UObject::UObject, Flags)]
#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct UGameInstance {
    #[offset(0x38)]
    pub LocalPlayers: super::Structs::TArray::TArray<*mut super::ULocalPlayer::ULocalPlayer,>,
}
