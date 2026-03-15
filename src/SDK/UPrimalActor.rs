use super::Structs::FString::FString;

#[doc = "ActorType 在 DesiredRepGraphBehavior 下面 +0x3 的位置"]
#[struct_macro::bitfields]
#[struct_macro::inherit(super::UObject::UObject, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct UPrimalActor {
    #[offset(0x01A0)]
    pub TargetingTeam: i32,

    #[offset(0x01D8)]
    #[bits(7, ActorType)]
    pub IsShooterPlayerController:  bool,
    pub IsPrimalDinoCharacter:      bool,
    pub IsShooterCharacter:         bool,
    pub IsPrimalCharacter:          bool,
    pub IsPrimalStructure:          bool,
    pub IsPrimalStructureExplosive: bool,
    pub IsInstancedFoliage:         bool,
}

impl UPrimalActor {
    #[inline]
    pub const unsafe fn is_friendly(&self, team_id: i32,) -> bool {
        self.TargetingTeam == team_id
    }

    #[inline]
    pub unsafe fn IsDead(&self,) -> bool {
        #[repr(C)]
        struct Params {
            return_value: bool,
        }

        let mut params = ::core::mem::zeroed::<Params,>();

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(0, -1, crate::w!("Function 'Engine.PrimalActor.IsDead'"), false,);
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
    pub unsafe fn IsInstancedFoliage(&self,) -> bool {
        #[repr(C)]
        struct Params {
            return_value: bool,
        }

        let mut params = ::core::mem::zeroed::<Params,>();

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance =
                super::StaticFindObject(0, -1, crate::w!("Function 'Engine.PrimalActor.IsInstancedFoliage'"), false,);
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
    pub unsafe fn GetCharacterController(&self,) -> *mut super::AShooterPlayerController::AShooterPlayerController {
        #[repr(C)]
        struct Params {
            return_value: *mut super::AShooterPlayerController::AShooterPlayerController,
        }

        let mut params = ::core::mem::zeroed::<Params,>();

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'Engine.PrimalActor.GetCharacterController'"),
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
    pub unsafe fn IsPrimalDino(&self,) -> bool {
        #[repr(C)]
        struct Params {
            return_value: bool,
        }

        let mut params = ::core::mem::zeroed::<Params,>();

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance =
                super::StaticFindObject(0, -1, crate::w!("Function '/Script/Engine.PrimalActor.IsPrimalDino'"), false,);
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
    pub unsafe fn IsShooterCharacter(&self,) -> bool {
        #[repr(C)]
        struct Params {
            ReturnValue: bool,
        }

        let mut params = ::core::mem::zeroed::<Params,>();

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/Engine.PrimalActor.IsShooterCharacter'"),
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
    pub unsafe fn GetDinoDescriptiveName(&self, IgnoreArticle: bool, IncludeDetails: bool,) -> FString {
        #[repr(C)]
        pub struct Params {
            pub ignore_article:  bool,
            pub include_details: bool,
            _pad_2:              [u8; 6],
            pub return_value:    FString,
        }

        let mut params = ::core::mem::zeroed::<Params,>();

        params.ignore_article = IgnoreArticle;
        params.include_details = IncludeDetails;

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/ShooterGame.PrimalDinoCharacter.GetDinoDescriptiveName'"),
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
    pub unsafe fn GetDistanceTo(&self, OtherActor: *mut super::AActor::AActor,) -> f32 {
        #[repr(C)]
        struct Params {
            OtherActor:   *mut super::AActor::AActor,
            return_value: f32,
            Pad_C:        [u8; 0x4],
        }

        let mut params = ::core::mem::zeroed::<Params,>();

        params.OtherActor = OtherActor;

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance =
                super::StaticFindObject(0, -1, crate::w!("Function '/Script/Engine.Actor.GetDistanceTo'"), false,);
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
    pub unsafe fn IsPrimalStructureItemContainer(&self,) -> bool {
        #[repr(C)]
        struct Params {
            ReturnValue: bool,
        }

        let mut params = ::core::mem::zeroed::<Params,>();

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/Engine.PrimalActor.IsPrimalStructureItemContainer'"),
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
    pub unsafe fn IsPrimalStructureExplosive(&self,) -> bool {
        #[repr(C)]
        struct Params {
            ReturnValue: bool,
        }

        let mut params = ::core::mem::zeroed::<Params,>();

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/Engine.PrimalActor.IsPrimalStructureExplosive'"),
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
    pub unsafe fn IsPrimalStructureTurret(&self,) -> bool {
        #[repr(C)]
        struct Params {
            ReturnValue: bool,
        }

        let mut params = ::core::mem::zeroed::<Params,>();

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/Engine.PrimalActor.IsPrimalStructureTurret'"),
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
    pub unsafe fn IsStandingMountedTurret(&self,) -> bool {
        #[repr(C)]
        struct Params {
            ReturnValue: bool,
        }

        let mut params = ::core::mem::zeroed::<Params,>();

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/Engine.PrimalActor.IsStandingMountedTurret'"),
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
    pub unsafe fn IsFemale(&self,) -> bool {
        #[repr(C)]
        struct Params {
            ReturnValue: bool,
        }

        let mut params = ::core::mem::zeroed::<Params,>();

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance =
                super::StaticFindObject(0, -1, crate::w!("Function '/Script/Engine.PrimalActor.IsFemale'"), false,);
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
    pub unsafe fn CurrentlyHasRider(&self,) -> bool {
        #[repr(C)]
        struct Params {
            return_value: bool,
        }

        let mut params = ::core::mem::zeroed::<Params,>();

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/Engine.PrimalActor.CurrentlyHasRider'"),
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
}
