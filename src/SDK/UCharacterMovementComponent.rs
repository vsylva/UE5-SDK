use super::Structs::FVector::FVector;

#[struct_macro::inherit(super::UPawnMovementComponent::UPawnMovementComponent, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct UCharacterMovementComponent {
    #[offset(0x01A0)]
    pub CharacterOwner: *mut super::ACharacter::ACharacter,

    #[offset(0x0241)]
    pub MovementMode: EMovementMode,

    #[offset(0x0294)]
    pub MaxFlySpeed: f32,
}

#[derive(Debug, Copy, Clone,)]
#[repr(u8)]
pub enum EMovementMode {
    MOVE_None = 0,
    MOVE_Walking = 1,
    MOVE_NavWalking = 2,
    MOVE_Falling = 3,
    MOVE_Swimming = 4,
    MOVE_Flying = 5,
    MOVE_Custom = 6,
    MOVE_MAX = 7,
}

impl UCharacterMovementComponent {
    #[inline]
    pub unsafe fn SetMovementMode(&self, NewMovementMode: EMovementMode, newCustomMode: u8,) {
        #[repr(C)]
        struct Params {
            NewMovementMode: EMovementMode,
            newCustomMode:   u8,
        }

        let mut params = Params { NewMovementMode, newCustomMode, };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'Engine.CharacterMovementComponent.SetMovementMode'"),
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
    pub unsafe fn GetCurrentAcceleration(&self,) -> FVector {
        #[repr(C)]
        struct Params {
            ReturnValue: FVector,
        }

        let mut params = Params { ReturnValue: FVector::zero(), };

        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'Engine.CharacterMovementComponent.GetCurrentAcceleration'"),
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
