use super::{Structs::FString::FString, UPrimalHarvestingComponent::UPrimalHarvestingComponent};

#[struct_macro::inherit(super::AActor::AActor, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct APrimalTargetableActor {
    #[offset(0x0538)]
    pub MyHarvestingComponent: *mut UPrimalHarvestingComponent,

    #[offset(0x0548)]
    pub DescriptiveName: FString,
}
