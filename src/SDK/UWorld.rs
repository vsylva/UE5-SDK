#[struct_macro::inherit(super::UObject::UObject, Flags)]
#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct UWorld {
    #[offset(0x02F8)]
    pub PersistentLevel: *mut super::ULevel::ULevel,

    #[offset(0x0300)]
    pub NetDriver: *mut super::UNetDriver::UNetDriver,

    #[offset(0x0428)]
    pub GameState: *mut super::AGameStateBase::AGameStateBase,

    #[offset(0x0440)]
    pub Levels: super::Structs::TArray::TArray<*mut super::ULevel::ULevel,>,

    #[offset(0x04A0)]
    pub OwningGameInstance: *mut super::UGameInstance::UGameInstance,

    #[offset(0x0990)]
    pub TimeSeconds: f64,
}

impl UWorld {
    #[inline(always)]
    pub unsafe fn get() -> *mut Self {
        *super::Globals::GworldPP
    }
}
