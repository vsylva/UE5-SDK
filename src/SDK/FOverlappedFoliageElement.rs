use super::{
    AActor::AActor, Structs::FVector::FVector, UInstancedStaticMeshComponent::UInstancedStaticMeshComponent,
    UPrimalHarvestingComponent::UPrimalHarvestingComponent,
};

#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct FOverlappedFoliageElement {
    pub HarvestActor: *mut AActor,
    pub InstancedStaticMeshComponent: *mut UInstancedStaticMeshComponent,
    pub HarvestingComponent: *mut UPrimalHarvestingComponent,
    pub HarvestLocation: FVector,
    pub hitBodyIndex: i32,
    pub MaxHarvestHealth: f32,
    pub CurrentHarvestHealth: f32,
    pub bIsUnharvestable: u8,
    pub Pad_3D: [u8; 0x3],
}
