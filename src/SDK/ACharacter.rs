#[struct_macro::inherit(super::APrimalPawn::APrimalPawn, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct ACharacter {
    #[offset(0x0540)]
    pub Mesh:              *mut super::USkeletalMeshComponent::USkeletalMeshComponent,
    pub CharacterMovement: *mut super::UCharacterMovementComponent::UCharacterMovementComponent,

    #[offset(0x0550)]
    pub CapsuleComponent: *mut super::UCapsuleComponent::UCapsuleComponent,
}

impl ACharacter {
    #[inline]
    pub unsafe fn ClientCheatFly(&self,) {
        pub static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(0, -1, crate::w!("Function 'Engine.Character.ClientCheatFly'"), false,);
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            0 as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }

    #[inline]
    pub unsafe fn ClientCheatWalk(&self,) {
        pub static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(0, -1, crate::w!("Function 'Engine.Character.ClientCheatWalk'"), false,);
        }

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            0 as usize,
            super::Globals::ProcessEventFnAddr,
        );
    }
}
