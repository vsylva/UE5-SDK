use super::{FRepMovement::FRepMovement, Structs::FVector::FVector};

#[struct_macro::inherit(super::UPrimalActor::UPrimalActor, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct AActor {
    #[offset(0x02A8)]
    pub ReplicatedMovement: FRepMovement,

    #[offset(0x0398)]
    pub RootComponent: *mut super::USceneComponent::USceneComponent,
}

impl AActor {
    #[inline]
    pub unsafe fn K2_GetActorLocation(&self,) -> FVector {
        #[repr(C)]
        struct Params {
            return_value: FVector,
        }

        let mut params = ::core::mem::zeroed::<Params,>();

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/Engine.Actor.K2_GetActorLocation'"),
                false,
            );
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        params.return_value
    }

    #[inline]
    pub unsafe fn SetReplicateMovement(&self, bInReplicateMovement: bool,) {
        #[repr(C)]
        struct Params {
            bInReplicateMovement: bool,
        }

        let mut params = Params { bInReplicateMovement, };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance =
                super::StaticFindObject(0, -1, crate::w!("Function 'Engine.Actor.SetReplicateMovement'"), false,);
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }

    #[inline]
    pub unsafe fn SetActorEnableCollision(&self, bNewActorEnableCollision: bool,) {
        #[repr(C)]
        struct Params {
            bNewActorEnableCollision: bool,
        }

        let mut params = Params { bNewActorEnableCollision, };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance =
                super::StaticFindObject(0, -1, crate::w!("Function 'Engine.Actor.SetActorEnableCollision'"), false,);
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }
}
