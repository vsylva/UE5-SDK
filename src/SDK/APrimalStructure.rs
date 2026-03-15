use super::Structs::FString::FString;

#[struct_macro::inherit(super::APrimalTargetableActor::APrimalTargetableActor, Flags)]
#[derive(Debug, Copy, Clone,)]
#[repr(C)]
pub struct APrimalStructure {
    #[offset(0x08B8)]
    pub OwnerName: FString,
}

impl APrimalStructure {
    #[inline]
    pub unsafe fn StaticClass() -> *mut super::UClass::UClass {
        pub static mut Instance: *mut super::UClass::UClass = ::core::ptr::null_mut();

        if ::core::intrinsics::unlikely(Instance.is_null(),)
        {
            Instance = super::StaticFindObject(
                0,
                -1,
                "Instance '/Script/ShooterGame.PrimalStructure'"
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
}
