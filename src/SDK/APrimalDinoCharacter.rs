use super::{
    APrimalCharacter::APrimalCharacter, AShooterCharacter::AShooterCharacter, Structs::TWeakObjectPtr::TWeakObjectPtr,
};

#[struct_macro::bitfields]
#[struct_macro::inherit(super::APrimalCharacter::APrimalCharacter, Flags)]
#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct APrimalDinoCharacter {
    #[offset(0x21C8)]
    pub Rider: TWeakObjectPtr<AShooterCharacter,>,

    #[offset(0x28E0)]
    #[bits(8, bIsFlying)]
    _0:            bool,
    _1:            bool,
    _2:            bool,
    _3:            bool,
    _4:            bool,
    _5:            bool,
    _6:            bool,
    pub bIsFlying: bool,

    #[offset(0x28E3)]
    #[bits(3, bIsFemale)]
    _0:            bool,
    _1:            bool,
    pub bIsFemale: bool,

    #[offset(0x28F0)]
    #[bits(2, bFlyerDinoAllowBackwardsFlight)]
    pub bFlyerDinoAllowBackwardsFlight: bool,
    pub bFlyerDinoAllowStrafing:        bool,
}

impl APrimalDinoCharacter {
    #[inline]
    pub unsafe fn StaticClass() -> *mut super::UClass::UClass {
        static mut Instance: *mut super::UClass::UClass = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                "Class '/Script/ShooterGame.PrimalDinoCharacter'"
                    .encode_utf16()
                    .chain(::core::iter::once(0,),)
                    .collect::<Vec<u16,>>()
                    .as_ptr(),
                false,
            );

            #[cfg(debug_assertions)]
            dbg!(Instance);
        }

        Instance
    }

    #[inline]
    pub unsafe fn is_tamed(&self,) -> bool {
        self.TargetingTeam >= 0xC350
    }

    #[inline]
    pub unsafe fn BPIsTamed(&self,) -> bool {
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
                crate::w!("Function 'ShooterGame.PrimalDinoCharacter.BPIsTamed'"),
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
    pub unsafe fn ServerClearRider(&self, OverrideUnboardDirection: i32,) {
        #[repr(C)]
        struct Params {
            OverrideUnboardDirection: i32,
        }

        let mut params = Params { OverrideUnboardDirection, };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'ShooterGame.PrimalDinoCharacter.ServerClearRider'"),
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
    pub unsafe fn CanMount(&self, aCharacter: *mut APrimalCharacter,) -> bool {
        #[repr(C)]
        struct Params {
            aCharacter:  *mut APrimalCharacter,
            ReturnValue: bool,
            Pad_9:       [u8; 0x7],
        }

        let mut params = Params { aCharacter, ReturnValue: false, Pad_9: [0u8; 0x7], };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'ShooterGame.PrimalDinoCharacter.CanMount'"),
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
}
