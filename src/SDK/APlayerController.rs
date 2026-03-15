use super::{
    AHUD::AHUD,
    APawn::APawn,
    APlayerCameraManager::APlayerCameraManager,
    Structs::{FVector::FVector, FVector2D::FVector2D},
    UObject::UObject,
};

#[struct_macro::inherit(super::APrimalController::APrimalController, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct APlayerController {
    #[offset(0x0540)]
    pub AcknowledgedPawn: *mut APawn,

    #[offset(0x0550)]
    pub MyHUD: *mut AHUD,

    #[offset(0x0558)]
    pub PlayerCameraManager: *mut APlayerCameraManager,

    #[offset(0x0620)]
    pub PlayerInput: usize,

    #[offset(0x0728)]
    pub RotationInput: super::Structs::FRotator::FRotator,
}

impl APlayerController {
    #[inline]
    pub unsafe fn ProjectWorldLocationToScreen(
        &self,
        world_location: FVector,
        screen_location: &mut FVector2D,
        b_player_viewport_relative: bool,
    ) -> bool {
        #[repr(C)]
        struct Params {
            world_location:             FVector,
            screen_location:            FVector2D,
            b_player_viewport_relative: bool,
            return_value:               bool,
            pad_2a:                     [u8; 6],
        }

        let mut params = Params {
            world_location: world_location,
            screen_location: FVector2D::zero(),
            b_player_viewport_relative,
            return_value: false,
            pad_2a: [0; 6],
        };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/Engine.PlayerController.ProjectWorldLocationToScreen'"),
                false,
            );
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        *screen_location = params.screen_location;

        params.return_value
    }

    #[inline]
    pub unsafe fn GetMousePosition(
        &self,
        LocationX: &mut f32,
        LocationY: &mut f32,
        bEvenWhenMouseNotAttached: bool,
    ) -> bool {
        #[repr(C)]
        struct Params {
            LocationX:                 f32,
            LocationY:                 f32,
            bEvenWhenMouseNotAttached: bool,
            ReturnValue:               bool,
            Pad_A:                     [u8; 0x2],
        }

        let mut params = Params { bEvenWhenMouseNotAttached, ..::core::mem::zeroed() };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/Engine.PlayerController.GetMousePosition'"),
                false,
            );
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        *LocationX = params.LocationX;
        *LocationY = params.LocationY;

        params.ReturnValue
    }

    #[inline]
    pub unsafe fn WasInputKeyJustPressed(&self, Key: super::Structs::FKey::FKey,) -> bool {
        #[repr(C)]
        struct Params {
            Key:         super::Structs::FKey::FKey,
            ReturnValue: bool,
            Pad_19:      [u8; 0x7],
        }

        let mut params = Params { Key, ..::core::mem::zeroed() };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/Engine.PlayerController.WasInputKeyJustPressed'"),
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
    pub unsafe fn IsInputKeyDown(&self, Key: super::Structs::FKey::FKey,) -> bool {
        #[repr(C)]
        struct Params {
            Key:         super::Structs::FKey::FKey,
            ReturnValue: bool,
            Pad_19:      [u8; 0x7],
        }

        let mut params = Params { Key, ..::core::mem::zeroed() };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance =
                super::StaticFindObject(0, -1, crate::w!("Function 'Engine.PlayerController.IsInputKeyDown'"), false,);
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
    pub unsafe fn ServerAcknowledgePossession(&self, P: *mut APawn,) {
        #[repr(C)]
        struct Params {
            P: *mut APawn,
        }

        let mut params = Params { P, };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'Engine.PlayerController.ServerAcknowledgePossession'"),
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
    pub unsafe fn ServerMultiUse(&self, ForObject: *mut UObject, UseIndex: i32,) {
        #[repr(C)]
        struct Params {
            ForObject: *mut UObject,
            UseIndex:  i32,
            _pad:      [u8; 4],
        }

        let mut params = Params { ForObject, UseIndex, ..::core::mem::zeroed() };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'ShooterGame.ShooterPlayerController.ServerMultiUse'"),
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
    pub unsafe fn SendToConsole(&self, Command: super::Structs::FString::FString,) {
        #[repr(C)]
        struct Params {
            Command: super::Structs::FString::FString,
        }

        let mut params = Params { Command, };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance =
                super::StaticFindObject(0, -1, crate::w!("Function 'Engine.PlayerController.SendToConsole'"), false,);
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }
}
