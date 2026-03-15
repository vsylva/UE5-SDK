use super::APrimalDinoCharacter::APrimalDinoCharacter;

#[struct_macro::bitfields]
#[struct_macro::inherit(super::APrimalCharacter::APrimalCharacter, Flags)]
#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct AShooterCharacter {
    #[offset(0x1CE0)]
    pub PlayerName: super::Structs::FString::FString,

    #[offset(0x1EF8)]
    pub RidingDino: super::Structs::TWeakObjectPtr::TWeakObjectPtr<APrimalDinoCharacter,>,

    #[offset(0x2038)]
    pub CurrentWeapon: *mut super::AShooterWeapon::AShooterWeapon,

    #[offset(0x2310)]
    pub ReplicatedWeight: f32,

    #[offset(0x2450)]
    #[bits(5, bIsRiding)]
    _0:            bool,
    _1:            bool,
    _2:            bool,
    _3:            bool,
    pub bIsRiding: bool,

    #[offset(0x2452)]
    #[bits(6, bIsConnected)]
    _0:               bool,
    _1:               bool,
    _2:               bool,
    _3:               bool,
    _4:               bool,
    pub bIsConnected: bool,
}

impl AShooterCharacter {
    #[inline]
    pub unsafe fn StaticClass() -> *mut super::UClass::UClass {
        pub static mut Instance: *mut super::UClass::UClass = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                "Instance '/Script/ShooterGame.ShooterCharacter'"
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
    pub unsafe fn GetRidingDino(&self,) -> *mut super::APrimalDinoCharacter::APrimalDinoCharacter {
        static mut Instance: *mut super::UFcuntion::UFunction = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                crate::w!("Function 'ShooterGame.ShooterCharacter.GetRidingDino'"),
                false,
            );
        }

        #[repr(C)]
        struct Param {
            ReturnValue: *mut super::APrimalDinoCharacter::APrimalDinoCharacter,
        }

        let mut params = Param { ..::core::mem::zeroed() };

        super::ProcessEvent(
            self as *const _ as usize,
            Instance as usize,
            &mut params as *mut _ as usize,
            super::Globals::ProcessEventFnAddr,
        );

        params.ReturnValue
    }
}
