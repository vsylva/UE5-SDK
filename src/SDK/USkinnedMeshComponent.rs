use super::{
    ProcessEvent,
    Structs::{FName::FName, FRotator::FRotator, FVector::FVector},
};

#[struct_macro::inherit(super::UMeshComponent::UMeshComponent, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct USkinnedMeshComponent {}

impl USkinnedMeshComponent {
    #[inline]
    pub unsafe fn TransformFromBoneSpace(
        &self,
        BoneName: FName,
        InPosition: FVector,
        InRotation: FRotator,
        OutPosition: &mut FVector,
        OutRotation: &mut FRotator,
    ) {
        #[repr(C)]
        struct Params {
            BoneName:    FName,
            InPosition:  FVector,
            InRotation:  FRotator,
            OutPosition: FVector,
            OutRotation: FRotator,
        }

        let mut params = Params { BoneName, InPosition, InRotation, ..::core::mem::zeroed() };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();
        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'Engine.SkinnedMeshComponent.TransformFromBoneSpace'"),
                false,
            );
        }

        ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        *OutPosition = params.OutPosition;
        *OutRotation = params.OutRotation;
    }
}
