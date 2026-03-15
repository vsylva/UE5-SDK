use super::{APrimalDinoCharacter::APrimalDinoCharacter, Structs::FVector::FVector};

#[struct_macro::bitfields]
#[struct_macro::inherit(super::ACharacter::ACharacter, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct APrimalCharacter {
    #[offset(0x0FC8)]
    pub TribeName:       super::Structs::FString::FString,
    pub DescriptiveName: super::Structs::FString::FString,

    #[offset(0x12C8)]
    pub MyCharacterStatusComponent: *mut super::UPrimalCharacterStatusComponent::UPrimalCharacterStatusComponent,
    pub MyInventoryComponent:       *mut super::UPrimalInventoryComponent::UPrimalInventoryComponent,

    #[offset(0x1338)]
    pub CorpseDestructionTime: f64,

    #[offset(0x1530)]
    pub ReplicatedCurrentHealth: f32,
    pub ReplicatedMaxHealth:     f32,
    pub ReplicatedCurrentTorpor: f32,

    #[offset(0x1690)]
    #[bits(2, bIsSleeping)]
    pub bIsSleeping: bool,
    pub bWantsToRun: bool,

    #[offset(0x1693)]
    #[bits(6, bIsDead)]
    _0:          bool,
    _1:          bool,
    _2:          bool,
    _3:          bool,
    _4:          bool,
    pub bIsDead: bool,
}

impl APrimalCharacter {
    #[inline]
    pub unsafe fn IsPrimalCharFriendly(&self, primalChar: *mut Self,) -> bool {
        #[repr(C)]
        struct Params {
            primalChar:   *mut APrimalCharacter,
            return_value: bool,
            Pad_C:        [u8; 0x7],
        }

        let mut params = ::core::mem::zeroed::<Params,>();

        params.primalChar = primalChar;

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function '/Script/ShooterGame.PrimalCharacter.IsPrimalCharFriendly'"),
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
    pub unsafe fn IsWildSlow(&self,) -> bool {
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
                crate::w!("Function '/Script/ShooterGame.PrimalCharacter.IsWildSlow'"),
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
    pub unsafe fn BPSuicide(&self,) {
        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance =
                super::StaticFindObject(0, -1, crate::w!("Function 'ShooterGame.PrimalCharacter.BPSuicide'"), false,);
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            0 as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }

    #[inline]
    pub unsafe fn CanMountOnMe(&self, DinoCharacter: *mut APrimalDinoCharacter,) -> bool {
        #[repr(C)]
        struct Params {
            DinoCharacter: *mut APrimalDinoCharacter,
            return_value:  bool,
            Pad_C:         [u8; 0x7],
        }

        let mut params = Params { DinoCharacter, return_value: false, Pad_C: [0u8; 0x7], };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'ShooterGame.PrimalCharacter.CanMountOnMe'"),
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
    pub unsafe fn GetVisualVelocity(&self,) -> FVector {
        #[repr(C)]
        struct Params {
            ReturnValue: FVector,
        }

        let mut params = ::core::mem::zeroed::<Params,>();

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'ShooterGame.PrimalCharacter.GetVisualVelocity'"),
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
