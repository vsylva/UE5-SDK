use super::AWorldSettings::AWorldSettings;

#[struct_macro::inherit(super::UObject::UObject, Flags)]
#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct ULevel {
    #[offset(0x00A0)]
    pub Actors: super::Structs::TArray::TArray<*mut super::AActor::AActor,>,

    #[offset(0x0390)]
    pub WorldSettings: AWorldSettings,
}
