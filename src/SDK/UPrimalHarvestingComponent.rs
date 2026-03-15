use super::Structs::FString::FString;

#[struct_macro::inherit(super::UActorComponent::UActorComponent, Flags)]
#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct UPrimalHarvestingComponent {
    #[offset(0x0140)]
    pub DescriptiveName: FString,
}
