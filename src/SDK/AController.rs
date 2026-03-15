use super::{
    ACharacter::ACharacter,
    APawn::APawn,
    APlayerState::APlayerState,
    Structs::{FRotator::FRotator, FVector::FVector},
};

#[struct_macro::inherit(super::AActor::AActor, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct AController {
    #[offset(0x0490)]
    pub PlayerState: *mut APlayerState,

    #[offset(0x04C8)]
    pub Pawn: *mut APawn,

    #[offset(0x04D8)]
    pub Character: *mut ACharacter,

    #[offset(0x0500)]
    pub ControlRotation: FRotator,
}

impl AController {
    #[inline]
    pub unsafe fn SetControlRotation(&self, NewRotation: FRotator,) {
        #[repr(C)]
        struct Params {
            NewRotation: FRotator,
        }

        let mut params = Params { NewRotation, };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/Engine.Controller.SetControlRotation'"),
                false,
            );
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }

    #[inline]
    pub unsafe fn GetControlRotation(&self,) -> FRotator {
        #[repr(C)]
        struct Params {
            pub ReturnValue: FRotator,
        }

        let mut params: Params = ::core::mem::zeroed();

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/Engine.Controller.GetControlRotation'"),
                false,
            );
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        params.ReturnValue
    }

    #[inline]
    pub unsafe fn SetIgnoreLookInput(&self, bNewLookInput: bool,) {
        #[repr(C)]
        struct Params {
            bNewLookInput: bool,
        }

        let mut params: Params = Params { bNewLookInput, };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/Engine.Controller.SetIgnoreLookInput'"),
                false,
            );
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }

    #[inline]
    pub unsafe fn GetPlayerViewPoint(&self, Location: *mut FVector, Rotation: *mut FRotator,) {
        #[repr(C)]
        struct Params {
            Location: *mut FVector,
            Rotation: *mut FRotator,
        }

        let mut params: Params = Params { Location, Rotation, };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance =
                super::StaticFindObject(0, -1, crate::w!("Function 'Engine.Controller.GetPlayerViewPoint'"), false,);
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }
}
