use super::{
    FActorInstanceHandle::FActorInstanceHandle,
    Structs::{FName::FName, FVector::FVector, TWeakObjectPtr::TWeakObjectPtr},
    UPrimitiveComponent::UPrimitiveComponent,
};

#[struct_macro::bitfields]
#[struct_macro::disinherit]
#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct FHitResult {
    #[offset(0x0000)]
    pub FaceIndex: i32,

    #[offset(0x0004)]
    pub Time: f32,

    #[offset(0x0008)]
    pub Distance: f32,

    #[offset(0x0010)]
    pub Location: FVector,

    #[offset(0x0028)]
    pub ImpactPoint: FVector,

    #[offset(0x0040)]
    pub Normal: FVector,

    #[offset(0x0058)]
    pub ImpactNormal: FVector,

    #[offset(0x0070)]
    pub TraceStart: FVector,

    #[offset(0x0088)]
    pub TraceEnd: FVector,

    #[offset(0x00A0)]
    pub PenetrationDepth: f32,

    #[offset(0x00AD)]
    #[bits(1, bBlockingHit)]
    pub bBlockingHit: bool,

    #[offset(0x00C0)]
    pub HitObjectHandle: FActorInstanceHandle,

    #[offset(0x00E0)]
    pub Component: TWeakObjectPtr<*mut UPrimitiveComponent,>,

    #[offset(0x00F8)]
    pub BoneName: FName,
}
