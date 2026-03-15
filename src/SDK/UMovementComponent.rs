use super::{
    FHitResult::FHitResult,
    Structs::{FRotator::FRotator, FVector::FVector},
};

#[struct_macro::inherit(super::UActorComponent::UActorComponent, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct UMovementComponent {
    #[offset(0x00D8)]
    pub Velocity: FVector,
}

impl UMovementComponent {
    #[inline]
    pub unsafe fn K2_MoveUpdatedComponent(
        &self,
        Delta: FVector,
        NewRotation: FRotator,
        OutHit: *mut FHitResult,
        bSweep: bool,
        bTeleport: bool,
    ) -> bool {
        #[repr(C)]
        struct Params {
            Delta:       FVector,
            NewRotation: FRotator,
            OutHit:      FHitResult,
            bSweep:      bool,
            bTeleport:   bool,
            ReturnValue: bool,
            Pad_13B:     [u8; 0x5],
        }

        let mut params = Params {
            Delta,
            NewRotation,
            OutHit: if !OutHit.is_null() { *OutHit } else { core::mem::zeroed() },
            bSweep,
            bTeleport,
            ReturnValue: false,
            ..::core::mem::zeroed()
        };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'Engine.MovementComponent.K2_MoveUpdatedComponent'"),
                false,
            );
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        return params.ReturnValue;
    }
}
