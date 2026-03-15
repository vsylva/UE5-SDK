use super::{
    FCameraCacheEntry::FCameraCacheEntry,
    Structs::{FRotator::FRotator, FVector::FVector},
};

#[struct_macro::inherit(super::AActor::AActor, Flags)]
#[repr(C)]
pub struct APlayerCameraManager {
    #[offset(0x1850)]
    pub CameraCachePrivate: FCameraCacheEntry,
}

impl APlayerCameraManager {
    #[inline]
    pub unsafe fn GetCameraLocation(&self,) -> FVector {
        #[repr(C)]
        struct Params {
            pub ReturnValue: FVector,
        }

        let mut params: Params = ::core::mem::zeroed();

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/Engine.PlayerCameraManager.GetCameraLocation'"),
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
    pub unsafe fn GetCameraRotation(&self,) -> FRotator {
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
                crate::w!("Function '/Script/Engine.PlayerCameraManager.GetCameraRotation'"),
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
    pub unsafe fn StopAllCameraShakes(&self, bImmediately: bool,) {
        #[repr(C)]
        struct Params {
            bImmediately: bool,
        }

        let mut params: Params = Params { bImmediately, };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/Engine.PlayerCameraManager.StopAllCameraShakes'"),
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
    pub unsafe fn StopAllCameraAnims(&self, bImmediately: bool,) {
        #[repr(C)]
        struct Params {
            bImmediately: bool,
        }

        let mut params: Params = Params { bImmediately, };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/Engine.PlayerCameraManager.StopAllCameraAnims'"),
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
    pub unsafe fn ClearCameraLensEffects(&self,) {
        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/Engine.PlayerCameraManager.ClearCameraLensEffects'"),
                false,
            );
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            0 as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }

    #[inline]
    pub unsafe fn StopCameraFade(&self,) {
        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/Engine.PlayerCameraManager.StopCameraFade'"),
                false,
            );
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            0 as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }
}
